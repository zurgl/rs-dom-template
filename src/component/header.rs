use dominator::{html, Dom};
use std::sync::Arc;

use crate::{app::App, component, logo};

pub fn render(app: Arc<App>) -> Dom {
    html!("header", {
        .attr("aria-label", "Site Header")
        .class(["mx-auto","max-w-screen-xl","px-4","sm:px-6","lg:px-8"])
        .child(
          html!("div", {
            .class(["flex","h-16","items-center","justify-between"])
            .children([
              logo::brand::render(app.clone()),
              component::nav::render(),
              component::theme::render(app),
            ])
          })
        )
    })
}
