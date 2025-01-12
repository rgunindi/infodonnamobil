#![allow(non_snake_case)]
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dotenv::dotenv;
#[cfg(feature = "server")]
use mongodb::{options::ClientOptions, Client};
#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use std::env;

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize)]
struct MarkdownEntry {
    content: String,
}

fn main() {
    dioxus::launch(App);
}
#[component]
pub fn App() -> Element {
    let markdown_content = use_signal(String::new);
    
    // Side effect ile asenkron veriyi yükle
    use_effect(move || {
        to_owned![markdown_content];
        spawn(async move {
            if let Ok(content) = get_markdown().await {
                markdown_content.set(content);
            } else {
                markdown_content.set("# Error\nFailed to load markdown.".to_string());
            }
        });
    });

    rsx! {
        head {
            style { "{include_str!(\"../assets/style.css\")}" }
        }
        div { class: "container",
            div { class: "markdown-preview",
                dangerous_inner_html: if markdown_content().is_empty() {
                    String::from("<p>Loading...</p>")
                } else {
                    markdown::to_html(&markdown_content())
                }
            }
        }
    }
}
#[cfg(feature = "server")]
use futures::TryStreamExt;
/// Markdown içeriğini sunucuda bir dosyaya kaydeder.
#[server(SaveMarkdown)]
async fn get_markdown() -> Result<String, ServerFnError> {
    const DEFAULT_MARKDOWN: &str = "# Welcome\nThis is a default markdown content.";

    //1. MongoDB'den cek
    #[cfg(feature = "server")]
    dotenv().ok();
    let client_uri =
        env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let client_options = ClientOptions::parse(client_uri)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse MongoDB URI: {e}")))?;
    let client = Client::with_options(client_options)
        .map_err(|e| ServerFnError::new(format!("Failed to create MongoDB client: {e}")))?;
    let db = client.database("markdown_db");
    let collection = db.collection::<MarkdownEntry>("markdown_entries");

    let filter=mongodb::bson::doc! {};
    let mut result = collection.find(filter).sort(mongodb::bson::doc! { "_id": -1 }).limit(1).await?;
    let result = result.try_next().await?;
    // Eğer kayıt varsa içeriğini, yoksa varsayılan değeri döndür
    Ok(result.map_or(DEFAULT_MARKDOWN.to_string(), |entry| entry.content))
}
