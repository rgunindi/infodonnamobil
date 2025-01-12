#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use dioxus::{logger::tracing::info, prelude::*};
#[cfg(feature = "server")]
use dotenv::dotenv;
#[cfg(feature = "server")]
use mongodb::{options::ClientOptions, Client};
#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use std::env;
#[cfg(feature = "server")]
use futures::StreamExt;
#[cfg(feature = "server")]
use tokio::time::Duration;


#[cfg(feature = "server")]
#[derive(Serialize, Deserialize,Debug)]
struct MarkdownEntry {
    content: String,
}

fn main() {
    // Ana uygulama başlatılıyor
    dioxus::launch(App);

}
#[component]
pub fn App() -> Element {
    let markdown_content = use_signal(String::new);
    info!("App component mounted");
    // Normal content loading effect
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

    // 5 saniyede bir kontrol eden coroutine
    use_coroutine(move |_rx: UnboundedReceiver<()>| {
        to_owned![markdown_content];
        async move {
            loop {
                match watch_markdown().await {
                    Ok(content) => {
                        if content != "No changes detected." {
                            markdown_content.set(content);
                            println!("Content updated!");
                        }
                    }
                    Err(e) => eprintln!("Watch error: {}", e),
                }
                
                #[cfg(feature = "server")]
                tokio::time::sleep(Duration::from_secs(10)).await;

                info!("Watching markdown changes...");
            }
        }
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
    println!("Fetching markdown content..."); // Debug log
    
    const DEFAULT_MARKDOWN: &str = "# Welcome\nThis is a default markdown content.";

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

#[server(watch_markdown)]
async fn watch_markdown() -> Result<String, ServerFnError> {
    println!("Watching markdown changes..."); // Debug log
    let client_uri =
        env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let client_options = ClientOptions::parse(client_uri)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse MongoDB URI: {e}")))?;
    let client = Client::with_options(client_options)
        .map_err(|e| ServerFnError::new(format!("Failed to create MongoDB client: {e}")))?;
    let db = client.database("markdown_db");
    let collection = db.collection::<MarkdownEntry>("markdown_entries");
    
    let mut change_stream = collection.watch()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to create change stream: {e}")))?;

    if let Some(change_result) = change_stream.next().await {
        if let Ok(change) = change_result {
            if let Some(doc) = change.full_document {
                println!("New content: {}", doc.content);
                return Ok(doc.content);
            }
        }
    }

    Ok("No changes detected.".to_string())
}
