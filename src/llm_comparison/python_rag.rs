//! Python-based RAG using sentence-transformers

use pyo3::prelude::*;
use pyo3::types::PyList;
use std::path::PathBuf;
use std::env;

pub struct PythonRAG {
    initialized: bool,
}

impl PythonRAG {
    pub fn new() -> Self {
        let venv_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("venv");
        
        if venv_path.exists() {
            env::set_var("VIRTUAL_ENV", &venv_path);
            let python_path = venv_path.join("lib/python3.11/site-packages");
            env::set_var("PYTHONPATH", python_path);
        }
        
        Self { initialized: false }
    }
    
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Ok(());
        }
        
        Python::with_gil(|py| -> Result<(), String> {
            let sys = py.import("sys").map_err(|e| e.to_string())?;
            let path: &PyList = sys.getattr("path")
                .map_err(|e| e.to_string())?
                .downcast()
                .map_err(|e| e.to_string())?;
            
            let python_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("python")
                .to_str()
                .ok_or_else(|| "Invalid path".to_string())?
                .to_string();
            
            path.insert(0, python_dir).map_err(|e| e.to_string())?;
            
            py.import("rag_backend").map_err(|e| {
                format!("Failed to import rag_backend: {}. Make sure venv is set up.", e)
            })?;
            
            Ok(())
        })?;
        
        self.initialized = true;
        println!("✓ Python RAG initialized");
        Ok(())
    }
    
    pub fn add_document(&mut self, text: &str) -> Result<(), String> {
        self.initialize()?;
        
        Python::with_gil(|py| {
            let rag = py.import("rag_backend").map_err(|e| e.to_string())?;
            rag.call_method1("add_document", (text,))
                .map_err(|e| e.to_string())?;
            Ok(())
        })
    }
    
    pub fn retrieve(&mut self, query: &str, top_k: usize) -> Result<Vec<String>, String> {
        self.initialize()?;
        
        Python::with_gil(|py| {
            let rag = py.import("rag_backend").map_err(|e| e.to_string())?;
            let results: &PyList = rag
                .call_method1("retrieve", (query, top_k))
                .map_err(|e| e.to_string())?
                .downcast()
                .map_err(|e| e.to_string())?;
            
            results.iter()
                .map(|item| item.extract::<String>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_python_rag() {
        let mut rag = PythonRAG::new();
        
        rag.add_document("Goods delivered Feb 28").unwrap();
        rag.add_document("Payment made March 25").unwrap();
        
        let results = rag.retrieve("delivery", 1).unwrap();
        assert_eq!(results.len(), 1);
        println!("✓ Retrieved: {:?}", results);
    }
}
