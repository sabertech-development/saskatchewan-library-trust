use crate::{
    allocation,
    content::*,
    finance::{self, Inputs, Projection},
};
use eframe::egui::*;

mod atlas;
mod scenario;
#[cfg(test)]
mod tests;
mod views;
mod workspace;
pub(super) const BG: Color32 = Color32::from_rgb(3, 5, 9);
pub(super) const PANEL: Color32 = Color32::from_rgb(8, 17, 29);
pub(super) const LINE: Color32 = Color32::from_rgb(28, 52, 71);
pub(super) const INK: Color32 = Color32::from_rgb(232, 242, 250);
pub(super) const MUTED: Color32 = Color32::from_rgb(145, 169, 188);
pub(super) const TEAL: Color32 = Color32::from_rgb(61, 202, 255);
pub(super) const GOLD: Color32 = Color32::from_rgb(180, 237, 255);
pub(super) const RED: Color32 = Color32::from_rgb(247, 156, 142);

pub struct ProposalApp {
    pub(super) route: usize,
    pub(super) inputs: Inputs,
    pub(super) preset: usize,
    pub(super) node: usize,
    pub(super) overlay: usize,
    pub(super) role: usize,
    pub(super) baseline: f64,
    pub(super) weights: [f64; 4],
    pub(super) search: String,
    pub(super) tour: Option<usize>,
    pub(super) mobile_menu: bool,
    pub(super) toast: String,
    pub(super) selected_year: usize,
    pub(super) inspector_tab: usize,
    pub(super) map_zoom: f32,
    pub(super) review_tabs: [usize; 10],
    pub(super) funding_tab: usize,
    pub(super) input_tab: usize,
    pub(super) result_tab: usize,
    pub(super) record: usize,
    pub(super) rule: usize,
}
impl ProposalApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::with_context(&cc.egui_ctx)
    }
    /// Shared initialization for the browser and deterministic offscreen QA.
    pub fn with_context(ctx: &Context) -> Self {
        // The presentation has a deliberate dark identity, independent of OS theme.
        ctx.set_theme(ThemePreference::Dark);
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Space Grotesk".into(),
            FontData::from_static(include_bytes!("../assets/fonts/SpaceGrotesk.ttf")).into(),
        );
        fonts.font_data.insert(
            "Space Grotesk Medium".into(),
            FontData::from_static(include_bytes!("../assets/fonts/SpaceGrotesk-Medium.ttf")).into(),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Space Grotesk".into());
        fonts.families.insert(
            FontFamily::Name("Display".into()),
            vec!["Space Grotesk Medium".into(), "Space Grotesk".into()],
        );
        ctx.set_fonts(fonts);
        let mut style = (*ctx.style()).clone();
        style.visuals = Visuals::dark();
        style.visuals.panel_fill = BG;
        style.visuals.window_fill = PANEL;
        style.visuals.extreme_bg_color = Color32::from_rgb(5, 12, 20);
        style.visuals.override_text_color = Some(INK);
        style.visuals.hyperlink_color = TEAL;
        style.visuals.selection.bg_fill = Color32::from_rgb(12, 42, 63);
        style.visuals.selection.stroke = Stroke::new(1.5, TEAL);
        style.visuals.widgets.inactive.bg_fill = PANEL;
        style.visuals.widgets.inactive.weak_bg_fill = PANEL;
        style.visuals.widgets.inactive.bg_stroke = Stroke::new(1., LINE);
        style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(12, 42, 63);
        style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(12, 42, 63);
        style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, TEAL);
        style.visuals.widgets.active.bg_stroke = Stroke::new(2., TEAL);
        style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1., LINE);
        style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1., INK);
        style.visuals.widgets.noninteractive.bg_fill = PANEL;
        style.visuals.widgets.active.bg_fill = Color32::from_rgb(16, 53, 74);
        style.visuals.widgets.active.weak_bg_fill = Color32::from_rgb(16, 53, 74);
        style.visuals.widgets.open.bg_fill = PANEL;
        style.visuals.widgets.open.weak_bg_fill = PANEL;
        for widget in [
            &mut style.visuals.widgets.inactive,
            &mut style.visuals.widgets.hovered,
            &mut style.visuals.widgets.active,
            &mut style.visuals.widgets.open,
            &mut style.visuals.widgets.noninteractive,
        ] {
            widget.corner_radius = CornerRadius::ZERO;
            widget.fg_stroke = Stroke::new(1., INK);
        }
        style.visuals.window_corner_radius = CornerRadius::ZERO;
        style
            .text_styles
            .insert(TextStyle::Body, FontId::proportional(16.));
        style
            .text_styles
            .insert(TextStyle::Button, FontId::proportional(15.));
        style
            .text_styles
            .insert(TextStyle::Small, FontId::proportional(13.));
        style
            .text_styles
            .insert(TextStyle::Heading, FontId::proportional(25.));
        style.spacing.item_spacing = vec2(12., 10.);
        style.spacing.button_padding = vec2(14., 10.);
        style.spacing.slider_width = 180.;
        style.animation_time = 0.; // Static transitions respect reduced-motion preferences by default.
        ctx.set_style(style);
        Self {
            route: browser_route().unwrap_or(0),
            inputs: Inputs::default(),
            preset: 0,
            node: 1,
            overlay: 0,
            role: 0,
            baseline: 5_000.,
            weights: [40., 25., 20., 15.],
            search: String::new(),
            tour: None,
            mobile_menu: false,
            toast: String::new(),
            selected_year: 0,
            inspector_tab: 0,
            map_zoom: 1.,
            review_tabs: [0; 10],
            funding_tab: 0,
            input_tab: 0,
            result_tab: 0,
            record: 0,
            rule: 0,
        }
    }
    pub fn navigate(&mut self, route: usize) {
        self.route = route.min(9);
        self.mobile_menu = false;
        self.toast.clear();
        set_route(self.route);
    }
    pub(super) fn start_tour(&mut self) {
        self.tour = Some(0);
        self.navigate(0);
    }
    fn navigation(&mut self, ui: &mut Ui) {
        eyebrow(ui, "THE PROPOSAL");
        ui.add_space(10.);
        for (idx, (_, name)) in ROUTES.iter().enumerate() {
            if idx == 9 {
                ui.add_space(14.);
                ui.separator();
            }
            let selected = self.route == idx;
            let text = RichText::new(*name)
                .size(15.)
                .color(if selected { TEAL } else { MUTED });
            if ui
                .add_sized(
                    [ui.available_width(), 42.],
                    Button::new(text).selected(selected),
                )
                .clicked()
            {
                self.navigate(idx);
            }
        }
        ui.add_space(24.);
        ui.label(
            RichText::new("DONATED CAPITAL\nCOMMUNITY PURPOSE")
                .size(13.)
                .color(GOLD),
        );
        ui.add_space(8.);
        ui.label(
            RichText::new("A proposal for provincial discussion.")
                .size(14.)
                .color(MUTED),
        );
        ui.add_space(16.);
        if ui.button("Take the guided tour").clicked() {
            self.start_tour();
        }
    }
    fn header(&mut self, ui: &mut Ui, compact: bool) {
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = 10.;
            let (r, _) = ui.allocate_exact_size(vec2(34., 40.), Sense::hover());
            draw_book(ui.painter(), r.center(), 24., TEAL);
            ui.allocate_ui_with_layout(
                vec2(if compact { 205. } else { 235. }, 45.),
                Layout::top_down(Align::LEFT),
                |ui| {
                    ui.spacing_mut().item_spacing.y = 4.;
                    ui.label(RichText::new("SASKATCHEWAN").font(display(18.)).color(INK));
                    ui.label(
                        RichText::new("LIBRARY SUSTAINABILITY TRUST")
                            .monospace()
                            .size(10.)
                            .color(MUTED),
                    );
                },
            );
            if compact {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui
                        .button(if self.mobile_menu { "Close" } else { "Menu" })
                        .clicked()
                    {
                        self.mobile_menu = !self.mobile_menu;
                    }
                });
            } else {
                ui.add_space(((ui.available_width() - 790.) * 0.45).max(0.));
                ui.spacing_mut().item_spacing.x = 13.;
                for (i, short) in [
                    "Purpose",
                    "Atlas",
                    "Donations",
                    "Funding",
                    "Scenarios",
                    "Fair funding",
                    "Oversight",
                    "Evidence",
                    "Readiness",
                ]
                .iter()
                .enumerate()
                {
                    let response = ui.add(
                        Button::new(RichText::new(*short).size(13.).color(if self.route == i {
                            INK
                        } else {
                            MUTED
                        }))
                        .frame(false)
                        .min_size(vec2(0., 64.)),
                    );
                    response
                        .widget_info(|| WidgetInfo::labeled(WidgetType::Button, true, ROUTES[i].1));
                    if self.route == i {
                        ui.painter().line_segment(
                            [response.rect.left_bottom(), response.rect.right_bottom()],
                            Stroke::new(2., TEAL),
                        );
                    }
                    if response.clicked() {
                        self.navigate(i);
                    }
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let review = ui.button(RichText::new("Review").monospace().size(12.));
                    review.widget_info(|| {
                        WidgetInfo::labeled(WidgetType::Button, true, "Review Summary")
                    });
                    if review.clicked() {
                        self.navigate(9);
                    }
                    if ui
                        .button(RichText::new("Share").monospace().size(12.))
                        .clicked()
                    {
                        copy_link(ui.ctx());
                        self.toast =
                            "Section link copied. Custom model inputs are not included.".into();
                    }
                });
            }
        });
    }
}

