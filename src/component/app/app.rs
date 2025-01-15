use std::time::Duration;

use crate::component::app::prelude::*;

use futures_util::stream::FusedStream;

use gloo_timers::callback::{Interval, Timeout};
// #[cfg(frontend)]
use gloo_timers::future::TimeoutFuture;
use log::info;

#[component]
pub fn App() -> Element {
    let markdown_content = use_signal(String::new);
    info!("App component mounted");
    // Normal content loading effect
    use_effect(move || {
        to_owned![markdown_content];
        spawn(async move {
            // TimeoutFuture::new(10000).await;
            // let int = Interval::new(10000, || info!("FROMINTERVAL"));
            // int.forget();
            // int.cancel();
            if let Ok(content) = get_markdown().await {
                markdown_content.set(content);
            } else {
                markdown_content.set("# Error\nFailed to load markdown.".to_string());
            }
        });
    });
    // Kontrollü watch coroutine
    use_coroutine(move |rx: UnboundedReceiver<()>| {
        to_owned![markdown_content];
        async move {
            // #[cfg(feature = "server")]
            // let mut interval = tokio::time::interval(Duration::from_secs(10));

            loop {
                TimeoutFuture::new(10000).await;
                // #[cfg(feature = "server")]
                // interval.tick().await; // 10 saniye bekle
                info!("FROM useCoroutine");
                //#[cfg(feature = "server")]
                if rx.is_terminated() {
                    break; // Component unmount edildiğinde döngüyü kır
                }

                match watch_markdown().await {
                    Ok(content) if content != "No changes detected." => {
                        markdown_content.set(content);
                        println!("Content updated!");
                    }
                    Err(e) => eprintln!("Watch error: {}", e),
                    _ => {} // No changes detected durumu
                }
            }
        }
    });

    rsx! {
        MarkdownPreview{content:markdown_content}
    }
}

#[component]
fn MarkdownPreview(content: Signal<String>) -> Element {
    rsx! {
        head {
            // style { "{include_str!(\"../../../assets/style.css\")}" }
            style { "{include_str!(\"./style.css\")}" }
            Icon{}
        }
        div { class: "container",
            div { class: "markdown-preview",
                dangerous_inner_html: if content().is_empty() {
                    String::from("<p>Loading...</p>")
                } else {
                    markdown::to_html(&content())
                }
            }
        }
    }
}
const FAVICON: Asset = asset!("assets/favicon.ico");
#[component]
fn Icon() -> Element {
    rsx! {
        head{
            document::Link { rel: "icon", href: FAVICON }
            link { rel: "icon", href: "../../../assets/favicon-16x16.png", sizes: "16x16", type: "image/png" }
            link { rel: "shortcut icon", href: "../../../assets/favicon.ico", type: "image/x-icon" }
        }
    }
}
