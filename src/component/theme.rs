use std::sync::Arc;

use crate::app::App;
use dominator::{clone, events, html, Dom};

#[derive(Debug, Clone, Copy, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

pub fn render(app: Arc<App>) -> Dom {
    html!("div", {
      .class(["flex","items-center","gap-4"])
      .children([
        html!("div", {
          .class(["sm:flex","sm:gap-4"])
          .children([
            html!("button", {
            .class(["bg-red-600", "inline-block","px-4","py-2","text-sm","font-medium","text-white","dark:text-white","focus:relative", "dark:bg-green-500"])
            .text("Toggle Theme")
            .event(clone!(app => move |_: events::Click| {
              app.theme.replace_with(|theme| {
                  match theme {
                      Theme::Light => Theme::Dark,
                      Theme::Dark => Theme::Light,
                  }
              });
          }))})
          ])
        }),
      ])
    })
}
