use dominator::{html, Dom};
use std::sync::Arc;

use crate::app::App;
use crate::logo;

pub fn render(app: Arc<App>) -> Dom {
    html!("footer", {
      .attr("aria-label", "Site Footer")
      .class(["mx-auto","max-w-screen-xl","px-4","py-8","sm:px-6","lg:px-8"])
      .child(
        html!("div", {
          .class(["sm:flex","sm:items-center","sm:justify-between"])
          .children([
            html!("div", {
              .class(["flex","justify-center","text-red-700","sm:justify-start", "dark:text-white"])
              .child(logo::brand_with_text::render(app))
            }),
            html!("p", {
              .class(["dark:text-white", "mt-4","text-center","text-sm","text-black","lg:mt-0","lg:text-right"])
              .text("Copyright © 2022. All rights reserved.")
            }),
          ])
        })
      )
    })
}
