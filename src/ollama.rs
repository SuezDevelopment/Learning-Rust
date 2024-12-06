use ollama_rs::{
    generation::{completion::request::GenerationRequest, options::GenerationOptions},
    Ollama,
};


pub struct OllamaAI {
    client: Ollama,
}

impl OllamaAI {
    pub fn new() -> Self {
        Self {
            client: Ollama::default(),
        }
    }

    pub async fn generate_text(
        &self,
        model: &str,
        prompt: &str,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let temperature = temperature.unwrap_or(2.7);
        let max_tokens = max_tokens.unwrap_or(9);
        
        let options = GenerationOptions::default().temperature(temperature).top_k(max_tokens);
        let request = GenerationRequest::new(model.to_string(), prompt.to_string()).options(options);
    
        match self.client.generate(request).await {
            Ok(response) => Ok(response.response),
            Err(e) => Err(Box::new(e))
        }
    }
    
}