impl ProposalApp {
    pub fn render(&mut self, ctx: &Context) {
        ctx.data_mut(|d| d.insert_temp(Id::new("panel-overflows"), Vec::<f32>::new()));
        if let Some(route) = browser_route() {
            if route != self.route {
                self.route = route;
            }
        }
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            self.mobile_menu = false;
            self.tour = None;
        }
        let mobile = ctx.content_rect().width() < 820.;
        let compact_header = ctx.content_rect().width() < 1340.;
        TopBottomPanel::top("masthead")
            .exact_height(84.)
            .frame(
                Frame::new()
                    .fill(BG)
                    .inner_margin(Margin::symmetric(if mobile { 16 } else { 40 }, 10)),
            )
            .show(ctx, |ui| {
                self.header(ui, compact_header);
                let r = ui.max_rect();
                ui.painter().line_segment(
                    [
                        r.left_bottom() + vec2(-40., 10.),
                        r.right_bottom() + vec2(40., 10.),
                    ],
                    Stroke::new(1., LINE),
                );
            });
        TopBottomPanel::bottom("status")
            .frame(
                Frame::new()
                    .fill(BG)
                    .inner_margin(Margin::symmetric(if mobile { 16 } else { 40 }, 8)),
            )
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(if mobile {
                            "CONCEPT DEMONSTRATION · CAD"
                        } else {
                            "PROPOSED LIBRARY SUSTAINABILITY TRUST"
                        })
                        .monospace()
                        .size(10.)
                        .color(MUTED),
                    );
                    if !mobile {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui
                                .add(
                                    Button::new(
                                        RichText::new("Accessible reading & calculator").size(12.),
                                    )
                                    .frame(false),
                                )
                                .clicked()
                            {
                                open_reader();
                            }
                        });
                    }
                });
                let r = ui.max_rect();
                ui.painter().line_segment(
                    [
                        r.left_top() + vec2(-40., -12.),
                        r.right_top() + vec2(40., -12.),
                    ],
                    Stroke::new(1., LINE),
                );
            });
        if self.mobile_menu && compact_header {
            TopBottomPanel::top("mobile navigation")
                .exact_height((ctx.content_rect().height() * 0.65).min(560.))
                .resizable(false)
                .frame(Frame::new().fill(PANEL).inner_margin(18))
                .show(ctx, |ui| {
                    ScrollArea::vertical().show(ui, |ui| self.navigation(ui));
                });
        }
        if let Some(step) = self.tour {
            TopBottomPanel::top("guided tour")
                .frame(frame())
                .show(ctx, |ui| {
                    eyebrow(
                        ui,
                        &format!("GUIDED TOUR · {} OF 5 · ABOUT 3 MINUTES", step + 1),
                    );
                    section(ui, TOUR[step].1);
                    paragraph(ui, TOUR[step].2);
                    ui.horizontal_wrapped(|ui| {
                        if ui.add_enabled(step > 0, Button::new("Previous")).clicked() {
                            self.tour = Some(step - 1);
                            self.navigate(TOUR[step - 1].0);
                        }
                        if primary(
                            ui,
                            if step + 1 == TOUR.len() {
                                "Finish in review summary"
                            } else {
                                "Next stop"
                            },
                        ) {
                            if step + 1 == TOUR.len() {
                                self.tour = None;
                                self.navigate(9);
                            } else {
                                self.tour = Some(step + 1);
                                self.navigate(TOUR[step + 1].0);
                            }
                        }
                        if ui.button("Exit tour").clicked() {
                            self.tour = None;
                        }
                    });
                });
        }
        CentralPanel::default()
            .frame(Frame::new().fill(BG).inner_margin(Margin::symmetric(
                if mobile { 18 } else { 40 },
                if mobile { 22 } else { 26 },
            )))
            .show(ctx, |ui| {
                let r = ui.max_rect();
                gradient(ui.painter(), r, BG, Color32::from_rgb(5, 16, 26));
                // Desktop is a bounded workspace. Extra material changes in tabs;
                // only a detail panel may scroll. Small/zoomed screens can reflow.
                if ui.available_width() >= 960. && ui.available_height() >= 470. {
                    self.workspace_page(ui);
                } else {
                    ScrollArea::vertical()
                        .id_salt(("small-page", self.route))
                        .auto_shrink([false, false])
                        .show(ui, |ui| self.workspace_page(ui));
                }
            });
        if !self.toast.is_empty() {
            Area::new(Id::new("notice"))
                .anchor(Align2::RIGHT_BOTTOM, [-30., -70.])
                .show(ctx, |ui| {
                    Frame::new()
                        .fill(PANEL)
                        .stroke(Stroke::new(1., TEAL))
                        .inner_margin(12)
                        .show(ui, |ui| {
                            ui.label(RichText::new(&self.toast).size(13.).color(GOLD));
                            if ui.small_button("Dismiss").clicked() {
                                self.toast.clear();
                            }
                        });
                });
        }
    }
}

