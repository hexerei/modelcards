use std::path::PathBuf;
use clap::Parser;
use jayson::preview::PreviewError;
use serde_json::Value;

mod project;

#[derive(Parser)]
#[command(name = "modelcards-edit", about = "Visual model card editor")]
struct Cli {
    /// Project root directory
    #[arg(short, long, default_value = ".")]
    root: PathBuf,

    /// Port to serve on (0 = random available port)
    #[arg(short, long, default_value = "3000")]
    port: u16,

    /// Path to JSON Schema file (defaults to built-in Google Model Card schema)
    #[arg(short, long)]
    schema: Option<PathBuf>,

    /// Layer files to edit (if none given, starts with an empty layer)
    layers: Vec<PathBuf>,
}

/// Preview renderer that uses modelcards' built-in HTML template.
struct ModelCardPreview;

impl jayson::preview::PreviewRenderer for ModelCardPreview {
    fn render(&self, data: &Value) -> Result<String, PreviewError> {
        modelcards::render::render_value_to_html(data.clone())
            .map_err(|e| PreviewError {
                message: e.to_string(),
            })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let project = project::detect(&cli.root);

    // Resolve schema: CLI flag > project detection > built-in default
    let schema: Value = if let Some(ref path) = cli.schema {
        jayson::schema::load_json(path)?
    } else if let Some(ref path) = project.schema {
        jayson::schema::load_json(path)?
    } else {
        serde_json::from_str(modelcards::assets::schema::get_schema())?
    };

    let layer_files = if cli.layers.is_empty() {
        project.data_files
    } else {
        cli.layers
    };

    let mut builder = jayson::Jayson::builder()
        .preview(ModelCardPreview)
        .port(cli.port)
        .title("Model Card Editor")
        .schema(schema);

    if layer_files.is_empty() {
        // Start with a single empty layer that can be saved
        builder = builder.layer(jayson::layer::Layer {
            name: "modelcard".to_string(),
            data: serde_json::json!({}),
            source: jayson::layer::LayerSource::File(
                cli.root.join("modelcard.json").canonicalize()
                    .unwrap_or_else(|_| cli.root.join("modelcard.json")),
            ),
            readonly: false,
        });
    } else {
        for path in &layer_files {
            let data = jayson::schema::load_json(path)?;
            builder = builder.layer(jayson::layer::Layer {
                name: path
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unnamed".to_string()),
                data,
                source: jayson::layer::LayerSource::File(path.clone()),
                readonly: false,
            });
        }
    }

    let editor = builder.build()?;
    editor.serve().await?;

    Ok(())
}
