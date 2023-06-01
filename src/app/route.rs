use dominator::routing;
use std::str::FromStr;
use strum::{EnumIter, EnumString, IntoEnumIterator};
use strum_macros::Display;
use web_sys::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display, EnumString)]
pub enum Route {
    #[strum(serialize = "#/")]
    Home,
    #[strum(serialize = "#/about")]
    About,
    #[strum(serialize = "#/contact")]
    Contact,
}

impl Route {
    pub fn from_url(url: &str) -> Self {
        let url = Url::new(url)
            .map(|url| url.hash())
            .unwrap_or(String::from("#/"));
        Route::from_str(url.as_str()).ok().unwrap_or(Route::Home)
    }

    pub fn not_home(&self) -> bool {
        !matches!(self, Route::Home)
    }

    pub fn as_text(&self) -> String {
        let route = &self.to_string()[2..];
        route[0..1].to_uppercase() + &route[1..]
    }
}

impl Default for Route {
    fn default() -> Self {
        Self::from_url(&routing::url().lock_ref())
    }
}

pub fn iter() -> RouteIter {
    Route::iter()
}
