use dominator::{html, Dom};

pub fn render() -> Dom {
    html!("div", {
      .class(["flex", "grow", "justify-center", "items-center", "h-screen"])
      .class(["bg-gray-50", "dark:bg-gray-900"])
      .child(
        html!("p", {
          .text("About Page")
          .class(["text-3xl", "font-semibold", "dark:text-white"])
        })
      )
    })
}
