// av's hooks for OpenCode 2.x, written by `av init --for opencode` (integrations/opencode/README.md).
const AV_BIN = "av";

// Runs inside OpenCode's server, so nothing here may throw into OpenCode or wait without a limit:
// a thrown error fails the prompt, the turn or the tool call (seen with OpenCode 2.3.2 on
// 2026-11-22). Every `av hook` child gets its stdin closed and a time limit, and its failures
// leave the session as it was.
import { spawn } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

const LIMIT_MS = Number(process.env.AV_OPENCODE_HOOK_TIMEOUT_MS) || 60_000;
const EDITS = new Set(["edit", "write", "patch"]);

let warned = false;
function warn(message) {
  if (warned) return;
  warned = true;
  try {
    console.error(`av plugin: ${message} (further problems are not logged)`);
  } catch {}
}

// The `av` entry `av init` wrote beside this plugin's folder, read at each call: the token lives
// in that file only and reaches the child in its environment, never in its arguments.
async function avEnv() {
  const env = { ...process.env, AV_AGENT: process.env.AV_AGENT || "opencode" };
  try {
    const text = await fs.promises.readFile(new URL("../opencode.json", import.meta.url), "utf8");
    const av = JSON.parse(text)?.mcp?.servers?.av;
    const headers = av?.headers ?? {};
    if (typeof av?.url === "string") env.AV_URL = av.url.replace(/\/mcp\/?$/, "");
    if (typeof headers["X-Av-Agent"] === "string") env.AV_AGENT = headers["X-Av-Agent"];
    if (typeof headers.Authorization === "string" && headers.Authorization.startsWith("Bearer "))
      env.AV_TOKEN = headers.Authorization.slice("Bearer ".length);
  } catch {}
  return env;
}

// `av hook <name>` with `payload` on stdin. Resolves to its stdout, or "" when it cannot start,
// fails, runs past `limitMs` or `signal` aborts; never rejects.
async function avHook(name, payload, { env = {}, limitMs = LIMIT_MS, signal } = {}) {
  const fullEnv = { ...(await avEnv()), ...env };
  return new Promise((resolve) => {
    let out = "";
    let err = "";
    let child;
    let done = false;
    const end = (text, problem) => {
      if (done) return;
      done = true;
      clearTimeout(timer);
      signal?.removeEventListener("abort", abort);
      if (problem) warn(problem);
      resolve(text);
    };
    const stop = (problem) => {
      try {
        child?.kill();
      } catch {}
      end("", problem);
    };
    const abort = () => stop();
    const timer = setTimeout(() => stop(`av hook ${name} ran past ${limitMs} ms and was stopped`), limitMs);
    signal?.addEventListener("abort", abort);
    try {
      const bin = path.isAbsolute(AV_BIN) && fs.existsSync(AV_BIN) ? AV_BIN : "av";
      child = spawn(bin, ["hook", name], { env: fullEnv, stdio: ["pipe", "pipe", "pipe"], windowsHide: true });
      child.on("error", (e) => end("", `cannot run av (${e.message}): the av hooks do nothing`));
      child.stdout.setEncoding("utf8");
      child.stderr.setEncoding("utf8");
      child.stdout.on("data", (d) => (out += d));
      child.stderr.on("data", (d) => (err += d));
      child.stdout.on("error", () => {});
      child.stderr.on("error", () => {});
      child.stdin.on("error", () => {});
      child.on("close", (code) =>
        end(code === 0 ? out : "", code === 0 ? err.trim().split("\n")[0] : `av hook ${name} exited with ${code}`),
      );
      child.stdin.end(JSON.stringify(payload));
      if (signal?.aborted) abort();
    } catch (e) {
      stop(`cannot run av (${e}): the av hooks do nothing`);
    }
  });
}

function inTask(dir) {
  for (let d = dir; ; d = path.dirname(d)) {
    if (fs.existsSync(path.join(d, ".av-task"))) return true;
    if (path.dirname(d) === d) return false;
  }
}

// As OpenCode itself turns a tool result into the model's content.
function contentOf(result) {
  if (typeof result.content === "string") return [{ type: "text", text: result.content }];
  if (Array.isArray(result.content) && result.content.length > 0) return [...result.content];
  const output = result.output;
  return [{ type: "text", text: typeof output === "string" ? output : (JSON.stringify(output) ?? String(output)) }];
}

function additionalContext(out) {
  if (!out.trim()) return undefined;
  const text = JSON.parse(out)?.hookSpecificOutput?.additionalContext;
  return typeof text === "string" && text ? text : undefined;
}

