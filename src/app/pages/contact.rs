use dominator::{html, Dom};

pub fn render() -> Dom {
    html!("div", {
      .class(["flex", "grow", "justify-center", "items-center", "h-screen"])
      .child(
        html!("p", {
          .text("Contact Page")
          .class(["text-3xl", "font-semibold", "dark:text-white"])
        })
      )
    })
}
