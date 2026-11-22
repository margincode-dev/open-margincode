// node harness.mjs <plugin.js> <script.json>: loads an OpenCode plugin with a stand-in `ctx`,
// plays the script's steps and prints one JSON document of what the plugin did. The shapes are
// OpenCode 2.3.2's plugin API, as seen on 2026-11-22.
//
// script: { sessions: {id: {directory, parentID?}}, entries?: {id: [{key, value}]},
//           throw?: ["session.get" | "entry.list" | "entry.put" | "synthetic"],
//           steps: [{hook, event, wait?: false} | {sleep: ms} | {untilFile: path} | {emit: event}] }
import fs from "node:fs";
import { pathToFileURL } from "node:url";

const [pluginPath, scriptPath] = process.argv.slice(2);
const script = JSON.parse(fs.readFileSync(scriptPath, "utf8"));
const entries = script.entries ?? {};
const throws = new Set(script.throw ?? []);
const hooks = {};
const puts = [];
const synthetic = [];
const steps = [];
const listeners = new Set();

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
const fail = (name) => {
  if (throws.has(name)) throw new Error(`stand-in ${name} fails`);
};

function subscribe(options) {
  return {
    [Symbol.asyncIterator]() {
      const queue = [];
      let wake = () => {};
      const listener = (event) => {
        if (!options?.types || options.types.includes(event.type)) {
          queue.push(event);
          wake();
        }
      };
      listeners.add(listener);
      return {
        async next() {
          while (queue.length === 0) await new Promise((resolve) => (wake = resolve));
          return { value: queue.shift(), done: false };
        },
        async return() {
          listeners.delete(listener);
          return { value: undefined, done: true };
        },
      };
    },
  };
}

const register = (prefix) => async (name, callback) => {
  hooks[prefix + name] = callback;
  return { dispose: async () => {} };
};

const ctx = {
  location: { directory: process.cwd() },
  event: { subscribe },
  tool: { hook: register("tool.") },
  session: {
    hook: register(""),
    async get({ sessionID }) {
      fail("session.get");
      const s = script.sessions[sessionID];
      if (!s) throw new Error(`no session ${sessionID}`);
      return { id: sessionID, location: { directory: s.directory }, ...(s.parentID ? { parentID: s.parentID } : {}) };
    },
    async synthetic({ sessionID, text }) {
      fail("synthetic");
      synthetic.push({ sessionID, text });
      return { id: `syn-${synthetic.length}`, type: "synthetic" };
    },
    instructions: {
      entry: {
        async list({ sessionID }) {
          fail("entry.list");
          return entries[sessionID] ?? [];
        },
        async put({ sessionID, key, value }) {
          fail("entry.put");
          puts.push({ sessionID, key, value });
          entries[sessionID] = [...(entries[sessionID] ?? []).filter((e) => e.key !== key), { key, value }];
          return {};
        },
      },
    },
  },
};

const plugin = (await import(pathToFileURL(pluginPath).href)).default;
const cleanup = await plugin.setup(ctx);

const pending = [];
for (const step of script.steps) {
  if (step.sleep !== undefined) {
    await sleep(step.sleep);
  } else if (step.untilFile !== undefined) {
    for (let i = 0; i < 6000 && !fs.existsSync(step.untilFile); i++) await sleep(20);
  } else if (step.emit !== undefined) {
    for (const listener of listeners) listener(step.emit);
  } else {
    const record = { hook: step.hook };
    steps.push(record);
    const event = structuredClone(step.event);
    const run = (async () => {
      try {
        await hooks[step.hook](event);
        record.returned = true;
      } catch (e) {
        record.error = String(e);
      }
      if (event.result) record.content = event.result.content;
    })();
    if (step.wait === false) pending.push(run);
    else await run;
  }
}
await Promise.all(pending);
if (typeof cleanup === "function") await cleanup();
console.log(JSON.stringify({ hooks: Object.keys(hooks).sort(), puts, synthetic, steps }));
process.exit(0);
