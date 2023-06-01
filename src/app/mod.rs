pub mod pages;
pub mod route;

use crate::component;

use dominator::{clone, html, routing, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use route::Route;
use std::sync::Arc;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct App {
    #[serde(skip)]
    pub route: Mutable<Route>,
}

impl App {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            route: Mutable::new(Route::default()),
        })
    }

    pub fn render(app: Arc<Self>) -> Dom {
        html!("div", {

            .future(routing::url()
              .signal_ref(|url| Route::from_url(url))
              .for_each(clone!(app => move |route| {
                  app.route.set_neq(route);
                  async {}
              })))

            .children([
              component::header::render(),
              pages::render(app),
              component::footer::render(),
            ])
        })
    }
}
