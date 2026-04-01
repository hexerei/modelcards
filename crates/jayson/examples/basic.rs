use jayson::layer::{Layer, LayerSource};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let editor = jayson::Jayson::builder()
        .layer(Layer {
            name: "defaults".to_string(),
            data: json!({
                "model_details": {
                    "name": "Example Model",
                    "version": "1.0",
                    "type": "classification"
                },
                "considerations": {
                    "users": ["ML engineers"],
                    "use_cases": ["Text classification"]
                }
            }),
            source: LayerSource::Memory,
            readonly: true,
        })
        .layer(Layer {
            name: "my_model".to_string(),
            data: json!({
                "model_details": {
                    "name": "My Custom Model",
                    "description": "A fine-tuned classifier"
                }
            }),
            source: LayerSource::Memory,
            readonly: false,
        })
        .port(3000)
        .title("Example Editor")
        .build()?;

    println!("Merged: {}", serde_json::to_string_pretty(&editor.merged_value())?);
    println!("\nStarting editor...");

    editor.serve().await?;

    Ok(())
}