impl eframe::App for ProposalApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.render(ctx);
    }
}

pub(super) fn frame() -> Frame {
    Frame::new()
        .fill(PANEL)
        .stroke(Stroke::new(1., LINE))
        .corner_radius(0)
        .inner_margin(Margin::same(22))
}
pub(super) fn display(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name("Display".into()))
}
pub(super) fn gradient(p: &Painter, r: Rect, left: Color32, right: Color32) {
    let mut mesh = epaint::Mesh::default();
    for (pos, color) in [
        (r.left_top(), left),
        (r.right_top(), right),
        (r.right_bottom(), right),
        (r.left_bottom(), left),
    ] {
        mesh.colored_vertex(pos, color);
    }
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    p.add(Shape::mesh(mesh));
}
pub(super) fn eyebrow(ui: &mut Ui, t: &str) {
    ui.label(RichText::new(t).color(TEAL).monospace().size(11.));
}
pub(super) fn small(ui: &mut Ui, t: &str) {
    ui.label(RichText::new(t).color(MUTED).size(14.));
}
pub(super) fn paragraph(ui: &mut Ui, t: &str) {
    ui.label(RichText::new(t).color(MUTED).size(16.));
    ui.add_space(4.);
}
pub(super) fn section(ui: &mut Ui, t: &str) {
    ui.label(RichText::new(t).color(INK).font(display(24.)));
    ui.add_space(5.);
}
pub(super) fn badge(ui: &mut Ui, t: &str, c: Color32) {
    ui.label(RichText::new(t).color(c).monospace().size(11.));
}
pub(super) fn primary(ui: &mut Ui, t: &str) -> bool {
    ui.add(
        Button::new(RichText::new(t).color(BG).monospace().size(12.))
            .fill(TEAL)
            .min_size(vec2(0., 44.)),
    )
    .clicked()
}
fn draw_book(p: &Painter, c: Pos2, s: f32, color: Color32) {
    let points = vec![
        c + vec2(-s * 0.65, -s * 0.32),
        c + vec2(0., -s * 0.12),
        c + vec2(s * 0.65, -s * 0.32),
        c + vec2(s * 0.65, s * 0.38),
        c + vec2(0., s * 0.58),
        c + vec2(-s * 0.65, s * 0.38),
    ];
    p.add(Shape::convex_polygon(
        points,
        PANEL,
        Stroke::new(1.7, color),
    ));
    p.line_segment(
        [c + vec2(0., -s * 0.12), c + vec2(0., s * 0.58)],
        Stroke::new(1.5, color),
    );
    for k in 0..3 {
        let y = -s * 0.05 + k as f32 * s * 0.15;
        p.line_segment(
            [c + vec2(-s * 0.48, y), c + vec2(-s * 0.15, y + s * 0.1)],
            Stroke::new(1., color),
        );
        p.line_segment(
            [c + vec2(s * 0.15, y + s * 0.1), c + vec2(s * 0.48, y)],
            Stroke::new(1., color),
        );
    }
}
fn draw_icon(p: &Painter, c: Pos2, s: f32, kind: usize, color: Color32) {
    match kind {
        0 | 6 => {
            for (x, y) in [(-0.5, 0.1), (0., -0.25), (0.5, 0.1)] {
                let q = c + vec2(x * s, y * s);
                p.circle_stroke(q, 5., Stroke::new(1.5, color));
                p.add(Shape::line(
                    vec![
                        q + vec2(-8., 18.),
                        q + vec2(-7., 9.),
                        q + vec2(7., 9.),
                        q + vec2(8., 18.),
                    ],
                    Stroke::new(1.5, color),
                ));
            }
        }
        5 => draw_book(p, c, s, color),
        3 => {
            for y in [-9., 0., 9.] {
                p.rect_stroke(
                    Rect::from_center_size(c + vec2(0., y), vec2(33., 12.)),
                    5.,
                    Stroke::new(1.5, color),
                    StrokeKind::Inside,
                );
            }
        }
        4 => {
            p.rect_stroke(
                Rect::from_center_size(c, vec2(33., 36.)),
                4.,
                Stroke::new(1.5, color),
                StrokeKind::Inside,
            );
            for y in [-10., 0., 10.] {
                p.circle_filled(c + vec2(-9., y), 2., color);
                p.line_segment(
                    [c + vec2(-2., y), c + vec2(11., y)],
                    Stroke::new(1.3, color),
                );
            }
        }
        _ => {
            let top = c + vec2(0., -19.);
            let l = c + vec2(-24., -7.);
            let r = c + vec2(24., -7.);
            let mid = c + vec2(0., 7.);
            let down = vec2(0., 23.);
            p.add(Shape::convex_polygon(
                vec![top, r, mid, l],
                Color32::from_rgb(33, 65, 69),
                Stroke::new(1.4, color),
            ));
            p.add(Shape::convex_polygon(
                vec![l, mid, mid + down, l + down],
                PANEL,
                Stroke::new(1.4, color),
            ));
            p.add(Shape::convex_polygon(
                vec![mid, r, r + down, mid + down],
                Color32::from_rgb(18, 37, 48),
                Stroke::new(1.4, color),
            ));
            for k in 0..3 {
                let q = l + vec2(6. + k as f32 * 6., 10. + k as f32 * 3.);
                p.line_segment([q, q + vec2(0., 9.)], Stroke::new(2., color));
                let q = r + vec2(-6. - k as f32 * 6., 10. + k as f32 * 3.);
                p.line_segment([q, q + vec2(0., 9.)], Stroke::new(2., color));
            }
        }
    }
}
fn browser_route() -> Option<usize> {
    #[cfg(target_arch = "wasm32")]
    {
        let hash = web_sys::window()?.location().hash().ok()?;
        let route = hash.trim_start_matches("#/").split('?').next()?;
        let route = match route {
            "investment" => "donations",
            "money" => "spending",
            "lab" => "scenarios",
            other => other,
        };
        ROUTES.iter().position(|(slug, _)| *slug == route)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}
fn set_route(route: usize) {
    #[cfg(target_arch = "wasm32")]
    if let Some(w) = web_sys::window() {
        let _ = w.location().set_hash(&format!("/{}", ROUTES[route].0));
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = route;
}
fn copy_link(ctx: &Context) {
    #[cfg(target_arch = "wasm32")]
    if let Some(w) = web_sys::window() {
        if let Ok(h) = w.location().href() {
            ctx.copy_text(h);
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    ctx.copy_text("Native preview. Use the published browser link to share.".into());
}

fn open_reader() {
    #[cfg(target_arch = "wasm32")]
    if let Some(w) = web_sys::window() {
        let _ = w.location().set_href("reader.html");
    }
}
