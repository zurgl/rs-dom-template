use crate::{
    app::{self, route::Route},
    component::nav_link,
};
use dominator::{html, Dom};

pub fn render() -> Dom {
    html!("div", {
      .class(["hidden","md:block"])
      .child(
        html!("nav", {
          .attr("aria-label", "Site Nav")
          .child(
            html!("ul", {
              .class(["flex","items-center","gap-6","text-xl", "font-bold"])
              .children(app::route::iter().filter(Route::not_home).map(|route| {
                nav_link::render(route, route.as_ref())
              }))
            })
          )
        })
      )
    })
}
