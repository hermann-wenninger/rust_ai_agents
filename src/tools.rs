use anyhow::Result;

pub struct ToolBox;

impl ToolBox {
    pub fn new() -> Self { Self }

    pub async fn try_tool(&self, input: &str) -> Result<Option<String>> {
        if input.contains("time") {
            Ok(Some(format!("Aktuelle Zeit: {:?}", chrono::Local::now())))
        } else if input.contains("sum") {
            Ok(Some("Summe = 42".to_string()))
        } else {
            Ok(None)
        }
    }
}
