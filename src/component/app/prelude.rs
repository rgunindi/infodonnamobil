pub use dioxus::prelude::*;

mod server {
    include!("../../../server/server.rs");
}
pub use server::{get_markdown, raw_markdown, watch_markdown};

#[cfg(feature = "server")]
pub use server::MarkdownEntry;

#[cfg(feature = "server")]
pub use tokio::time::Duration;
