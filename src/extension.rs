use wasm_bindgen::closure::Closure;
use crate::binding::{registerCommand, showInformationMessage, Disposable};

pub struct Extension {
    // learn more about Closure
    // https://rustwasm.github.io/wasm-bindgen/reference/passing-rust-closures-to-js.html
    closures: Vec<Closure<dyn FnMut()>>,
    // Disposable is a JS type imported
    // https://podman-desktop.io/api/classes/Disposable
    disposables: Vec<Disposable>
}

impl Extension {
    pub(crate) fn new () -> Extension {
        Extension {
            closures: vec![],
            disposables: vec![],
        }
    }

    pub fn init(&mut self) {
        // create a closure for the wasm.hello command
        let closure = Closure::wrap(Box::new(|| {
            showInformationMessage("Hello world! Called from rust compiled into wasm");
        }) as Box<dyn FnMut()>);

        // Register the command
        self.disposables.push(
            registerCommand("wasm.hello", &closure),
        );

        // let's keep the closure alive
        // learn more https://rustwasm.github.io/wasm-bindgen/reference/passing-rust-closures-to-js.html#heap-allocated-closures
        self.closures.push(closure);
    }
}

impl Drop for Extension {
    fn drop (&mut self) {
        self.closures.clear();
        // dispose each disposable
        self.disposables.iter().for_each(|disposable| {disposable.dispose()});
        self.disposables.clear();
    }
}