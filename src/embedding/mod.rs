pub mod embedder_trait;
pub use embedder_trait::*;
mod error;
pub mod ollama;
pub mod openai;
pub use error::*;

#[cfg(feature = "fastembed")]
mod fastembed;
#[cfg(feature = "fastembed")]
pub use fastembed::*;
