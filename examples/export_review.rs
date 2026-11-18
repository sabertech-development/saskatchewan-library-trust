use saskatchewan_library_funding::{
    app::ProposalApp,
    finance::{Inputs, calculate},
};
fn main() {
    let app = ProposalApp::with_context(&eframe::egui::Context::default());
    let cash = calculate(&Inputs::default()).unwrap();
    std::fs::create_dir_all("dist").unwrap();
    std::fs::write("dist/review.txt", app.review_text(&cash)).unwrap();
    println!("Default plain-text review brief exported.");
}
