use crate::component::app::prelude::*;

use dioxus::logger::tracing::info;
use futures_util::stream::FusedStream;

use dioxus_sdk::utils::timing::use_interval;

use std::time::Duration;

#[component]
pub fn App() -> Element {
    if cfg!(target_os = "ios") {
        println!("this is ios");
        info!("this is ios");
    }
    if cfg!(target_os = "android") {
        println!("this is android");
        info!("this is android");
    }

    let markdown_content = use_signal(String::new);
    info!("App component mounted");
    use_coroutine(move |rx: UnboundedReceiver<()>| {
        to_owned![markdown_content];
        async move {
            use_interval(Duration::from_secs(60), move || {
                info!("FROM useCoroutine");
                // println!("FROM useCoroutine");

                if rx.is_terminated() {
                    return; // Component unmount edildiğinde döngüyü kır
                }
                spawn(async move {
                    match raw_markdown().await {
                        Ok(entry) => {
                            markdown_content.set(entry.content);
                        }
                        Err(err) => {
                            info!("Detailed connection error: {:?}", err);
                            info!("Error type: {}", err.to_string());
                            markdown_content.set("# Error\nFailed to load markdown.".to_string());
                        }
                    }
                });
            });
        }
    });

    rsx! {
        MarkdownPreview { content: markdown_content }
    }
}

#[component]
fn get_markdownto(c: Signal<String>) -> Element {
    use_future(move || async move {
        to_owned![c];
        let api_url = "https://backoffice.koyeb.app/api/raw_markdown";
        let client = reqwest::Client::new();

        match client
            .request(reqwest::Method::POST, api_url)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .send()
            .await
        {
            Ok(response) => {
                info!("Response status: {}", response.status());
                match response.text().await {
                    Ok(text) => {
                        info!("Raw response text: {}", text);
                        if text.is_empty() {
                            eprintln!("Empty response received");
                            return;
                        }
                        // Manuel JSON parse
                        match serde_json::from_str::<serde_json::Value>(&text) {
                            Ok(json) => {
                                info!("Parsed JSON: {:?}", json);
                                if let Some(content) = json.get("content").and_then(|v| v.as_str())
                                {
                                    info!("Raw markdown content received");
                                    c.set(content.to_string());
                                } else {
                                    eprintln!("JSON does not contain 'content' field");
                                    info!("JSON structure: {:?}", json);
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to parse JSON: {}", e);
                                info!("Failed text: {}", text);
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to get response text: {}", e),
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    });
    rsx! {
        MarkdownPreview { content: c }
    }
}
#[component]
fn MarkdownPreview(content: Signal<String>) -> Element {
    rsx! {
        head {
            // style { "{include_str!(\"../../../assets/style.css\")}" }
            style { "{include_str!(\"./style.css\")}" }
        }
        div { class: "container",
            div {
                class: "markdown-preview",
                dangerous_inner_html: if content().is_empty() { String::from("<p>Loading...</p>") } else { markdown::to_html(&content()) },
            }
        }
    }
}
