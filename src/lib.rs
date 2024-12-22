use wasm_bindgen::prelude::*;
use crate::binding::*;
use crate::extension::Extension;

mod utils;
mod extension;
mod binding;

thread_local! {
    static EXTENSION: std::cell::RefCell<Extension> = std::cell::RefCell::new(Extension::new());
}

#[wasm_bindgen]
pub fn activate() {
    log("Hello, demo-wasm-pack!");

    EXTENSION.with(|extension| extension.borrow_mut().init());
}

#[wasm_bindgen]
pub fn deactivate() {
    log("Goodbye, demo-wasm-pack!");

    EXTENSION.with(|extension| {
        let extension = extension.borrow_mut();
        drop(extension);
    });
}
