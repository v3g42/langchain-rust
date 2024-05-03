#[cfg(feature = "fastembed")]
use langchain_rust::embedding::{Embedder, EmbeddingModel, FastEmbed, InitOptions, TextEmbedding};
#[cfg(feature = "fastembed")]
#[tokio::main]
async fn main() {
    //Default
    let fastembed = FastEmbed::try_new().unwrap();
    let embeddings = fastembed
        .embed_documents(&["hello world".to_string(), "foo bar".to_string()])
        .await
        .unwrap();

    println!("Len: {:?}", embeddings.len());

    //With custom model

    let model = TextEmbedding::try_new(InitOptions {
        model_name: EmbeddingModel::AllMiniLML6V2,
        show_download_progress: true,
        ..Default::default()
    })
    .unwrap();

    let fastembed = FastEmbed::from(model);

    fastembed
        .embed_documents(&["hello world".to_string(), "foo bar".to_string()])
        .await
        .unwrap();

    println!("Len: {:?}", embeddings.len());
}
#[cfg(not(feature = "fastembed"))]
fn main() {
    println!("enable fastembed feature")
}
