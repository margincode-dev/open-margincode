# scholion.openevaluations.si

The project website: a static page, no build step. Everything is in this folder.

| File | What |
|---|---|
| `index.html` | The page (inline CSS and JS; fonts from Google Fonts). |
| `llms.txt` | The same story in Markdown for agents ([llmstxt.org](https://llmstxt.org)); the page's "agent view" shows it. |
| `guides/` | Plain how-to pages aimed at search (handoffs between agents, agent memory). Use real commands and recorded output only. |
| `demo.html` | `/demo`: plays back `demo-replay.json`, a recorded run of three agents on a toy repo. |
| `demo-replay.json` | Written by `scripts/demo-replay.py` from a real daemon; do not edit by hand. |
| `404.html` | Served by Cloudflare Pages for unknown paths. |
| `_headers` | Cloudflare Pages response headers (CSP, `text/markdown` for `llms.txt`). |
| `og.png` | 1200×630 link preview, rendered from `scripts/og.html` (see "Link preview"). |
| `img/demo-replay.png` | Screenshot of `/demo` under the hero: `docs/img/demo-replay.png` (which the README uses) with its top 10 px cropped off (a sliver of the page header). If you replace it, keep the `width`/`height` attributes of its `<img>` in `index.html` equal to the file's size. |
| `favicon.svg`, `robots.txt`, `sitemap.xml` | The usual. |

Numbers on the page come from these result directories, and only these:

| Section | Source |
|---|---|
| Retrieval (E1), 4000-token rows | `bench/retrieval/results/2026-10-28-packer-rrf/report.md` |
| Retrieval (E1), 800-token caption | `bench/retrieval/results/2026-10-29-budget-800/report.md` (on PR #21's branch until it merges) |
| End-to-end agents (Haiku) | `bench/results/2026-10-27-haiku/` (the `04-final-shipped-*` runs) |
| Knowledge relay (E4) | `bench/results/2026-10-29-relay-lean/report.md` |

When a newer run replaces one, change this table, `index.html` and `llms.txt` in the same commit.
A hero stat must be something av does that a plain file of notes does not.

## The replay demo

`/demo` shows what the daemon actually returned. To re-record it after a change to packs or the
API, start a daemon on an empty database and run `scripts/demo-replay.py` (see its docstring). The
script stops if a caption's claim (say, "the pack leads with the dead end") no longer holds in the
output; fix the caption or the story, not the check.

## Preview

```bash
python3 -m http.server -d site 8000   # then open http://localhost:8000
```

## Link preview

`og.png` is a screenshot of `scripts/og.html`, which lives outside `site/` so it is not served.
Edit the template, then render it with headless Chrome (the virtual-time budget gives Google Fonts
time to load; without it the serif falls back to Georgia) and look at the result:

```bash
chrome --headless=new --hide-scrollbars --force-device-scale-factor=1 --window-size=1200,630 \
  --virtual-time-budget=8000 --screenshot=site/og.png "file://$PWD/scripts/og.html"
```

When the guides or other pages change, update their `dateModified` (JSON-LD and the visible date
line) and the page's `<lastmod>` in `sitemap.xml`.

## Deploy to Cloudflare

`wrangler.jsonc` at the repo root serves `site/` as static assets (Worker name `scholion-deploy`).

**Git integration.** Cloudflare dashboard → Workers & Pages → Create → Import a repository →
this repo. Leave the build command empty and keep the deploy command `npx wrangler deploy`.
Cloudflare builds the production branch (`main`), so merge the site there first.

**GitHub Actions.** `.github/workflows/site.yml` runs `wrangler deploy` on pushes to `main` that
touch `site/`, if the secrets `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` exist.

**Custom domain.** Worker → Settings → Domains & Routes → add `scholion.openevaluations.si` (the zone must
be on Cloudflare; otherwise move the `.si` nameservers there). `wrangler.jsonc` also declares it as
a `custom_domain` route, so `wrangler deploy` attaches it.

## Cutover (root to subdomain)

Decision of 2026-11-04: the Scholion site moves from `openevaluations.si` to
`scholion.openevaluations.si`; the root then serves only the openevaluations tracker and handoff
benchmark (repo `vinhqbui/openevaluations`, Worker `openevaluations`). Merging this change does
not touch the root. Three owner steps, in order:

1. Add the Cloudflare custom domain `scholion.openevaluations.si` to the Worker `scholion-deploy`
   (Workers & Pages -> scholion-deploy -> Settings -> Domains & Routes), or just deploy this
   branch: `wrangler.jsonc` declares the same custom domain. Check that
   `https://scholion.openevaluations.si/demo` loads.
2. Deploy the openevaluations site to the root: merge `root-site` in `vinhqbui/openevaluations`
   (deploys the Worker `openevaluations`; its attach of the custom domain `openevaluations.si` to
   the root has to wait for step 3).
3. Remove the root domain `openevaluations.si` from `scholion-deploy` (Domains & Routes). Cloudflare
   allows one Worker per custom domain, so only now can `openevaluations` take the root. Its
   `wrangler.jsonc` does **not** declare the root yet: add
   `"routes": [{ "pattern": "openevaluations.si", "custom_domain": true }]` there, then rerun its
   deploy. Expect a short gap between removing and attaching; do the two back to back.

State on 2026-11-04: step 3's removal ran before the openevaluations Worker could attach, and the
root went dark. As a stopgap the zone has a Single Redirect rule (its description starts
`temp: openevaluations.si root+www`) sending `openevaluations.si` and `www` to the same path on
`scholion.openevaluations.si` with a 302, plus two placeholder proxied `A 192.0.2.1` records for
the root and `www`. Before the openevaluations Worker takes the root, delete that rule and both
placeholder records: a custom domain cannot attach over an existing record, and the rule would
hide the new site.

Redirect for a while: add a rule on the `openevaluations.si` zone (Rules -> Redirect Rules, or a
small Worker route) that sends the old Scholion paths, `/demo`, `/demo-replay.json`, `/guides/*`
and `/llms.txt` of the old site, to the same path on `scholion.openevaluations.si` with a 301.
Do not redirect `/`, `/tracker/`, `/handoff/` and the other openevaluations pages. Note that
openevaluations has its own `/llms.txt` and `/scholion/`; redirect `/llms.txt` only if you want
the old one gone, and drop all of these rules after a few months.
