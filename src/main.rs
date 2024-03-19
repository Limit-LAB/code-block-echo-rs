use std::collections::HashMap;

use limithub_code_block_sdk::{CodeBlockAppBuilder, Port, PortValue, PortValueType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    CodeBlockAppBuilder::new()
        .inputs([Port::new("text", PortValueType::String)])
        .outputs([Port::new("text", PortValueType::String)])
        .build()?
        .serve(handler)
        .await?;
    Ok(())
}

async fn handler(inputs: HashMap<String, PortValue>) -> Result<HashMap<String, PortValue>, String> {
    Ok(inputs)
}
