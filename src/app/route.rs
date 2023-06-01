use dominator::routing;
use strum::{EnumIter, IntoEnumIterator};
use strum_macros::{AsRefStr, Display};
use web_sys::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Display, AsRefStr)]
pub enum Route {
    #[strum(to_string = "About")]
    About,
    #[strum(to_string = "Home")]
    Home,
    #[strum(to_string = "Contact")]
    Contact,
}

impl Route {
    pub fn from_url(url: &str) -> Self {
        let url = Url::new(url).unwrap();
        match url.hash().as_str() {
            "#/" => Route::Home,
            "#/about" => Route::About,
            "#/contact" => Route::Contact,
            _ => Route::Home,
        }
    }

    pub fn not_home(&self) -> bool {
        !matches!(self, Route::Home)
    }

    #[allow(clippy::wrong_self_convention, dead_code)]
    pub fn to_url(&self) -> &'static str {
        match self {
            Route::Home => "#/",
            Route::About => "#/about",
            Route::Contact => "#/contact",
        }
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
