use dominator::{html, link, Dom};

use crate::app::route::Route;

pub fn render(route: Route) -> Dom {
    html!("li", {
      .child(link!(route.to_string(), {
        .class(["text-black","transition","hover:text-gray-500/75", "dark:text-white","hover:text-gray-50/75"])
        .text(&route.as_text())
      }))
    })
}
