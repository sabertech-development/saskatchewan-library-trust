pub mod allocation;
pub mod app;
pub mod content;
pub mod finance;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        let window = web_sys::window().expect("browser window");
        let document = window.document().expect("browser document");
        let canvas = document
            .get_element_by_id("proposal")
            .expect("generated canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("canvas element");
        let runner = eframe::WebRunner::new();
        runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(app::ProposalApp::new(cc)))),
            )
            .await
            .expect("start presentation");
        std::mem::forget(runner);
    });
}
