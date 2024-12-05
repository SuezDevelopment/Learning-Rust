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
        temperature: f32,
        max_tokens: u32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let options = GenerationOptions::default().temperature(temperature).top_k(max_tokens);
        let request = GenerationRequest::new(model.to_string(), prompt.to_string()).options(options);
    
        match self.client.generate(request).await {
            Ok(response) => Ok(response.response),
            Err(e) => Err(Box::new(e))
        }
    }
    
}