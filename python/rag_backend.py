"""Simple RAG using sentence-transformers"""
from sentence_transformers import SentenceTransformer
import numpy as np

class SimpleRAG:
    def __init__(self):
        print("Loading embedding model (first time: ~80MB download)...")
        self.model = SentenceTransformer('all-MiniLM-L6-v2')
        self.documents = []
        self.embeddings = []
        print("✓ Model loaded")
    
    def add_document(self, text: str):
        embedding = self.model.encode(text)
        self.documents.append(text)
        self.embeddings.append(embedding)
    
    def retrieve(self, query: str, top_k: int = 3):
        if not self.documents:
            return []
        
        query_emb = self.model.encode(query)
        similarities = []
        
        for doc_emb in self.embeddings:
            sim = float(np.dot(query_emb, doc_emb) / 
                       (np.linalg.norm(query_emb) * np.linalg.norm(doc_emb)))
            similarities.append(sim)
        
        top_indices = np.argsort(similarities)[-top_k:][::-1]
        return [self.documents[int(i)] for i in top_indices]

_rag = None

def get_rag():
    global _rag
    if _rag is None:
        _rag = SimpleRAG()
    return _rag

def add_document(text: str):
    get_rag().add_document(text)

def retrieve(query: str, top_k: int = 3):
    return get_rag().retrieve(query, top_k)
