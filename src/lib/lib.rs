//! # Modelcards Lib
//! 
//! Flexible library with assets and utility functions to work with model cards.
//! 
//! Model cards are typically used to document all aspects of a machine learning model, like the model's purpose, performance, and limitations.
//! The most common visual representation of a model card is a markdown file, but the library can also render model cards to other formats.
//! 
//! To support automatic creation and updates of model cards, the library provides functions to deal with model card data stored in JSON files.
//! The model card data files can be merged to create a single model card from multiple sources (e.g. defaults, use-case level common data, model details).
//! The library also provides a schema file to validate model cards against a predefined structure as used in Googles model card toolkit, but you can provide your own schema for custom formats.
//! Finally the generated model card can be rendered to a markdown file or any other format supported by the Jinja2 templating engine.
//! 
//! To simplify the creation of model cards, the library provides a command line interface (CLI) application.
//! 
//! ## Usage
//! 
//! The library provides several functions to support the modelcards cli application, but can be used as a library in your own projects.
//! 
//! The functions are divided into modules:
//! - `assets`: Contains the assets used by the library, like templates and schemas.
//! - `merge`: Functions to merge multiple model data files.
//! - `render`: Functions to render model cards using Jinja templates.
//! - `utils`: Utility functions used by the library.
//! - `validate`: Functions to validate modelcards against a schema file.

/// Contains the assets used by the library, like templates and schemas.
pub mod assets;
/// Functions to merge multiple model data files.
pub mod merge;
/// Functions to render model cards using Jinja templates.
pub mod render;
/// Utility functions used by the library.
pub mod utils;
/// Functions to validate modelcards against a schema file.
pub mod validate;

/// Defines the theme to use for rendering the model card.
/// 
/// The theme defines the style and layout of the rendered model card.
/// 
/// The available themes are:
/// - `HuggingFace`: The default theme used by the HuggingFace model card toolkit.
/// - `Lazy`: A simple theme with minimal styling.
/// - `Google`: The theme used by the Google model card toolkit.
/// 
/// <div class="warning">In the current version the Theme is not used, as only the Google format is implemented!</div>
pub enum Theme {
    HuggingFace,
    Lazy,
    Google,
}