export default {
  id: "av",
  async setup(ctx) {
    const seen = new Set();
    const compacted = new Set();
    // sessions the last idle continued, until the user's next prompt (`stop_hook_active`)
    const continued = new Set();
    // the running `av hook stop` of each session; an interrupt aborts it
    const stopping = new Map();
    const events = new AbortController();

    try {
      // A late `synthetic` after an interrupt starts a new execution (seen with 2.3.2), so an
      // interrupt kills the session's `av hook stop` and nothing is sent. Events are server-wide.
      void (async () => {
        try {
          const interrupts = ctx.event.subscribe({ types: ["session.execution.interrupted"], signal: events.signal });
          for await (const event of interrupts) stopping.get(event?.data?.sessionID)?.abort();
        } catch (e) {
          warn(`cannot follow interrupts (${e})`);
        }
      })();

      // `synthetic` does not fire `prompt`, so each call is a real prompt (OpenCode 2.3.2).
      await ctx.session.hook("prompt", async (event) => {
        try {
          const id = event.sessionID;
          continued.delete(id);
          const afterCompaction = compacted.delete(id);
          if (seen.has(id) && !afterCompaction) return;
          seen.add(id);
          const session = await ctx.session.get({ sessionID: id });
          if (session.parentID) return;
          const dir = session.location.directory;
          const payload = { session_id: id, source: afterCompaction ? "compact" : "startup", cwd: dir };
          // no pack to wait for outside a task; the footprint snapshot still needs the call
          if (!inTask(dir)) return void avHook("session-start", payload);
          // the entry outlives this plugin instance: a resumed session or a reload has it already
          if (!afterCompaction) {
            const entries = await ctx.session.instructions.entry.list({ sessionID: id });
            if (entries.some((e) => e.key === "av-pack")) return;
          }
          const pack = await avHook("session-start", payload);
          if (pack.trim()) await ctx.session.instructions.entry.put({ sessionID: id, key: "av-pack", value: pack });
        } catch (e) {
          warn(`prompt: ${e}`);
        }
      });

      await ctx.session.hook("compaction", async (event) => {
        try {
          compacted.add(event.sessionID);
        } catch {}
      });

      await ctx.tool.hook("execute.after", async (event) => {
        try {
          if (event.status !== "completed" || (event.tool !== "read" && !EDITS.has(event.tool))) return;
          // `patch` names its files in the result, as the tool resolved them (OpenCode 2.3.2)
          const file = event.tool === "patch" ? event.result.metadata?.files?.[0]?.file : event.input?.path;
          if (event.tool === "read" && typeof file !== "string") return;
          const dir = (await ctx.session.get({ sessionID: event.sessionID })).location.directory;
          const tool_input = typeof file === "string" ? { file_path: path.resolve(dir, file) } : {};
          const out =
            event.tool === "read"
              ? await avHook("pre-read", { tool_input, hook_event_name: "PreToolUse" })
              : await avHook("post-edit", { cwd: dir, tool_input });
          const text = additionalContext(out);
          if (text) event.result.content = [...contentOf(event.result), { type: "text", text }];
        } catch (e) {
          warn(`${event?.tool}: ${e}`);
        }
      });

      await ctx.session.hook("experimental.idle", async (event) => {
        const id = event?.sessionID;
        const interrupt = new AbortController();
        try {
          if (event.background.length > 0) return;
          stopping.set(id, interrupt);
          const session = await ctx.session.get({ sessionID: id });
          if (session.parentID) return;
          // OpenCode gives this hook no time limit and holds the user's next prompt until it
          // returns, so the wait for work is short unless the server's environment says otherwise.
          const wait = process.env.AV_HOOK_WAIT_SECS || "1";
          const payload = { session_id: id, cwd: session.location.directory, stop_hook_active: continued.has(id) };
          const out = await avHook("stop", payload, {
            env: { AV_HOOK_WAIT_SECS: wait },
            limitMs: (Number(wait) || 0) * 1000 + LIMIT_MS,
            signal: interrupt.signal,
          });
          if (interrupt.signal.aborted) return;
          const decision = out.trim() ? JSON.parse(out) : undefined;
          if (decision?.decision === "block" && typeof decision.reason === "string") {
            continued.add(id);
            await ctx.session.synthetic({ sessionID: id, text: decision.reason });
          } else {
            continued.delete(id);
          }
        } catch (e) {
          warn(`idle: ${e}`);
        } finally {
          if (stopping.get(id) === interrupt) stopping.delete(id);
        }
      });
    } catch (e) {
      warn(`cannot register the av hooks (${e})`);
    }
    return () => events.abort();
  },
};
