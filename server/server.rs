use dioxus::prelude::*;

#[cfg(feature = "server")]
use dotenv::dotenv;
#[cfg(feature = "server")]
use futures::{StreamExt, TryStreamExt};
#[cfg(feature = "server")]
use mongodb::{options::ClientOptions, Client};
#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use std::env;

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize, Debug)]
pub struct MarkdownEntry {
    pub content: String,
}

#[cfg(not(feature = "server"))]
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct MarkdownEntry {
    pub content: String,
}

/// Markdown içeriğini sunucuda bir dosyaya kaydeder.
#[server(endpoint = "get_markdown")]
pub async fn get_markdown() -> Result<String, ServerFnError> {
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

    let filter = mongodb::bson::doc! {};
    let mut result = collection
        .find(filter)
        .sort(mongodb::bson::doc! { "_id": -1 })
        .limit(1)
        .await?;
    let result = result.try_next().await?;
    // Eğer kayıt varsa içeriğini, yoksa varsayılan değeri döndür
    Ok(result.map_or(DEFAULT_MARKDOWN.to_string(), |entry| entry.content))
}

#[server(endpoint = "watch_markdown")]
pub async fn watch_markdown() -> Result<String, ServerFnError> {
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

    let mut change_stream = collection
        .watch()
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

#[server(endpoint = "raw_markdown")]
pub async fn raw_markdown() -> Result<MarkdownEntry, ServerFnError> {
    println!("Fetching raw markdown entry...");

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

    let filter = mongodb::bson::doc! {};
    let mut result = collection
        .find(filter)
        .sort(mongodb::bson::doc! { "_id": -1 })
        .limit(1)
        .await?;

    match result.try_next().await? {
        Some(entry) => Ok(entry),
        None => Ok(MarkdownEntry {
            content: "# Welcome\nThis is a default markdown content.".to_string(),
        }),
    }
}
