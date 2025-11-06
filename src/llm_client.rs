use llm::{Ollama, OllamaModel};
use anyhow::Result;

pub struct LlmClient {
    model: OllamaModel,
}

impl LlmClient {
    pub fn new(model: &str) -> Result<Self> { Ok ( Self {model: OllamaModel::new(model),})}

    pub async fn query(&self, prompt: &str) -> Result<String> {
        let ollama = Ollama::default();
        let response = ollama
            .generate(&self.model, prompt)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(response.output_text)
    }
}
