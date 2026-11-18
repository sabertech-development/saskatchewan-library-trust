use super::*;

pub(super) fn download_pdf(inputs: &Inputs) -> Result<(), String> {
    let payload = crate::report::data(inputs)?;
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::{JsCast, JsValue};
        let window = web_sys::window().ok_or("Browser window is unavailable.")?;
        let function = js_sys::Reflect::get(&window, &JsValue::from_str("downloadTrustReport"))
            .map_err(|_| "The PDF download could not start.")?
            .dyn_into::<js_sys::Function>()
            .map_err(|_| "The PDF tools are still loading. Please try again.")?;
        function
            .call1(&window, &JsValue::from_str(&payload))
            .map_err(|_| "The PDF download could not start.")?;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::{
            io::Write,
            process::{Command, Stdio},
        };
        let mut child = Command::new("node")
            .args(["scripts/export_report.mjs", "-", "library-trust-review.pdf"])
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| format!("PDF export needs Node.js: {e}"))?;
        child
            .stdin
            .take()
            .ok_or("PDF input stream unavailable.")?
            .write_all(payload.as_bytes())
            .map_err(|e| e.to_string())?;
        if !child.wait().map_err(|e| e.to_string())?.success() {
            return Err("PDF export failed. Check the local console.".into());
        }
    }
    Ok(())
}

impl ProposalApp {
    pub fn review_text(&self, p: &Projection) -> String {
        let mut t = narrative_text();
        t.push_str("\nHYPOTHETICAL SCENARIO — NOT SECURED FUNDS OR A FORECAST\nAll inputs are editable in the scenario tool. The default opening balance illustrates the proposed fundraising target as if already raised.\n");
        t.push_str(&serde_json::to_string_pretty(&self.inputs).unwrap());
        t.push_str("\n\nCALCULATION RULES\n");
        for (title, body) in finance::RULES {
            t.push_str(&format!("\n{title}\n{body}\n"));
        }
        t.push_str("\nYEARLY LEDGER — CAD (purchasing power in starting-year dollars)\nYear | Long-term opening | Capital gifts | Gross cash interest | Capital value change | Total return | Costs paid | Net interest | Net interest reinvested | Endowment support | Immediate support | Long-term closing | Purchasing power | Cumulative support | Library need | Remaining gap | Immediate carryover | Unpaid costs\n");
        for y in &p.years {
            t.push_str(&format!(
                "{} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {}\n",
                y.year,
                finance::cad(y.opening),
                finance::cad(y.capital_donations),
                finance::cad(y.interest_income),
                finance::cad(y.valuation_change),
                finance::cad(y.investment_return),
                finance::cad(y.costs_paid),
                finance::cad(y.net_interest),
                finance::cad(y.retained_interest),
                finance::cad(y.endowment_distribution),
                finance::cad(y.immediate_distribution),
                finance::cad(y.closing),
                finance::cad(y.real_closing),
                finance::cad(y.cumulative_support),
                finance::cad(y.library_need),
                finance::cad(y.gap),
                finance::cad(y.immediate_balance),
                finance::cad(y.unpaid_costs)
            ));
        }
        t
    }
}

pub(super) fn download(name: &str, mime: &str, content: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        let parts = js_sys::Array::new();
        parts.push(&wasm_bindgen::JsValue::from_str(content));
        let options = web_sys::BlobPropertyBag::new();
        options.set_type(mime);
        if let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&parts, &options) {
            if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    if let Ok(a) = doc.create_element("a").and_then(|e| {
                        e.dyn_into::<web_sys::HtmlAnchorElement>()
                            .map_err(|e| e.into())
                    }) {
                        a.set_href(&url);
                        a.set_download(name);
                        a.click();
                    }
                }
                let _ = web_sys::Url::revoke_object_url(&url);
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = mime;
        let _ = std::fs::write(name, content);
    }
}
