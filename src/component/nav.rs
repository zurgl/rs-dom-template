use crate::app;
use dominator::{html, link, Dom};
pub fn render() -> Dom {
    html!("div", {
      .class(["hidden","md:block"])
      .child(
        html!("nav", {
          .attr("aria-label", "Site Nav")
          .child(
            html!("ul", {
              .class(["flex","items-center","gap-6","text-xl", "font-bold"])
              .children(app::route::Route::nav_iter().map(|(text, route)| {
                html!("li", {
                  .child(link!(route.to_url(), {
                    .class(["text-black","transition","hover:text-gray-500/75", "dark:text-white","hover:text-gray-50/75"])
                    .text(text)
                  }))
                })
              }))
            })
          )
        })
      )
    })
}
