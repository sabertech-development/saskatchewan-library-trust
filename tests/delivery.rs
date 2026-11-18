use saskatchewan_library_funding::{content, finance, project_json, reader};
#[test]
fn semantic_companion_exposes_questions_controls_and_ledger() {
    let h = reader::html();
    for item in [
        "<main",
        "Skip to main content",
        "<fieldset>",
        "<legend>",
        "role=\"status\"",
        "role=\"alert\"",
        "<caption>",
        "first_year_return",
        "library_interest_share",
        "interest_only",
        "protect_principal",
        "No donations collected",
    ] {
        assert!(h.contains(item), "Missing {item}");
    }
    for (q, _, _) in content::QUESTIONS {
        assert!(h.contains(q));
    }
    assert!(h.contains("CA$250 million"));
    assert!(!h.to_lowercase().contains("private equity"));
}
#[test]
fn both_interfaces_use_the_same_projection_engine() {
    let i = finance::Inputs::default();
    let serialized = serde_json::to_string(&i).unwrap();
    let from_api: finance::Projection =
        serde_json::from_str(&project_json(&serialized).unwrap()).unwrap();
    let direct = finance::calculate(&i).unwrap();
    assert!((from_api.last().closing - direct.last().closing).abs() < 0.00001);
    assert!((from_api.last().retained_interest - direct.last().retained_interest).abs() < 0.00001);
    assert!(project_json("{bad}").is_err());
    assert!(project_json("{\"unrecognized_input\":2}").is_err());
}
#[test]
fn scenario_form_has_no_submission_or_collection_endpoint() {
    let h = reader::html();
    assert!(!h.contains("method=\"post\""));
    assert!(!h.contains("action=\"http"));
    assert!(reader::SCRIPT.contains("event.preventDefault()"));
    assert!(!reader::SCRIPT.contains("fetch("));
    assert!(!reader::SCRIPT.contains("localStorage"));
}
