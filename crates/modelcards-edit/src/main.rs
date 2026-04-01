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
    #[arg(short, long, default_value = "0")]
    port: u16,

    /// Path to JSON Schema file
    #[arg(short, long)]
    schema: Option<PathBuf>,

    /// Layer files to edit (positional)
    layers: Vec<PathBuf>,
}

/// Preview renderer that uses modelcards' built-in HTML template.
struct ModelCardPreview;

impl jayson::preview::PreviewRenderer for ModelCardPreview {
    fn render(&self, data: &Value) -> Result<String, PreviewError> {
        let template_content = modelcards::assets::templates::get_html();
        let mut env = minijinja::Environment::new();
        env.add_template("modelcard.html", template_content)
            .map_err(|e| PreviewError { message: format!("Template load error: {}", e) })?;
        let template = env.get_template("modelcard.html")
            .map_err(|e| PreviewError { message: format!("Template get error: {}", e) })?;
        template.render(data)
            .map_err(|e| PreviewError { message: format!("Render error: {}", e) })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let project = project::detect(&cli.root);

    // Resolve schema: CLI flag > project detection > embedded default
    let schema: Option<Value> = if let Some(ref path) = cli.schema {
        Some(jayson::schema::load_json(path)?)
    } else if let Some(ref path) = project.schema {
        Some(jayson::schema::load_json(path)?)
    } else {
        Some(serde_json::from_str(modelcards::assets::schema::get_schema())?)
    };

    let layer_files = if cli.layers.is_empty() {
        project.data_files
    } else {
        cli.layers
    };

    anyhow::ensure!(!layer_files.is_empty(), "No layer files specified and no project data files found.\nUsage: modelcards-edit [OPTIONS] [LAYERS]...");

    // Build jayson editor
    let mut builder = jayson::Jayson::builder()
        .preview(ModelCardPreview)
        .port(cli.port)
        .title("Model Card Editor");

    if let Some(s) = schema {
        builder = builder.schema(s);
    }

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

    let editor = builder.build()?;
    editor.serve().await?;

    Ok(())
}
