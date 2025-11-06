use crate::llm_client::LlmClient;
use crate::tools::ToolBox;
use anyhow::Result;

pub struct Agent {
    pub name: String,
    llm: LlmClient,
    tools: ToolBox,
}

impl Agent {
    pub fn new(name: &str, llm: LlmClient, tools: ToolBox) -> Self {
        Self {
            name: name.to_string(),
            llm,
            tools,
        }
    }

    pub async fn handle(&self, input: &str) -> Result<String> {
        if let Some(output) = self.tools.try_tool(input).await? {
            return Ok(output);
        }

        let response = self.llm.query(input).await?;
        Ok(response)
    }
}
