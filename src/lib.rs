pub mod allocation;
pub mod app;
pub mod content;
pub mod finance;
pub mod reader;

pub fn project_json(input: &str) -> Result<String, String> {
    let inputs: finance::Inputs =
        serde_json::from_str(input).map_err(|e| format!("Invalid scenario inputs: {e}"))?;
    serde_json::to_string(&finance::calculate(&inputs)?).map_err(|e| e.to_string())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn project_scenario(input: &str) -> Result<String, JsValue> {
    project_json(input).map_err(|e| JsValue::from_str(&e))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn scenario_inputs(index: usize) -> String {
    serde_json::to_string(&finance::scenario(index)).unwrap()
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    // The semantic HTML companion uses the same engine without creating a canvas.
    if web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("proposal"))
        .is_none()
    {
        return;
    }
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
