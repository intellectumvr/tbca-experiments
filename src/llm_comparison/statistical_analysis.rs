//! Statistical analysis utilities

#[derive(Debug, Clone)]
pub struct Statistics {
    pub mean: f64,
    pub std_dev: f64,
    pub ci_lower: f64,
    pub ci_upper: f64,
}

impl Statistics {
    pub fn from_data(data: &[f64]) -> Self {
        if data.is_empty() {
            return Self {
                mean: 0.0,
                std_dev: 0.0,
                ci_lower: 0.0,
                ci_upper: 0.0,
            };
        }
        
        let mean = data.iter().sum::<f64>() / data.len() as f64;
        
        let variance = if data.len() > 1 {
            data.iter()
                .map(|x| (x - mean).powi(2))
                .sum::<f64>() / (data.len() - 1) as f64
        } else {
            0.0
        };
        
        let std_dev = variance.sqrt();
        
        // 95% CI using t-distribution approximation (z=1.96 for large n)
        let margin = 1.96 * std_dev / (data.len() as f64).sqrt();
        
        Self {
            mean,
            std_dev,
            ci_lower: mean - margin,
            ci_upper: mean + margin,
        }
    }
}
