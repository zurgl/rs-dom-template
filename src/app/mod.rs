pub mod pages;
pub mod route;

use crate::component;

use dominator::{clone, html, routing, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use route::Route;
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlElement};

#[derive(Debug, Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug)]
pub struct App {
    pub route: Mutable<Route>,
    pub theme: Mutable<Theme>,
}

pub fn body() -> HtmlElement {
    window().unwrap().document().unwrap().body().unwrap()
}

impl App {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            route: Mutable::new(Route::default()),
            theme: Mutable::new(Theme::Light),
        })
    }

    pub fn render(app: Arc<Self>) -> Dom {
        let body = body();

        spawn_local(app.theme.signal().for_each(move |theme| {
            match theme {
                Theme::Light => body.class_list().remove_1("dark").unwrap(),
                Theme::Dark => body.class_list().add_1("dark").unwrap(),
            };
            async {}
        }));

        html!("div", {

            .future(routing::url()
              .signal_ref(|url| Route::from_url(url))
              .for_each(clone!(app => move |route| {
                  app.route.set_neq(route);
                  async {}
              })))

            .class(["bg-gray-300", "dark:bg-gray-900"])

            .children([
              component::header::render(app.clone()),
              pages::render(app.clone()),
              component::footer::render(app),
            ])
        })
    }
}
