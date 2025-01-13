#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
mod component;

use component::app::App;

fn main() {
    // Ana uygulama başlatılıyor
    dioxus::launch(App);

}

