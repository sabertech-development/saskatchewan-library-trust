fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1440.0, 960.0])
            .with_min_inner_size([390.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        saskatchewan_library_funding::content::TITLE,
        options,
        Box::new(|cc| {
            Ok(Box::new(
                saskatchewan_library_funding::app::ProposalApp::new(cc),
            ))
        }),
    )
}
