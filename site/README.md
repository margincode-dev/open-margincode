# openevaluations.si

The project website: a static page, no build step. Everything is in this folder.

| File | What |
|---|---|
| `index.html` | The page (inline CSS and JS; fonts from Google Fonts). |
| `llms.txt` | The same story in Markdown for agents ([llmstxt.org](https://llmstxt.org)); the page's "agent view" shows it. |
| `404.html` | Served by Cloudflare Pages for unknown paths. |
| `_headers` | Cloudflare Pages response headers (CSP, `text/markdown` for `llms.txt`). |
| `og.png` | 1200×630 link preview. |
| `favicon.svg`, `robots.txt`, `sitemap.xml` | The usual. |

Numbers on the page come from `bench/` (E1 retrieval at a 4000-token budget, and the 2026-10-27
Haiku runs). When the benchmarks change, update `index.html` and `llms.txt` together.

## Preview

```bash
python3 -m http.server -d site 8000   # then open http://localhost:8000
```

## Deploy to Cloudflare

`wrangler.jsonc` at the repo root serves `site/` as static assets (Worker name `scholion-deploy`).

**Git integration.** Cloudflare dashboard → Workers & Pages → Create → Import a repository →
this repo. Leave the build command empty and keep the deploy command `npx wrangler deploy`.
Cloudflare builds the production branch (`main`), so merge the site there first.

**GitHub Actions.** `.github/workflows/site.yml` runs `wrangler deploy` on pushes to `main` that
touch `site/`, if the secrets `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` exist.

**Custom domain.** Worker → Settings → Domains & Routes → add `openevaluations.si` (the zone must
be on Cloudflare; otherwise move the `.si` nameservers there).
