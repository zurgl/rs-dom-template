pub mod brand;
pub mod brand_with_text;

use futures_signals::signal::{Signal, SignalExt};
use std::sync::Arc;

use crate::{app::App, component::theme::Theme};

pub fn update_fill_color(app: Arc<App>) -> impl Signal<Item = &'static str> {
    app.theme.signal().map(|theme| match theme {
        Theme::Light => "#b91c1c",
        Theme::Dark => "#22c55e",
    })
}
