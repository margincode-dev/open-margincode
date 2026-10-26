mod embed;
mod ollama;
mod search;
mod vector;

pub use embed::{Embedder, ScriptedEmbedder, TeiEmbedder};
pub use ollama::OllamaEmbedder;
pub use search::SemanticSearch;
pub use vector::{
    COLLECTION_CODE_CHUNKS, COLLECTION_CONVENTIONS, COLLECTION_RECORDS, MemoryIndex, QdrantIndex,
    UpsertPoint, VectorIndex,
};
