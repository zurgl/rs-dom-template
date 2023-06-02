use dominator::{svg, Dom};
use std::sync::Arc;

use crate::{app::App, logo::update_fill_color};

#[derive(Debug, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct Letters {
    letters: Vec<String>,
}

impl IntoIterator for Letters {
    type Item = String;
    type IntoIter = <Vec<String> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.letters.into_iter()
    }
}

impl Default for Letters {
    fn default() -> Self {
        let letters = include!("letter.in");
        Letters { letters }
    }
}

impl Letters {
    fn render(app: Arc<App>) -> Vec<Dom> {
        let letters = Letters::default();
        letters
            .into_iter()
            .map(|letter| {
                svg!("path", {
                  .attr("d", &letter)
                  .attr_signal("fill", update_fill_color(app.clone()))
                })
            })
            .collect::<Vec<Dom>>()
    }
}

pub fn render(app: Arc<App>) -> Dom {
    svg!("svg", {
        .class("h-8")
        .attr("viewBox", "0 0 118 24")
        .attr("fill", "none")
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .children(Letters::render(app))
    })
}
