mod render;
pub mod tera_renderer;

pub use tera_renderer::TEMPLATE_ENGINE;

pub use render::{render, render_with_status, render_html, render_safe};
