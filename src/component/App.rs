
use crate::component::prelude::*;

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
            style { "{include_str!(\"../../assets/style.css\")}" }
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

