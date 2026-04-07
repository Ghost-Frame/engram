use std::sync::Arc;
use engram_lib::config::Config;
use engram_lib::db::Database;
use engram_lib::embeddings::EmbeddingProvider;
use engram_lib::reranker::Reranker;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub config: Arc<Config>,
    pub embedder: Option<Arc<dyn EmbeddingProvider>>,
    pub reranker: Option<Arc<Reranker>>,
}
