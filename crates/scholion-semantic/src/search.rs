use async_trait::async_trait;
use scholion_core::{AvError, ConventionId, Result, Semantic, SemanticHit, SemanticKind, SymbolId};

use crate::embed::Embedder;
use crate::vector::VectorIndex;

/// Embeds a query and searches the three collections.
pub struct SemanticSearch<E, V> {
    embedder: E,
    index: V,
}

impl<E, V> SemanticSearch<E, V> {
    pub fn new(embedder: E, index: V) -> Self {
        Self { embedder, index }
    }
}

#[async_trait]
impl<E, V> Semantic for SemanticSearch<E, V>
where
    E: Embedder,
    V: VectorIndex,
{
    async fn search(&self, repo: &str, query: &str, limit: usize) -> Result<Vec<SemanticHit>> {
        self.search_with(repo, query, limit, limit).await
    }

    /// One embedding of the query, then each collection with its own cap; a cap of 0 skips the
    /// collection.
    async fn search_with(
        &self,
        repo: &str,
        query: &str,
        limit: usize,
        record_limit: usize,
    ) -> Result<Vec<SemanticHit>> {
        let query = query.trim();
        if query.is_empty() {
            return Ok(Vec::new());
        }
        let embedded = self.embedder.embed(&[query.to_string()]).await?;
        if embedded.len() != 1 {
            return Err(AvError::Semantic(format!(
                "embedder returned {} vectors for 1 input",
                embedded.len()
            )));
        }
        let vector = &embedded[0];
        let dim = self.embedder.dimension().await?;
        if vector.len() != dim {
            return Err(AvError::Semantic(format!(
                "query vector length is {}, embedder dimension is {dim}",
                vector.len()
            )));
        }
        let capped = |kind, cap: usize| async move {
            if cap == 0 {
                Ok(Vec::new())
            } else {
                self.index.search(kind, repo, vector, cap).await
            }
        };
        let (records, conventions, chunks) = tokio::join!(
            capped(SemanticKind::Record, record_limit),
            capped(SemanticKind::Convention, limit),
            capped(SemanticKind::CodeChunk, limit),
        );
        let mut hits = Vec::new();
        hits.extend(records?);
        hits.extend(conventions?);
        hits.extend(chunks?);
        hits.retain(|hit| !hit.superseded && hit.repo == repo);
        hits.sort_by(|left, right| {
            right
                .score
                .total_cmp(&left.score)
                .then_with(|| entity_key(left).cmp(&entity_key(right)))
        });
        Ok(hits)
    }
}

/// `record_id`, else `symbol_id`, else `convention_id`, in display form.
fn entity_key(hit: &SemanticHit) -> String {
    if let Some(id) = hit.record_id {
        return id.to_string();
    }
    if let Some(SymbolId(id)) = hit.symbol_id {
        return id.to_string();
    }
    if let Some(ConventionId(id)) = hit.convention_id {
        return id.to_string();
    }
    String::new()
}
