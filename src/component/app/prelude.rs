pub use dioxus::prelude::*;

mod server_local {
    include!("../../../server/server.rs");
}
pub use server_local::{get_markdown, raw_markdown, watch_markdown};

#[cfg(feature = "server")]
pub use server_local::MarkdownEntry;

#[cfg(feature = "server")]
pub use tokio::time::Duration;
