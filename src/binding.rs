use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen(module="@podman-desktop/api")]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    pub fn showInformationMessage(s: &str);

    #[wasm_bindgen(js_namespace = commands)]
    pub fn registerCommand(command: &str, closure: &Closure<dyn FnMut()>) -> Disposable;
}

#[wasm_bindgen(module="@podman-desktop/api")]
extern "C" {
    pub type Disposable;

    #[wasm_bindgen(method)]
    pub fn dispose(this: &Disposable);
}

