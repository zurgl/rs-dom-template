use dominator::{clone, events, html, svg, Dom};
use web_sys::{window, HtmlElement};

pub fn body() -> HtmlElement {
    window().unwrap().document().unwrap().body().unwrap()
}

pub fn render() -> Dom {
    #[allow(clippy::let_unit_value)]
    let _app = ();
    html!("header", {
        .attr("aria-label", "Site Header")
        .class(["bg-gray-200", "dark:bg-black"])
        .child(
            html!("div", {
                .class(["mx-auto","max-w-screen-xl","px-4","sm:px-6","lg:px-8"])
                .child(
                    html!("div", {
                        .class(["flex","h-16","items-center","justify-between"])
                        .children([
                            html!("div", {
                                .class(["md:flex","md:items-center","md:gap-12"])
                                .child(
                                    html!("a", {
                                        .class(["block","text-teal-600", "dark:text-white"])
                                        .attr("href", "/#")
                                        .children([
                                            crate::logo::brand()
                                        ])
                                    })
                                )
                            }),
                            nav::render(),
                            html!("div", {
                                .class(["flex","items-center","gap-4"])
                                .children([
                                    html!("div", {
                                        .class(["sm:flex","sm:gap-4"])
                                        .children([
                                          html!("button", {
                                            .class(["bg-red-600", "inline-block","px-4","py-2","text-sm","font-medium","text-white","dark:text-white","focus:relative"])
                                            .text("Toggle Theme")
                                            .event(clone!(_app => move |_: events::Click| {
                                              let body = body();
                                              body.class_list().toggle("dark").unwrap();
                                            }))
                                          })
                                        ])
                                    }),
                                    html!("div", {
                                        .class(["block","md:hidden"])
                                        .child(
                                            html!("button", {
                                                .class(["rounded","bg-gray-100","p-2","text-gray-600","transition","hover:text-gray-600/75"])
                                                .child(
                                                    svg!("svg", {
                                                        .attr("xmlns", "http://www.w3.org/2000/svg")
                                                        .class(["h-5","w-5"])
                                                        .attr("fill", "none")
                                                        .attr("viewBox", "0 0 24 24")
                                                        .attr("stroke", "currentColor")
                                                        .attr("stroke-width", "2")
                                                        .child(
                                                            svg!("path", {
                                                                .attr("stroke-linecap", "round")
                                                                .attr("stroke-linejoin", "round")
                                                                .attr("d", "M4 6h16M4 12h16M4 18h16")
                                                            })
                                                        )
                                                    })
                                                )
                                            })
                                        )
                                    }),
                                ])
                            }),
                        ])
                    })
                )
            })
        )
    })
}

mod nav {
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
}
