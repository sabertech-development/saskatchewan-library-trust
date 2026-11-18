pub mod allocation;
pub mod app;
pub mod content;
pub mod finance;
pub mod reader;
pub mod report;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn report_scenario(input: &str) -> Result<String, JsValue> {
    report::from_json(input).map_err(|e| JsValue::from_str(&e))
}

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
}

// Return the renderer's startup promise to the loader so errors reach the UI.
// The semantic reader initializes only the calculation engine.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start_visual() -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|w| w.document())
        .ok_or_else(|| JsValue::from_str("Browser document unavailable"))?;
    let canvas = document
        .get_element_by_id("proposal")
        .ok_or_else(|| JsValue::from_str("Presentation canvas unavailable"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let runner = eframe::WebRunner::new();
    runner
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(|cc| Ok(Box::new(app::ProposalApp::new(cc)))),
        )
        .await?;
    std::mem::forget(runner);
    Ok(())
}
