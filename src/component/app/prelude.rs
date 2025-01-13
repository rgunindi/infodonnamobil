pub use dioxus::{logger::tracing::info, prelude::*};

mod server {
    include!("../../../server/server.rs");
}
pub use server::{get_markdown,watch_markdown};

#[cfg(feature = "server")]
pub use server::MarkdownEntry;

#[cfg(feature = "server")]
pub use tokio::time::Duration;