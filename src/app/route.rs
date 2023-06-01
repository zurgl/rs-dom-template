use dominator::routing;
use web_sys::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    About,
    Home,
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

    #[allow(clippy::wrong_self_convention, dead_code)]
    pub fn to_url(&self) -> &'static str {
        match self {
            Route::Home => "#/",
            Route::About => "#/about",
            Route::Contact => "#/contact",
        }
    }

    pub fn nav_iter() -> std::vec::IntoIter<(&'static str, Route)> {
        vec![("About", Route::About), ("Contact", Route::Contact)].into_iter()
    }
}

impl Default for Route {
    fn default() -> Self {
        Self::from_url(&routing::url().lock_ref())
    }
}
