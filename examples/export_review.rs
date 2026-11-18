use saskatchewan_library_funding::{
    app::ProposalApp,
    finance::{Inputs, calculate},
};
fn main() {
    let app = ProposalApp::with_context(&eframe::egui::Context::default());
    let cash = calculate(&Inputs::default()).unwrap();
    std::fs::create_dir_all("dist").unwrap();
    std::fs::write("dist/review.txt", app.review_text(&cash)).unwrap();
    std::fs::write(
        "dist/reader.html",
        saskatchewan_library_funding::reader::html(),
    )
    .unwrap();
    std::fs::write(
        "dist/reader.js",
        saskatchewan_library_funding::reader::SCRIPT,
    )
    .unwrap();
    std::fs::create_dir_all("dist/fonts").unwrap();
    std::fs::copy("assets/reader-layout.css", "dist/reader-layout.css").unwrap();
    std::fs::copy("assets/reader-layout.js", "dist/reader-layout.js").unwrap();
    std::fs::copy(
        "assets/fonts/SpaceGrotesk.ttf",
        "dist/fonts/SpaceGrotesk.ttf",
    )
    .unwrap();
    std::fs::copy("assets/fonts/OFL-SpaceGrotesk.txt", "dist/fonts/OFL.txt").unwrap();
    println!("Default plain-text review brief exported.");
}
