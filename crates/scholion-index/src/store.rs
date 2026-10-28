//! Postgres persistence for the code index: symbols, reference graph, parse cache.

use std::collections::{BTreeSet, HashMap};

use async_trait::async_trait;
use scholion_core::{
    Anchor, AvError, ChangeId, NewOutbox, OutboxOp, Result, SemanticKind, SymbolId, TaskId,
};
use sha2::{Digest, Sha256};
use sqlx::postgres::PgPool;
use sqlx::types::Json;
use sqlx::{PgConnection, Row};
use uuid::Uuid;

use crate::extract::{Symbol, SymbolKind, extract};

/// Path segments that are never indexed.
const IGNORED_SEGMENTS: [&str; 4] = ["node_modules", ".venv", "target", ".git"];
/// Neighbor walks are capped at this many hops.
const MAX_HOPS: u8 = 2;
/// Chunk text kept for embedding, counted in Unicode scalars.
pub const MAX_EMBED_CHARS: usize = 2000;

fn ierr(e: sqlx::Error) -> AvError {
    AvError::Index(e.to_string())
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IndexStats {
    /// Files parsed by tree-sitter.
    pub parsed: usize,
    /// Files whose symbols came from the content-hash parse cache.
    pub cached: usize,
    /// Files already indexed with identical content.
    pub unchanged: usize,
    /// Files dropped because their path is inside an ignored directory.
    pub skipped: usize,
    /// Files whose extraction returned an error (the batch still continues).
    pub failed: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexedSymbol {
    pub id: SymbolId,
    pub symbol: Symbol,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeChunk {
    pub symbol_id: SymbolId,
    pub repo: String,
    pub scope: String,
    pub path: String,
    pub task_id: Option<TaskId>,
    pub change_id: Option<ChangeId>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnchorResolution {
    /// The anchored symbol exists.
    Symbol(SymbolId),
    /// The file exists but the anchor's symbol is absent (renamed/removed) or the anchor named no symbol.
    PathOnly(String),
    /// The anchored file is gone.
    Stale,
}

#[async_trait]
pub trait CodeIndex: Send + Sync {
    /// Indexes `files` (path, bytes) into `(repo, scope)`; unchanged content is not re-parsed.
    async fn index_paths(
        &self,
        repo: &str,
        scope: &str,
        files: Vec<(String, Vec<u8>)>,
    ) -> Result<IndexStats> {
        self.index_paths_at(repo, scope, None, true, files).await
    }
    /// As `index_paths`, recording `change`. Upserts are enqueued only when `embed` is set;
    /// symbol deletes are always enqueued.
    async fn index_paths_at(
        &self,
        repo: &str,
        scope: &str,
        change: Option<&ChangeId>,
        embed: bool,
        files: Vec<(String, Vec<u8>)>,
    ) -> Result<IndexStats>;
    /// The change `(repo, scope)` fully reflects, if recorded. Any `index_paths_at` or
    /// `remove_paths` on the scope clears it; whoever completes an index of one change records it.
    async fn scope_change(&self, repo: &str, scope: &str) -> Result<Option<ChangeId>>;
    async fn set_scope_change(&self, repo: &str, scope: &str, change: &ChangeId) -> Result<()>;
    /// Copies the index of `from` into the empty scope `to` (new symbol ids, edges remapped,
    /// chunks and file hashes copied; nothing enqueued for embedding) when `from` is recorded at
    /// `change`, and records `to` at `change`. `Ok(false)`: `from` is not at `change` and nothing
    /// was written. `to` already holding an index is `Invalid`.
    async fn clone_scope(
        &self,
        repo: &str,
        from: &str,
        to: &str,
        change: &ChangeId,
    ) -> Result<bool>;
    async fn list_code_chunks(&self, repo: Option<&str>) -> Result<Vec<CodeChunk>>;
    async fn chunk_exists(&self, symbol_id: SymbolId) -> Result<bool>;
    async fn remove_paths(&self, repo: &str, scope: &str, paths: &[String]) -> Result<()>;
    async fn symbols_in(&self, repo: &str, scope: &str, path: &str) -> Result<Vec<IndexedSymbol>>;
    async fn get_symbols(&self, ids: &[SymbolId]) -> Result<Vec<IndexedSymbol>>;
    /// Case-insensitive literal substring match on qname or path; exact qname matches first.
    async fn search_symbols(
        &self,
        repo: &str,
        scope: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<IndexedSymbol>>;
    async fn resolve(&self, repo: &str, scope: &str, anchor: &Anchor) -> Result<AnchorResolution>;
    /// Code chunks of a scope ranked by full-text match against `terms` (lowercase words), each
    /// term weighted by its rarity in the scope (BM25's IDF), best first. Works without the
    /// semantic layer. The default finds nothing.
    async fn search_code_lexical(
        &self,
        _repo: &str,
        _scope: &str,
        _terms: &[String],
        _limit: usize,
    ) -> Result<Vec<(SymbolId, f32)>> {
        Ok(vec![])
    }
    /// Symbols within `hops` (1..=2) of `seeds`, following references in both directions,
    /// each with its minimum hop distance. Seeds are excluded.
    async fn neighbors(
        &self,
        repo: &str,
        scope: &str,
        seeds: &[SymbolId],
        hops: u8,
    ) -> Result<Vec<(SymbolId, u8)>>;
    /// Per symbol, how many symbols reference it; symbols nothing references are absent.
    async fn in_degree(&self, ids: &[SymbolId]) -> Result<HashMap<SymbolId, u32>>;
    /// The (repo, scope) a symbol belongs to, if it exists.
    async fn symbol_scope(&self, id: SymbolId) -> Result<Option<(String, String)>>;
    /// Removes a whole scope (symbols, edges, chunks, file hashes, its recorded change) and
    /// enqueues a vector delete per code chunk. The parse cache stays (`prune_parse_cache`).
    async fn drop_scope(&self, repo: &str, scope: &str) -> Result<()>;
    /// Deletes parse-cache entries whose content no scope holds any more; returns how many.
    async fn prune_parse_cache(&self) -> Result<u64>;
}

#[derive(Clone)]
pub struct PgIndex {
    pool: PgPool,
}

impl PgIndex {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

const SYMBOL_COLS: &str = "id, path, qname, kind, signature, start_line, end_line, refs";

fn symbol_from_row(r: &sqlx::postgres::PgRow) -> Result<IndexedSymbol> {
    Ok(IndexedSymbol {
        id: SymbolId(r.try_get("id").map_err(ierr)?),
        symbol: Symbol {
            path: r.try_get("path").map_err(ierr)?,
            qname: r.try_get("qname").map_err(ierr)?,
            kind: kind_from_str(&r.try_get::<String, _>("kind").map_err(ierr)?)?,
            signature: r.try_get("signature").map_err(ierr)?,
            start_line: r.try_get::<i32, _>("start_line").map_err(ierr)? as u32,
            end_line: r.try_get::<i32, _>("end_line").map_err(ierr)? as u32,
            refs: r.try_get("refs").map_err(ierr)?,
        },
    })
}

fn is_ignored(path: &str) -> bool {
    path.split('/').any(|seg| IGNORED_SEGMENTS.contains(&seg))
}

fn cache_key(path: &str, bytes: &[u8]) -> (String, String) {
    let hash = format!("{:x}", Sha256::digest(bytes));
    let ext = path.rsplit_once('.').map(|(_, e)| e).unwrap_or("");
    (hash.clone(), format!("{hash}:{ext}"))
}

fn kind_from_str(s: &str) -> Result<SymbolKind> {
    match s {
        "function" => Ok(SymbolKind::Function),
        "method" => Ok(SymbolKind::Method),
        "type" => Ok(SymbolKind::Type),
        "constant" => Ok(SymbolKind::Constant),
        other => Err(AvError::Index(format!("unknown symbol kind {other}"))),
    }
}

/// Recomputes outgoing edges of `ids`. An identifier in `refs` links to every other symbol in the
/// same scope whose last qname segment equals it; k candidates each get weight 1/k.
async fn rebuild_edges(conn: &mut PgConnection, ids: &[i64]) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }
    sqlx::query("DELETE FROM symbol_refs WHERE src = ANY($1)")
        .bind(ids)
        .execute(&mut *conn)
        .await
        .map_err(ierr)?;
    sqlx::query(
        "INSERT INTO symbol_refs (src, dst, weight)
         SELECT s.id, d.id, (1.0 / cnt.k)::real
         FROM symbols s
         CROSS JOIN LATERAL unnest(s.refs) AS r(name)
         JOIN symbols d ON d.repo = s.repo AND d.scope = s.scope AND d.name = r.name AND d.id <> s.id
         JOIN LATERAL (SELECT count(*)::real AS k FROM symbols d2
                       WHERE d2.repo = s.repo AND d2.scope = s.scope AND d2.name = r.name AND d2.id <> s.id) cnt ON true
         WHERE s.id = ANY($1)
         ON CONFLICT (src, dst) DO UPDATE SET weight = GREATEST(symbol_refs.weight, EXCLUDED.weight)",
    )
    .bind(ids)
    .execute(&mut *conn)
    .await
    .map_err(ierr)?;
    Ok(())
}

async fn dependents_of(
    conn: &mut PgConnection,
    repo: &str,
    scope: &str,
    names: &BTreeSet<String>,
) -> Result<Vec<i64>> {
    if names.is_empty() {
        return Ok(vec![]);
    }
    let names: Vec<&String> = names.iter().collect();
    let rows = sqlx::query("SELECT id FROM symbols WHERE repo = $1 AND scope = $2 AND refs && $3")
        .bind(repo)
        .bind(scope)
        .bind(&names)
        .fetch_all(&mut *conn)
        .await
        .map_err(ierr)?;
    rows.iter().map(|r| r.try_get("id").map_err(ierr)).collect()
}

/// Keeps at most `MAX_EMBED_CHARS` scalars. A truncated value ends with `…`.
fn truncate_embed(text: String) -> String {
    if text.chars().count() <= MAX_EMBED_CHARS {
        return text;
    }
    let mut out: String = text.chars().take(MAX_EMBED_CHARS - 1).collect();
    out.push('\u{2026}');
    out
}

/// 1-based inclusive line slice of `bytes`. Missing lines are omitted.
fn line_slice(bytes: &[u8], start_line: u32, end_line: u32) -> String {
    if start_line == 0 || end_line < start_line {
        return String::new();
    }
    let Ok(text) = std::str::from_utf8(bytes) else {
        return String::new();
    };
    let mut out = String::new();
    for (i, line) in text.split_inclusive('\n').enumerate() {
        let n = i as u32 + 1;
        if n < start_line {
            continue;
        }
        if n > end_line {
            break;
        }
        out.push_str(line);
    }
    out
}

fn chunk_text(path: &str, symbol: &Symbol, bytes: &[u8]) -> String {
    let body = line_slice(bytes, symbol.start_line, symbol.end_line);
    truncate_embed(format!(
        "{path}\n{}\n{}\n{body}",
        symbol.qname, symbol.signature
    ))
}

async fn task_id_for_scope(
    conn: &mut PgConnection,
    repo: &str,
    scope: &str,
) -> Result<Option<TaskId>> {
    if scope == "main" {
        return Ok(None);
    }
    let id: Option<Uuid> = sqlx::query_scalar("SELECT id FROM tasks WHERE repo = $1 AND slug = $2")
        .bind(repo)
        .bind(scope)
        .fetch_optional(&mut *conn)
        .await
        .map_err(ierr)?;
    Ok(id.map(TaskId))
}

/// Any write to a scope makes its recorded change unknown until a complete index records it again.
async fn forget_scope_change(conn: &mut PgConnection, repo: &str, scope: &str) -> Result<()> {
    sqlx::query("DELETE FROM index_scopes WHERE repo = $1 AND scope = $2")
        .bind(repo)
        .bind(scope)
        .execute(&mut *conn)
        .await
        .map_err(ierr)?;
    Ok(())
}

struct ChunkCtx<'a> {
    repo: &'a str,
    task_id: Option<TaskId>,
    change: Option<&'a ChangeId>,
    path: &'a str,
}

fn symbol_outbox(
    ctx: &ChunkCtx<'_>,
    op: OutboxOp,
    symbol_id: i64,
    text: Option<String>,
) -> NewOutbox {
    NewOutbox {
        kind: SemanticKind::CodeChunk,
        op,
        entity_id: symbol_id.to_string(),
        repo: ctx.repo.to_owned(),
        task_id: ctx.task_id,
        record_id: None,
        symbol_id: Some(SymbolId(symbol_id)),
        convention_id: None,
        path: Some(ctx.path.to_owned()),
        change_id: ctx.change.cloned(),
        superseded: false,
        text,
    }
}

async fn enqueue_symbol(
    conn: &mut PgConnection,
    ctx: &ChunkCtx<'_>,
    op: OutboxOp,
    symbol_id: i64,
    text: Option<String>,
) -> Result<()> {
    scholion_store::PgStore::enqueue(&mut *conn, &symbol_outbox(ctx, op, symbol_id, text))
        .await
        .map_err(|e| AvError::Index(e.to_string()))
}

#[async_trait]
impl CodeIndex for PgIndex {
    async fn index_paths_at(
        &self,
        repo: &str,
        scope: &str,
        change: Option<&ChangeId>,
        embed: bool,
        files: Vec<(String, Vec<u8>)>,
    ) -> Result<IndexStats> {
        // A fixed number of statements per call, however many files and symbols: starting a task
        // indexes the whole repo into its scope, so per-symbol round trips dominated.
        let mut stats = IndexStats::default();
        let mut tx = self.pool.begin().await.map_err(ierr)?;
        forget_scope_change(&mut tx, repo, scope).await?;
        let task_id = task_id_for_scope(&mut tx, repo, scope).await?;

        // input order is kept (it decides symbol ids); a repeated path keeps its last content
        let mut order: Vec<(String, Vec<u8>)> = Vec::with_capacity(files.len());
        let mut pos: HashMap<String, usize> = HashMap::new();
        for (path, bytes) in files {
            if is_ignored(&path) {
                stats.skipped += 1;
            } else if let Some(&i) = pos.get(&path) {
                order[i].1 = bytes;
            } else {
                pos.insert(path.clone(), order.len());
                order.push((path, bytes));
            }
        }
        let all_paths: Vec<&str> = order.iter().map(|(p, _)| p.as_str()).collect();
        let existing: HashMap<String, String> = sqlx::query_as(
            "SELECT path, content_hash FROM index_files
             WHERE repo = $1 AND scope = $2 AND path = ANY($3)",
        )
        .bind(repo)
        .bind(scope)
        .bind(&all_paths)
        .fetch_all(&mut *tx)
        .await
        .map_err(ierr)?
        .into_iter()
        .collect();
        let mut changed = vec![];
        for (path, bytes) in order {
            let (hash, key) = cache_key(&path, &bytes);
            if existing.get(&path) == Some(&hash) {
                stats.unchanged += 1;
            } else {
                changed.push((path, bytes, hash, key));
            }
        }
        let keys: Vec<&str> = changed.iter().map(|(_, _, _, k)| k.as_str()).collect();
        let mut cache: HashMap<String, Vec<Symbol>> =
            sqlx::query_as::<_, (String, Json<Vec<Symbol>>)>(
                "SELECT cache_key, symbols FROM parse_cache WHERE cache_key = ANY($1)",
            )
            .bind(&keys)
            .fetch_all(&mut *tx)
            .await
            .map_err(ierr)?
            .into_iter()
            .map(|(k, Json(syms))| (k, syms))
            .collect();

        let mut new_cache: Vec<(String, serde_json::Value)> = vec![];
        let mut parsed: Vec<(String, Vec<u8>, String, Vec<Symbol>)> = vec![];
        for (path, bytes, hash, key) in changed {
            let mut symbols = match cache.get(&key) {
                Some(syms) => {
                    stats.cached += 1;
                    syms.clone()
                }
                None => match extract(&path, &bytes) {
                    Ok(syms) => {
                        stats.parsed += 1;
                        new_cache.push((
                            key.clone(),
                            serde_json::to_value(&syms)
                                .map_err(|e| AvError::Index(e.to_string()))?,
                        ));
                        cache.insert(key, syms.clone());
                        syms
                    }
                    Err(_) => {
                        stats.failed += 1; // the file keeps its previous symbols
                        continue;
                    }
                },
            };
            for s in &mut symbols {
                s.path = path.clone();
            }
            parsed.push((path, bytes, hash, symbols));
        }
        if !new_cache.is_empty() {
            let (k, v): (Vec<String>, Vec<serde_json::Value>) = new_cache.into_iter().unzip();
            sqlx::query(
                "INSERT INTO parse_cache (cache_key, symbols)
                 SELECT * FROM unnest($1::text[], $2::jsonb[]) ON CONFLICT DO NOTHING",
            )
            .bind(k)
            .bind(v)
            .execute(&mut *tx)
            .await
            .map_err(ierr)?;
        }
        if parsed.is_empty() {
            tx.commit().await.map_err(ierr)?;
            return Ok(stats);
        }

        let paths: Vec<&str> = parsed.iter().map(|(p, ..)| p.as_str()).collect();
        let old: Vec<(i64, String, String)> = sqlx::query_as(
            "DELETE FROM symbols WHERE repo = $1 AND scope = $2 AND path = ANY($3)
             RETURNING id, name, path",
        )
        .bind(repo)
        .bind(scope)
        .bind(&paths)
        .fetch_all(&mut *tx)
        .await
        .map_err(ierr)?;
        let mut touched_names: BTreeSet<String> = BTreeSet::new();
        let mut outbox: Vec<NewOutbox> = vec![];
        for (id, name, path) in old {
            touched_names.insert(name);
            // Deletes are enqueued even when embed is false, so a later embed cannot leave orphans.
            let ctx = ChunkCtx {
                repo,
                task_id,
                change,
                path: &path,
            };
            outbox.push(symbol_outbox(&ctx, OutboxOp::Delete, id, None));
        }

        let syms: Vec<(&str, &Symbol)> = parsed
            .iter()
            .flat_map(|(p, _, _, ss)| ss.iter().map(move |s| (p.as_str(), s)))
            .collect();
        let last = |q: &str| q.rsplit('.').next().unwrap_or(q).to_owned();
        let inserted: Vec<(i64, String, String)> = sqlx::query_as(
            "INSERT INTO symbols (repo, scope, path, qname, name, kind, signature, start_line, end_line, refs)
             SELECT $1, $2, u.path, u.qname, u.name, u.kind, u.signature, u.start_line, u.end_line,
                    ARRAY(SELECT jsonb_array_elements_text(u.refs))
             FROM unnest($3::text[], $4::text[], $5::text[], $6::text[], $7::text[], $8::int[],
                         $9::int[], $10::jsonb[])
                  WITH ORDINALITY AS u(path, qname, name, kind, signature, start_line, end_line,
                                       refs, ord)
             ORDER BY u.ord
             ON CONFLICT (repo, scope, path, qname) DO NOTHING
             RETURNING id, path, qname",
        )
        .bind(repo)
        .bind(scope)
        .bind(syms.iter().map(|(p, _)| *p).collect::<Vec<_>>())
        .bind(syms.iter().map(|(_, s)| s.qname.as_str()).collect::<Vec<_>>())
        .bind(syms.iter().map(|(_, s)| last(&s.qname)).collect::<Vec<_>>())
        .bind(syms.iter().map(|(_, s)| s.kind.as_str()).collect::<Vec<_>>())
        .bind(syms.iter().map(|(_, s)| s.signature.as_str()).collect::<Vec<_>>())
        .bind(syms.iter().map(|(_, s)| s.start_line as i32).collect::<Vec<_>>())
        .bind(syms.iter().map(|(_, s)| s.end_line as i32).collect::<Vec<_>>())
        .bind(syms.iter().map(|(_, s)| serde_json::json!(s.refs)).collect::<Vec<_>>())
        .fetch_all(&mut *tx)
        .await
        .map_err(ierr)?;

        // A repeated qname in one file keeps its first symbol, as the per-row insert did.
        let mut first: HashMap<(&str, &str), (&Symbol, &[u8])> = HashMap::new();
        for (p, bytes, _, ss) in &parsed {
            for s in ss {
                first
                    .entry((p.as_str(), s.qname.as_str()))
                    .or_insert((s, bytes));
            }
        }
        let mut rebuild: BTreeSet<i64> = BTreeSet::new();
        let (mut chunk_ids, mut chunk_paths, mut chunk_texts) = (vec![], vec![], vec![]);
        for (id, path, qname) in &inserted {
            let (s, bytes) = first[&(path.as_str(), qname.as_str())];
            rebuild.insert(*id);
            touched_names.insert(last(qname));
            let text = chunk_text(path, s, bytes);
            if embed {
                let ctx = ChunkCtx {
                    repo,
                    task_id,
                    change,
                    path,
                };
                outbox.push(symbol_outbox(
                    &ctx,
                    OutboxOp::Upsert,
                    *id,
                    Some(text.clone()),
                ));
            }
            chunk_ids.push(*id);
            chunk_paths.push(path.as_str());
            chunk_texts.push(text);
        }
        sqlx::query(
            "INSERT INTO code_chunks (symbol_id, repo, scope, path, task_id, change_id, text)
             SELECT u.id, $1, $2, u.path, $3, $4, u.text
             FROM unnest($5::bigint[], $6::text[], $7::text[]) AS u(id, path, text)",
        )
        .bind(repo)
        .bind(scope)
        .bind(task_id.map(|id| id.0))
        .bind(change.map(|c| c.0.clone()))
        .bind(&chunk_ids)
        .bind(&chunk_paths)
        .bind(&chunk_texts)
        .execute(&mut *tx)
        .await
        .map_err(ierr)?;
        scholion_store::PgStore::enqueue_many(&mut *tx, &outbox)
            .await
            .map_err(|e| AvError::Index(e.to_string()))?;
        sqlx::query(
            "INSERT INTO index_files (repo, scope, path, content_hash)
             SELECT $1, $2, u.path, u.hash FROM unnest($3::text[], $4::text[]) AS u(path, hash)
             ON CONFLICT (repo, scope, path) DO UPDATE SET content_hash = EXCLUDED.content_hash",
        )
        .bind(repo)
        .bind(scope)
        .bind(&paths)
        .bind(
            parsed
                .iter()
                .map(|(_, _, h, _)| h.as_str())
                .collect::<Vec<_>>(),
        )
        .execute(&mut *tx)
        .await
        .map_err(ierr)?;

        rebuild.extend(dependents_of(&mut tx, repo, scope, &touched_names).await?);
        rebuild_edges(&mut tx, &rebuild.into_iter().collect::<Vec<_>>()).await?;
        tx.commit().await.map_err(ierr)?;
        Ok(stats)
    }

    async fn scope_change(&self, repo: &str, scope: &str) -> Result<Option<ChangeId>> {
        let at: Option<String> =
            sqlx::query_scalar("SELECT change_id FROM index_scopes WHERE repo = $1 AND scope = $2")
                .bind(repo)
                .bind(scope)
                .fetch_optional(&self.pool)
                .await
                .map_err(ierr)?;
        Ok(at.map(ChangeId))
    }

    async fn set_scope_change(&self, repo: &str, scope: &str, change: &ChangeId) -> Result<()> {
        sqlx::query(
            "INSERT INTO index_scopes (repo, scope, change_id) VALUES ($1, $2, $3)
             ON CONFLICT (repo, scope) DO UPDATE SET change_id = EXCLUDED.change_id",
        )
        .bind(repo)
        .bind(scope)
        .bind(&change.0)
        .execute(&self.pool)
        .await
        .map_err(ierr)?;
        Ok(())
    }

    async fn clone_scope(
        &self,
        repo: &str,
        from: &str,
        to: &str,
        change: &ChangeId,
    ) -> Result<bool> {
        let mut tx = self.pool.begin().await.map_err(ierr)?;
        // every statement below sees the same version of `from`, even while it is being written
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
            .execute(&mut *tx)
            .await
            .map_err(ierr)?;
        let at: Option<String> =
            sqlx::query_scalar("SELECT change_id FROM index_scopes WHERE repo = $1 AND scope = $2")
                .bind(repo)
                .bind(from)
                .fetch_optional(&mut *tx)
                .await
                .map_err(ierr)?;
        if at.as_deref() != Some(change.0.as_str()) {
            return Ok(false);
        }
        let occupied: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM index_files WHERE repo = $1 AND scope = $2)
                 OR EXISTS (SELECT 1 FROM symbols WHERE repo = $1 AND scope = $2)",
        )
        .bind(repo)
        .bind(to)
        .fetch_one(&mut *tx)
        .await
        .map_err(ierr)?;
        if occupied {
            return Err(AvError::Invalid(format!(
                "index scope {to} of {repo} already has an index"
            )));
        }
        let task_id = task_id_for_scope(&mut tx, repo, to).await?;
        // `(path, qname)` is unique per scope, so it maps each source symbol to its copy.
        let statements = [
            // source id order, so relative ids (and tie-breaks that use them) match a fresh index
            "INSERT INTO symbols (repo, scope, path, qname, name, kind, signature, start_line,
                                  end_line, refs)
             SELECT repo, $3, path, qname, name, kind, signature, start_line, end_line, refs
             FROM symbols WHERE repo = $1 AND scope = $2 ORDER BY id",
            "INSERT INTO symbol_refs (src, dst, weight)
             SELECT ns.id, nd.id, r.weight FROM symbol_refs r
             JOIN symbols os ON os.id = r.src
             JOIN symbols od ON od.id = r.dst
             JOIN symbols ns ON ns.repo = $1 AND ns.scope = $3
                            AND ns.path = os.path AND ns.qname = os.qname
             JOIN symbols nd ON nd.repo = $1 AND nd.scope = $3
                            AND nd.path = od.path AND nd.qname = od.qname
             WHERE os.repo = $1 AND os.scope = $2",
            "INSERT INTO code_chunks (symbol_id, repo, scope, path, task_id, change_id, text)
             SELECT ns.id, c.repo, $3, c.path, $4, $5, c.text FROM code_chunks c
             JOIN symbols os ON os.id = c.symbol_id
             JOIN symbols ns ON ns.repo = $1 AND ns.scope = $3
                            AND ns.path = os.path AND ns.qname = os.qname
             WHERE os.repo = $1 AND os.scope = $2",
            "INSERT INTO index_files (repo, scope, path, content_hash)
             SELECT repo, $3, path, content_hash FROM index_files
             WHERE repo = $1 AND scope = $2",
            "INSERT INTO index_scopes (repo, scope, change_id) VALUES ($1, $3, $5)
             ON CONFLICT (repo, scope) DO UPDATE SET change_id = EXCLUDED.change_id",
        ];
        for sql in statements {
            // the same five parameters for each (a typed parameter a statement ignores is fine)
            sqlx::query(sql)
                .bind(repo)
                .bind(from)
                .bind(to)
                .bind(task_id.map(|id| id.0))
                .bind(&change.0)
                .execute(&mut *tx)
                .await
                .map_err(ierr)?;
        }
        tx.commit().await.map_err(ierr)?;
        Ok(true)
    }

    async fn list_code_chunks(&self, repo: Option<&str>) -> Result<Vec<CodeChunk>> {
        let rows = sqlx::query(
            "SELECT symbol_id, repo, scope, path, task_id, change_id, text
             FROM code_chunks
             WHERE $1::text IS NULL OR repo = $1
             ORDER BY symbol_id",
        )
        .bind(repo)
        .fetch_all(&self.pool)
        .await
        .map_err(ierr)?;
        let mut out = Vec::with_capacity(rows.len());
        for r in &rows {
            let task_id: Option<Uuid> = r.try_get("task_id").map_err(ierr)?;
            let change_id: Option<String> = r.try_get("change_id").map_err(ierr)?;
            out.push(CodeChunk {
                symbol_id: SymbolId(r.try_get("symbol_id").map_err(ierr)?),
                repo: r.try_get("repo").map_err(ierr)?,
                scope: r.try_get("scope").map_err(ierr)?,
                path: r.try_get("path").map_err(ierr)?,
                task_id: task_id.map(TaskId),
                change_id: change_id.map(ChangeId),
                text: r.try_get("text").map_err(ierr)?,
            });
        }
        Ok(out)
    }

    async fn chunk_exists(&self, symbol_id: SymbolId) -> Result<bool> {
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM code_chunks WHERE symbol_id = $1)")
            .bind(symbol_id.0)
            .fetch_one(&self.pool)
            .await
            .map_err(ierr)
    }

    async fn remove_paths(&self, repo: &str, scope: &str, paths: &[String]) -> Result<()> {
        let mut tx = self.pool.begin().await.map_err(ierr)?;
        forget_scope_change(&mut tx, repo, scope).await?;
        let task_id = task_id_for_scope(&mut tx, repo, scope).await?;
        let removed = sqlx::query(
            "DELETE FROM symbols WHERE repo = $1 AND scope = $2 AND path = ANY($3) RETURNING id, name, path",
        )
        .bind(repo)
        .bind(scope)
        .bind(paths)
        .fetch_all(&mut *tx)
        .await
        .map_err(ierr)?;
        sqlx::query("DELETE FROM index_files WHERE repo = $1 AND scope = $2 AND path = ANY($3)")
            .bind(repo)
            .bind(scope)
            .bind(paths)
            .execute(&mut *tx)
            .await
            .map_err(ierr)?;
        let mut names = BTreeSet::new();
        for r in &removed {
            let id: i64 = r.try_get("id").map_err(ierr)?;
            names.insert(r.try_get::<String, _>("name").map_err(ierr)?);
            let path: String = r.try_get("path").map_err(ierr)?;
            let ctx = ChunkCtx {
                repo,
                task_id,
                change: None,
                path: &path,
            };
            enqueue_symbol(&mut tx, &ctx, OutboxOp::Delete, id, None).await?;
        }
        let dependents = dependents_of(&mut tx, repo, scope, &names).await?;
        rebuild_edges(&mut tx, &dependents).await?;
        tx.commit().await.map_err(ierr)
    }

    async fn symbols_in(&self, repo: &str, scope: &str, path: &str) -> Result<Vec<IndexedSymbol>> {
        let rows = sqlx::query(&format!(
            "SELECT {SYMBOL_COLS} FROM symbols WHERE repo = $1 AND scope = $2 AND path = $3 ORDER BY start_line, id"
        ))
        .bind(repo)
        .bind(scope)
        .bind(path)
        .fetch_all(&self.pool)
        .await
        .map_err(ierr)?;
        rows.iter().map(symbol_from_row).collect()
    }

    async fn get_symbols(&self, ids: &[SymbolId]) -> Result<Vec<IndexedSymbol>> {
        let ids: Vec<i64> = ids.iter().map(|s| s.0).collect();
        let rows = sqlx::query(&format!(
            "SELECT {SYMBOL_COLS} FROM symbols WHERE id = ANY($1) ORDER BY id"
        ))
        .bind(&ids)
        .fetch_all(&self.pool)
        .await
        .map_err(ierr)?;
        rows.iter().map(symbol_from_row).collect()
    }

    async fn search_symbols(
        &self,
        repo: &str,
        scope: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<IndexedSymbol>> {
        if query.trim().is_empty() {
            return Ok(vec![]);
        }
        // position() is a literal substring test, so '%' and '_' in the query need no escaping.
        let rows = sqlx::query(&format!(
            "SELECT {SYMBOL_COLS} FROM symbols
             WHERE repo = $1 AND scope = $2
               AND (position(lower($3) in lower(qname)) > 0 OR position(lower($3) in lower(path)) > 0)
             ORDER BY (lower(qname) = lower($3)) DESC, path COLLATE \"C\", start_line, id
             LIMIT $4"
        ))
        .bind(repo)
        .bind(scope)
        .bind(query.trim())
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(ierr)?;
        rows.iter().map(symbol_from_row).collect()
    }

    async fn search_code_lexical(
        &self,
        repo: &str,
        scope: &str,
        terms: &[String],
        limit: usize,
    ) -> Result<Vec<(SymbolId, f32)>> {
        if terms.is_empty() || limit == 0 {
            return Ok(vec![]);
        }
        let n: i64 =
            sqlx::query_scalar("SELECT count(*) FROM code_chunks WHERE repo = $1 AND scope = $2")
                .bind(repo)
                .bind(scope)
                .fetch_one(&self.pool)
                .await
                .map_err(ierr)?;
        if n == 0 {
            return Ok(vec![]);
        }
        let dfs: Vec<(String, i64)> = sqlx::query_as(
            "SELECT t, count(c.symbol_id) FROM unnest($3::text[]) AS t
             LEFT JOIN code_chunks c ON c.repo = $1 AND c.scope = $2
                                    AND c.tsv @@ plainto_tsquery('simple', t)
             GROUP BY t",
        )
        .bind(repo)
        .bind(scope)
        .bind(terms)
        .fetch_all(&self.pool)
        .await
        .map_err(ierr)?;
        let (mut words, mut idfs) = (vec![], vec![]);
        for (t, df) in dfs.into_iter().filter(|(_, df)| *df > 0) {
            let (n, df) = (n as f64, df as f64);
            words.push(t);
            idfs.push(((n - df + 0.5) / (df + 0.5) + 1.0).ln());
        }
        if words.is_empty() {
            return Ok(vec![]);
        }
        // ts_rank with normalization 1 divides by 1 + log(chunk length), so long chunks do not
        // win on length alone.
        let rows: Vec<(i64, f64)> = sqlx::query_as(
            "SELECT c.symbol_id, sum(w.idf * ts_rank(c.tsv, plainto_tsquery('simple', w.t), 1))::float8 AS s
             FROM code_chunks c
             JOIN unnest($3::text[], $4::float8[]) AS w(t, idf)
               ON c.tsv @@ plainto_tsquery('simple', w.t)
             WHERE c.repo = $1 AND c.scope = $2
             GROUP BY c.symbol_id
             ORDER BY s DESC, c.symbol_id
             LIMIT $5",
        )
        .bind(repo)
        .bind(scope)
        .bind(&words)
        .bind(&idfs)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(ierr)?;
        Ok(rows
            .into_iter()
            .map(|(id, s)| (SymbolId(id), s as f32))
            .collect())
    }

    async fn resolve(&self, repo: &str, scope: &str, anchor: &Anchor) -> Result<AnchorResolution> {
        if let Some(q) = &anchor.symbol {
            let hit: Option<(i64,)> =
                sqlx::query_as("SELECT id FROM symbols WHERE repo = $1 AND scope = $2 AND path = $3 AND qname = $4")
                    .bind(repo)
                    .bind(scope)
                    .bind(&anchor.path)
                    .bind(q)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(ierr)?;
            if let Some((id,)) = hit {
                return Ok(AnchorResolution::Symbol(SymbolId(id)));
            }
        }
        let file: Option<(i32,)> = sqlx::query_as(
            "SELECT 1 FROM index_files WHERE repo = $1 AND scope = $2 AND path = $3",
        )
        .bind(repo)
        .bind(scope)
        .bind(&anchor.path)
        .fetch_optional(&self.pool)
        .await
        .map_err(ierr)?;
        Ok(match file {
            Some(_) => AnchorResolution::PathOnly(anchor.path.clone()),
            None => AnchorResolution::Stale,
        })
    }

    async fn neighbors(
        &self,
        _repo: &str,
        _scope: &str,
        seeds: &[SymbolId],
        hops: u8,
    ) -> Result<Vec<(SymbolId, u8)>> {
        if seeds.is_empty() || hops == 0 {
            return Ok(vec![]);
        }
        let seed_ids: Vec<i64> = seeds.iter().map(|s| s.0).collect();
        let limit = hops.min(MAX_HOPS) as i32;
        // Edges only ever connect symbols of the same (repo, scope), so seeds' scope is preserved.
        let rows = sqlx::query(
            "WITH RECURSIVE walk(id, hops) AS (
                 SELECT unnest($1::bigint[]), 0
               UNION
                 SELECT CASE WHEN e.src = w.id THEN e.dst ELSE e.src END, w.hops + 1
                 FROM walk w JOIN symbol_refs e ON (e.src = w.id OR e.dst = w.id)
                 WHERE w.hops < $2
             )
             SELECT id, MIN(hops) AS hops FROM walk WHERE NOT (id = ANY($1)) GROUP BY id ORDER BY MIN(hops), id",
        )
        .bind(&seed_ids)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(ierr)?;
        rows.iter()
            .map(|r| {
                Ok((
                    SymbolId(r.try_get("id").map_err(ierr)?),
                    r.try_get::<i32, _>("hops").map_err(ierr)? as u8,
                ))
            })
            .collect()
    }
    async fn symbol_scope(&self, id: SymbolId) -> Result<Option<(String, String)>> {
        sqlx::query_as("SELECT repo, scope FROM symbols WHERE id = $1")
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(ierr)
    }

    async fn drop_scope(&self, repo: &str, scope: &str) -> Result<()> {
        let mut tx = self.pool.begin().await.map_err(ierr)?;
        forget_scope_change(&mut tx, repo, scope).await?;
        let task_id = task_id_for_scope(&mut tx, repo, scope).await?;
        let chunks: Vec<(i64, String)> = sqlx::query_as(
            "SELECT symbol_id, path FROM code_chunks WHERE repo = $1 AND scope = $2
             ORDER BY symbol_id",
        )
        .bind(repo)
        .bind(scope)
        .fetch_all(&mut *tx)
        .await
        .map_err(ierr)?;
        let deletes: Vec<NewOutbox> = chunks
            .iter()
            .map(|(id, path)| {
                let ctx = ChunkCtx {
                    repo,
                    task_id,
                    change: None,
                    path,
                };
                symbol_outbox(&ctx, OutboxOp::Delete, *id, None)
            })
            .collect();
        scholion_store::PgStore::enqueue_many(&mut *tx, &deletes)
            .await
            .map_err(|e| AvError::Index(e.to_string()))?;
        // edges and chunks go with their symbols (ON DELETE CASCADE)
        for table in ["symbols", "index_files"] {
            sqlx::query(&format!(
                "DELETE FROM {table} WHERE repo = $1 AND scope = $2"
            ))
            .bind(repo)
            .bind(scope)
            .execute(&mut *tx)
            .await
            .map_err(ierr)?;
        }
        tx.commit().await.map_err(ierr)
    }

    async fn prune_parse_cache(&self) -> Result<u64> {
        // keyed `<content sha256>:<ext>`; kept while any scope holds a file with that content
        let done = sqlx::query(
            "DELETE FROM parse_cache p WHERE NOT EXISTS (
                 SELECT 1 FROM index_files f WHERE f.content_hash = split_part(p.cache_key, ':', 1))",
        )
        .execute(&self.pool)
        .await
        .map_err(ierr)?;
        Ok(done.rows_affected())
    }

    async fn in_degree(&self, ids: &[SymbolId]) -> Result<HashMap<SymbolId, u32>> {
        if ids.is_empty() {
            return Ok(HashMap::new());
        }
        let ids: Vec<i64> = ids.iter().map(|s| s.0).collect();
        let rows: Vec<(i64, i64)> = sqlx::query_as(
            "SELECT dst, count(*) FROM symbol_refs WHERE dst = ANY($1) GROUP BY dst",
        )
        .bind(&ids)
        .fetch_all(&self.pool)
        .await
        .map_err(ierr)?;
        Ok(rows
            .into_iter()
            .map(|(id, n)| (SymbolId(id), n as u32))
            .collect())
    }
}
