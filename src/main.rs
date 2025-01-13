#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
mod component;
fn main() {
    // Ana uygulama başlatılıyor
    dioxus::launch(component::App);

}

