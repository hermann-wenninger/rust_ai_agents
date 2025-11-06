mod llm_client;
mod agent;
mod tools;

use crate::agent::Agent;
use crate::llm_client::LlmClient;
use crate::tools::ToolBox;
use anyhow::Result;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let llm = LlmClient::new("mistral")?;
    let tools = ToolBox::new();
    let agent = Agent::new("Aqvise", llm, tools);

    println!("🧠 Aqvise Agent aktiv. Tippe deine Anfrage:");
    let mut input = String::new();

    loop {
        input.clear();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().eq_ignore_ascii_case("exit") {
            break;
        }
        let output = agent.handle(&input.trim()).await?;
        println!("🤖 Antwort: {output}\n");
    }

    Ok(())
}
