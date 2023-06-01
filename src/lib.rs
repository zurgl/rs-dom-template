mod app;
mod component;
mod logo;

use app::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() {
    init_logger();
    std::panic::set_hook(Box::new(on_panic));

    let app = App::new();
    dominator::append_dom(&dominator::body(), App::render(app));
}

cfg_if::cfg_if! {
  if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook"))] {
      fn init_logger() {
          wasm_logger::init(wasm_logger::Config::default());
          log::info!("rust logging enabled!!!");
      }
  } else {
      fn init_logger() {
          log::info!("rust logging disabled!");
      }
  }
}

fn on_panic(info: &std::panic::PanicInfo) {
    log::error!("panic: {:?}", info);
    web_sys::window()
        .unwrap()
        .alert_with_message("got a panic!")
        .unwrap();
}
