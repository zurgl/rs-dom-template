use dominator::{html, Dom};

pub fn render() -> Dom {
    html!("section", {
      .child(
        html!("div", {
          .class(["mx-auto","max-w-screen-xl","px-4","py-32","lg:flex","lg:h-screen","lg:items-center"])
          .child(
            html!("div", {
              .class(["mx-auto","max-w-xl","text-center"])
              .children([
                html!("h1", {
                  .class(["text-3xl","font-extrabold","sm:text-5xl", "dark:text-white"])
                  .text("Understand User Flow. ")
                  .child(
                    html!("strong", {
                      .class(["font-extrabold","text-red-700", "dark:text-green-500","sm:block"])
                      .text("Increase Transmission.")
                    })
                  )
                }),
                html!("p", {
                  .class(["mt-4","sm:text-xl/relaxed", "dark:text-white"])
                  .text("Lorem ipsum dolor sit amet consectetur, adipisicing elit. Nesciunt illo tenetur fuga ducimus numquam ea!")
                }),
                html!("div", {
                  .class(["mt-8","flex","flex-wrap","justify-center","gap-4"])
                  .children([
                    html!("a", {
                      .class(["block","w-full","rounded","bg-red-600","px-12","py-3","text-sm", "dark:bg-green-500", "font-medium","text-white","shadow","active:bg-red-500","sm:w-auto"])
                      .attr("href", "/get-started")
                      .text("Get Started ")
                    }),
                    html!("a", {
                      .class(["block","w-full","rounded","px-12","py-3","text-sm","font-medium","text-red-600","shadow","sm:w-auto", "dark:bg-white", "dark:text-green-500"])
                      .attr("href", "/about")
                      .text("Learn More")
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
