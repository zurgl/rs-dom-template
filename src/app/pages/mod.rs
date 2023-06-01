mod about;
mod contact;
mod home;

use crate::app::{route::Route, App};
use dominator::{html, Dom};
use futures_signals::signal::SignalExt;
use std::sync::Arc;

pub fn render(app: Arc<App>) -> Dom {
    html!("div", {
      .class(["bg-gray-50", "dark:bg-gray-900"])
      .children(& mut[
        html!("div", {
          .visible_signal(app.route.signal().map(|route| route == Route::Home))
          .child(home::render())
        }),
        html!("div", {
          .visible_signal(app.route.signal().map(|route| route == Route::About))
          .child(about::render())
        }),
        html!("div", {
          .visible_signal(app.route.signal().map(|route| route == Route::Contact))
          .child(contact::render())
        }),
      ])
    })
}
