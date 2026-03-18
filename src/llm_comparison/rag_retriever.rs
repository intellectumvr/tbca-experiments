// crates/experiments/src/llm_comparison/rag_retriever.rs
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;

pub struct RAGRetriever {
    embeddings_model: SentenceEmbeddingsModel,
    knowledge_base: Vec<(String, Vec<f32>)>, // (text, embedding)
}

impl RAGRetriever {
    pub fn new() -> Self {
        let model = SentenceEmbeddingsBuilder::remote(
            SentenceEmbeddingsModelType::AllMiniLmL12V2
        ).create_model().unwrap();
        
        Self {
            embeddings_model: model,
            knowledge_base: Vec::new(),
        }
    }
    
    pub fn add_knowledge(&mut self, text: &str) {
        let embedding = self.embeddings_model.encode(&[text]).unwrap()[0].clone();
        self.knowledge_base.push((text.to_string(), embedding));
    }
    
    pub fn retrieve_top_k(&self, query: &str, k: usize) -> Vec<String> {
        let query_emb = self.embeddings_model.encode(&[query]).unwrap()[0];
        
        let mut scores: Vec<_> = self.knowledge_base.iter()
            .map(|(text, emb)| {
                let score = cosine_similarity(&query_emb, emb);
                (text.clone(), score)
            })
            .collect();
        
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scores.into_iter().take(k).map(|(t, _)| t).collect()
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot / (norm_a * norm_b)
}