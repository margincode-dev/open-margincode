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

## Deploy to Cloudflare Pages

Either way, the project is named `openevaluations` and serves this folder as is.

**Git integration (no secrets in GitHub).** Cloudflare dashboard → Workers & Pages → Create →
Pages → Connect to Git → this repository. Production branch `main`, framework preset *None*,
build command empty, build output directory `site`.

**GitHub Actions.** `.github/workflows/site.yml` deploys on every push to `main` that touches
`site/`. Create the Pages project once (`npx wrangler pages project create openevaluations
--production-branch main`), then add the repository secrets `CLOUDFLARE_API_TOKEN` (a token with
*Account → Cloudflare Pages → Edit*) and `CLOUDFLARE_ACCOUNT_ID`. Without them the workflow skips
the deploy.

**Custom domain.** Pages project → Custom domains → add `openevaluations.si` (and
`www.openevaluations.si`, redirected to the apex with a Bulk Redirect or a redirect rule). With
the `.si` zone on Cloudflare the DNS records are created for you; otherwise point a `CNAME` at
`openevaluations.pages.dev` at your registrar.
