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
        let (records, conventions, chunks) = tokio::join!(
            self.index.search(SemanticKind::Record, repo, vector, limit),
            self.index
                .search(SemanticKind::Convention, repo, vector, limit),
            self.index
                .search(SemanticKind::CodeChunk, repo, vector, limit),
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
