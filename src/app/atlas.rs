use super::*;

const BLUE_DIM: Color32 = Color32::from_rgb(39, 100, 134);
const DISTRICTS: [(f32, f32); 7] = [
    (100., 100.),
    (300., 285.),
    (100., 445.),
    (300., 505.),
    (495., 100.),
    (495., 320.),
    (495., 520.),
];

impl ProposalApp {
    pub(super) fn atlas(&mut self, ui: &mut Ui) {
        super::workspace::heading(
            ui,
            "THE LIBRARY ECOSYSTEM",
            "Libraries at the centre.",
            "Connect donors, communities, the proposed fund, oversight and library services. All relationships are illustrative; no participation is claimed.",
        );
        let width = ui.available_width();
        let height = (ui.max_rect().bottom().min(ui.clip_rect().bottom()) - ui.cursor().top() - 8.)
            .max(320.);
        if width >= 960. {
            let inspector_width = 342.;
            let gap = 20.;
            let start = ui.cursor().min;
            let map_rect = Rect::from_min_size(start, vec2(width - inspector_width - gap, height));
            let detail_rect = Rect::from_min_size(
                pos2(map_rect.right() + gap, start.y),
                vec2(inspector_width, height),
            );
            ui.scope_builder(
                UiBuilder::new()
                    .max_rect(map_rect)
                    .layout(Layout::top_down(Align::LEFT)),
                |ui| {
                    self.network(ui, height);
                },
            );
            ui.scope_builder(
                UiBuilder::new()
                    .max_rect(detail_rect)
                    .layout(Layout::top_down(Align::LEFT)),
                |ui| {
                    self.inspector(ui, height);
                },
            );
            ui.advance_cursor_after_rect(Rect::from_min_size(start, vec2(width, height)));
        } else {
            self.network(ui, if width < 500. { 470. } else { 600. });
            ui.add_space(18.);
            self.inspector(ui, 640.);
        }
    }

    fn network(&mut self, ui: &mut Ui, height: f32) {
        let outer = frame().inner_margin(18).show(ui, |ui| {
            let content_top = ui.cursor().top();
            ui.set_min_height(height - 38.);
            ui.set_max_height(height - 38.);
            ui.horizontal(|ui| {
                eyebrow(ui, "LIBRARY ATLAS");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new("7 CONNECTED ROLES")
                            .monospace()
                            .size(10.)
                            .color(MUTED),
                    );
                });
            });
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 4.;
                for (i, name) in ["Funding", "Oversight", "Outcomes"].iter().enumerate() {
                    if tab_button(ui, name, self.overlay == i) {
                        self.overlay = i;
                    }
                }
            });
            let map_height = (height - 38. - (ui.cursor().top() - content_top) - 84.).max(180.);
            let (r, _) =
                ui.allocate_exact_size(vec2(ui.available_width(), map_height), Sense::hover());
            let p = ui.painter().with_clip_rect(r);
            let map = World::new(r, self.map_zoom);
            draw_world(&p, &map, Some(self.node));

            let links: &[(usize, usize)] = match self.overlay {
                0 => &[(0, 1), (0, 5), (1, 3), (1, 5), (5, 6)],
                1 => &[(4, 1), (4, 3), (4, 5), (2, 5)],
                _ => &[(2, 5), (1, 5), (5, 6), (6, 2)],
            };
            for &(a, b) in links {
                if a == self.node || b == self.node {
                    let (ax, ay) = DISTRICTS[a];
                    let (bx, by) = DISTRICTS[b];
                    let start = map.at(ax, ay, 4.);
                    let end = map.at(bx, by, 4.);
                    let elbow = map.at(bx, ay, 4.);
                    let color = if self.overlay == 2 { GOLD } else { TEAL };
                    p.line_segment([start, elbow], Stroke::new(5., color.gamma_multiply(0.09)));
                    p.line_segment([elbow, end], Stroke::new(5., color.gamma_multiply(0.09)));
                    dashed(&p, start, elbow, color.gamma_multiply(0.7));
                    dashed(&p, elbow, end, color.gamma_multiply(0.7));
                }
            }
            // Markers stay screen-sized so the controls remain usable at every zoom level.
            for (i, n) in NODES.iter().enumerate() {
                let (x, y) = DISTRICTS[i];
                let z = [78., 155., 67., 54., 81., 90., 72.][i];
                let anchor = map.at(x, y, z);
                let narrow = r.width() < 500.;
                let label = if narrow {
                    [
                        "Corporate donors",
                        "Long-term fund",
                        "Communities",
                        "Future capacity",
                        "Oversight",
                        "Libraries",
                        "Outcomes",
                    ][i]
                } else {
                    n.short
                };
                let galley = p.layout_no_wrap(
                    label.into(),
                    FontId::proportional(if narrow { 11. } else { 13. }),
                    INK,
                );
                let size = galley.size() + vec2(24., 16.);
                let pos = pos2(
                    (anchor.x - size.x * 0.5)
                        .clamp(r.left() + 3., (r.right() - size.x - 3.).max(r.left() + 3.)),
                    anchor.y - size.y,
                );
                let tag = Rect::from_min_size(pos, size);
                let hit = ui.interact(
                    tag.expand2(vec2(3., 5.)),
                    ui.id().with(("node", i)),
                    Sense::click(),
                );
                hit.widget_info(|| {
                    WidgetInfo::labeled(WidgetType::Button, true, format!("Select {}", n.name))
                });
                let active = i == self.node || hit.hovered() || hit.has_focus();
                p.line_segment([anchor, anchor + vec2(0., 18.)], Stroke::new(1., BLUE_DIM));
                p.circle_filled(anchor + vec2(0., 18.), 7., TEAL.gamma_multiply(0.12));
                p.circle_filled(
                    anchor + vec2(0., 18.),
                    2.5,
                    if active { GOLD } else { BLUE_DIM },
                );
                p.rect(
                    tag,
                    0.,
                    if active {
                        Color32::from_rgb(10, 53, 82)
                    } else {
                        Color32::from_rgb(3, 16, 29)
                    },
                    Stroke::new(
                        if hit.has_focus() { 2. } else { 1. },
                        if active { TEAL } else { BLUE_DIM },
                    ),
                    StrokeKind::Inside,
                );
                p.galley(pos + vec2(12., 8.), galley, INK);
                if hit.clicked() {
                    self.node = i;
                }
            }
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new(match self.overlay {
                        0 => "FUNDING RELATIONSHIPS",
                        1 => "PROPOSED OVERSIGHT",
                        _ => "PRACTICAL SERVICE OUTCOMES",
                    })
                    .monospace()
                    .size(10.)
                    .color(TEAL),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui
                        .add_enabled(self.map_zoom < 1.4, Button::new("+"))
                        .on_hover_text("Zoom in")
                        .clicked()
                    {
                        self.map_zoom = (self.map_zoom + 0.15).min(1.4);
                    }
                    if ui.button("Reset").clicked() {
                        self.map_zoom = 1.;
                    }
                    if ui
                        .add_enabled(self.map_zoom > 0.75, Button::new("−"))
                        .on_hover_text("Zoom out")
                        .clicked()
                    {
                        self.map_zoom = (self.map_zoom - 0.15).max(0.75);
                    }
                });
            });
            ui.label(
                RichText::new("Illustrative relationships · Not a geographic map")
                    .size(12.)
                    .color(MUTED),
            );
        });
        panel_corner(ui.painter(), outer.response.rect);
    }

    fn inspector(&mut self, ui: &mut Ui, height: f32) {
        let n = &NODES[self.node];
        let outer = frame().inner_margin(22).show(ui, |ui| {
            ui.set_min_height(height - 46.);
            ui.horizontal(|ui| {
                let (icon, _) = ui.allocate_exact_size(vec2(40., 40.), Sense::hover());
                draw_icon(ui.painter(), icon.center(), 24., n.kind, TEAL);
                ui.vertical(|ui| {
                    ui.label(RichText::new("PROPOSAL COMPONENT").monospace().size(10.).color(MUTED));
                    ui.label(RichText::new("CONNECTED SYSTEM").monospace().size(10.).color(TEAL));
                });
            });
            ui.add_space(12.);
            ui.label(RichText::new(n.name).font(display(27.)).color(INK));
            ui.label(RichText::new(n.purpose).size(14.).color(MUTED));
            ui.add_space(9.);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 3.;
                for (i, title) in ["Overview", "Relationships", "Readiness"].iter().enumerate() {
                    if tab_button(ui, title, self.inspector_tab == i) { self.inspector_tab = i; }
                }
            });
            ui.separator();
            let remaining = (height - 46. - (ui.cursor().top() - outer_top(ui)) - 110.).max(120.);
            ScrollArea::vertical().id_salt("inspector-content").max_height(remaining).min_scrolled_height(remaining).auto_shrink([false, false]).show(ui, |ui| {
                match self.inspector_tab {
                    0 => {
                        detail(ui, "ROLE IN THE PROPOSAL", n.purpose);
                        detail(ui, "RESPONSIBLE PARTY", n.owner);
                        detail(ui, "DEPENDENCIES", n.dependencies);
                    }
                    1 => {
                        detail(ui, "WHAT FLOWS IN", n.inputs);
                        detail(ui, "WHAT FLOWS OUT", n.outputs);
                        detail(ui, "CONNECTED POLICY", match self.overlay {
                            0 => "Corporate donations build the proposed CA$250 million principal fund. The proposed split sends 50% of net interest to libraries and reinvests 50%. Immediate-use gifts remain separate.",
                            1 => "Trustees oversee the fund, professional managers work within a mandate, and library authorities retain ownership, governance and service decisions.",
                            _ => "Support could contribute to opening hours, staffing, collections, digital access, accessibility and programming. Actual needs and results require local evidence."
                        });
                    }
                    _ => {
                        detail(ui, "CURRENT READINESS", n.readiness);
                        detail(ui, "OUTSTANDING QUESTION", n.question);
                        detail(ui, "REQUIRED BEFORE IMPLEMENTATION", n.dependencies);
                    }
                }
                Frame::new().fill(Color32::from_rgb(10, 32, 48)).inner_margin(15).show(ui, |ui| {
                    eyebrow(ui, "FOR DISCUSSION");
                    ui.label(RichText::new(n.readiness).size(14.).color(GOLD));
                    ui.label(RichText::new("Proposed arrangements. No donations, participation or outcomes are secured.").size(13.).color(MUTED));
                });
            });
            ui.add_space(10.);
            if ui.add_sized([ui.available_width(), 44.], Button::new(RichText::new("Explore endowment scenarios").monospace().size(12.))).clicked() { self.navigate(4); }
            if ui.add_sized([ui.available_width(), 30.], Button::new(RichText::new("View evidence").size(13.).color(TEAL)).frame(false)).clicked() { self.navigate(7); }
        });
        panel_corner(ui.painter(), outer.response.rect);
    }
}

fn outer_top(ui: &Ui) -> f32 {
    ui.max_rect().top()
}

fn detail(ui: &mut Ui, title: &str, body: &str) {
    ui.add_space(8.);
    ui.label(RichText::new(title).monospace().size(11.).color(TEAL));
    ui.label(RichText::new(body).size(14.).color(MUTED));
    ui.add_space(10.);
}

pub(super) fn tab_button(ui: &mut Ui, title: &str, selected: bool) -> bool {
    ui.add(
        Button::new(
            RichText::new(title)
                .size(12.)
                .color(if selected { GOLD } else { MUTED }),
        )
        .fill(if selected {
            Color32::from_rgb(12, 42, 63)
        } else {
            Color32::TRANSPARENT
        })
        .stroke(Stroke::new(
            1.,
            if selected {
                BLUE_DIM
            } else {
                Color32::TRANSPARENT
            },
        )),
    )
    .clicked()
}

pub(super) fn panel_corner(p: &Painter, r: Rect) {
    p.line_segment(
        [r.left_top(), r.left_top() + vec2(25., 0.)],
        Stroke::new(1.5, TEAL),
    );
}

pub(super) fn draw_preview(p: &Painter, rect: Rect) {
    draw_world(p, &World::new(rect, 1.), None);
}

struct World {
    origin: Pos2,
    scale: f32,
}
impl World {
    fn new(r: Rect, zoom: f32) -> Self {
        let scale = (r.width() / 960.).min(r.height() / 620.) * zoom;
        Self {
            origin: r.center() + vec2(0., 25. * scale),
            scale,
        }
    }
    fn at(&self, x: f32, y: f32, z: f32) -> Pos2 {
        self.origin + vec2((x - y) * 0.77, (x + y - 600.) * 0.40 - z) * self.scale
    }
    fn poly(&self, p: &Painter, pts: &[(f32, f32, f32)], fill: Color32, stroke: Color32) {
        p.add(Shape::convex_polygon(
            pts.iter().map(|&(x, y, z)| self.at(x, y, z)).collect(),
            fill,
            Stroke::new(0.75, stroke),
        ));
    }
    fn slab(
        &self,
        p: &Painter,
        x: f32,
        y: f32,
        w: f32,
        d: f32,
        z: f32,
        fill: Color32,
        stroke: Color32,
    ) {
        self.poly(
            p,
            &[(x, y, z), (x + w, y, z), (x + w, y + d, z), (x, y + d, z)],
            fill,
            stroke,
        );
    }
    fn block(&self, p: &Painter, x: f32, y: f32, w: f32, d: f32, h: f32) {
        self.poly(
            p,
            &[
                (x, y + d, 3.),
                (x + w, y + d, 3.),
                (x + w, y + d, h),
                (x, y + d, h),
            ],
            Color32::from_rgb(12, 35, 54),
            BLUE_DIM,
        );
        self.poly(
            p,
            &[
                (x + w, y, 3.),
                (x + w, y + d, 3.),
                (x + w, y + d, h),
                (x + w, y, h),
            ],
            Color32::from_rgb(7, 24, 39),
            Color32::from_rgb(32, 73, 100),
        );
        self.slab(
            p,
            x,
            y,
            w,
            d,
            h,
            Color32::from_rgb(23, 58, 80),
            Color32::from_rgb(65, 147, 183),
        );
        for level in (10..h as i32 - 3).step_by(12) {
            p.line_segment(
                [
                    self.at(x + 4., y + d, level as f32),
                    self.at(x + w - 4., y + d, level as f32),
                ],
                Stroke::new(0.8, Color32::from_rgb(31, 111, 151)),
            );
            p.line_segment(
                [
                    self.at(x + w, y + 4., level as f32),
                    self.at(x + w, y + d - 4., level as f32),
                ],
                Stroke::new(0.6, BLUE_DIM),
            );
        }
        for col in (10..w as i32 - 3).step_by(12) {
            p.line_segment(
                [
                    self.at(x + col as f32, y + d, 6.),
                    self.at(x + col as f32, y + d, h - 4.),
                ],
                Stroke::new(0.6, BLUE_DIM),
            );
        }
        p.line_segment(
            [self.at(x, y, h), self.at(x + w, y, h)],
            Stroke::new(1., TEAL.gamma_multiply(0.7)),
        );
    }
    fn tree(&self, p: &Painter, x: f32, y: f32) {
        p.line_segment(
            [self.at(x, y, 2.), self.at(x, y, 21.)],
            Stroke::new(1., BLUE_DIM),
        );
        let q = self.at(x, y, 25.);
        p.add(Shape::convex_polygon(
            vec![
                q + vec2(0., -11.) * self.scale,
                q + vec2(7., 4.) * self.scale,
                q + vec2(0., 9.) * self.scale,
                q + vec2(-7., 4.) * self.scale,
            ],
            Color32::from_rgb(13, 51, 60),
            Stroke::new(0.7, Color32::from_rgb(34, 103, 119)),
        ));
    }
}

// Original architectural drawing for this proposal. Every building is a Rust mesh;
// it contains no copied reference illustration or geographic/participation claim.
fn draw_world(p: &Painter, m: &World, selected: Option<usize>) {
    for q in (-100..=700).step_by(40) {
        p.line_segment(
            [m.at(q as f32, -70., -20.), m.at(q as f32, 670., -20.)],
            Stroke::new(0.5, Color32::from_rgb(13, 36, 53)),
        );
        p.line_segment(
            [m.at(-70., q as f32, -20.), m.at(670., q as f32, -20.)],
            Stroke::new(0.5, Color32::from_rgb(13, 36, 53)),
        );
    }
    m.poly(
        p,
        &[
            (-12., 612., 0.),
            (612., 612., 0.),
            (612., 612., -19.),
            (-12., 612., -19.),
        ],
        Color32::from_rgb(7, 21, 36),
        LINE,
    );
    m.poly(
        p,
        &[
            (612., -12., 0.),
            (612., 612., 0.),
            (612., 612., -19.),
            (612., -12., -19.),
        ],
        Color32::from_rgb(4, 14, 27),
        LINE,
    );
    m.slab(
        p,
        -12.,
        -12.,
        624.,
        624.,
        0.,
        Color32::from_rgb(7, 20, 32),
        BLUE_DIM,
    );
    for x in [3., 220., 423.] {
        for y in [3., 220., 423.] {
            m.slab(
                p,
                x,
                y,
                174.,
                174.,
                2.,
                Color32::from_rgb(10, 31, 45),
                Color32::from_rgb(24, 63, 83),
            );
        }
    }
    for q in [196., 400.] {
        for t in (0..600).step_by(24) {
            p.line_segment(
                [m.at(q, t as f32, 1.), m.at(q, t as f32 + 11., 1.)],
                Stroke::new(1., BLUE_DIM),
            );
            p.line_segment(
                [m.at(t as f32, q, 1.), m.at(t as f32 + 11., q, 1.)],
                Stroke::new(1., BLUE_DIM),
            );
        }
    }
    // Donor and community meeting spaces around an open courtyard.
    m.block(p, 27., 24., 55., 56., 58.);
    m.block(p, 104., 30., 50., 60., 78.);
    m.block(p, 40., 118., 112., 27., 24.);
    // Rear precinct: books, learning, and independent review.
    m.block(p, 249., 32., 95., 72., 34.);
    for x in [259., 281., 303., 325.] {
        m.slab(
            p,
            x,
            38.,
            12.,
            60.,
            35.,
            Color32::from_rgb(28, 82, 111),
            BLUE_DIM,
        );
    }
    for y in [128., 151.] {
        for x in [250., 276., 302., 328.] {
            m.tree(p, x, y);
        }
    }
    m.block(p, 449., 31., 95., 70., 54.);
    m.block(p, 465., 113., 68., 34., 29.);
    for x in [455., 477., 499., 521.] {
        m.block(p, x, 96., 7., 24., 45.);
    }
    // Community learning spaces and local library branches.
    m.block(p, 24., 244., 117., 65., 32.);
    for x in [31., 65., 99.] {
        m.block(p, x, 250., 23., 51., 41.);
    }
    m.block(p, 36., 340., 38., 35., 24.);
    m.block(p, 96., 333., 49., 41., 32.);
    // The shared fund: a civic-scale stewardship centre.
    m.block(p, 240., 235., 47., 61., 93.);
    m.block(p, 298., 250., 58., 72., 136.);
    m.block(p, 257., 328., 91., 40., 53.);
    let peak = m.at(327., 285., 158.);
    p.line_segment([m.at(327., 285., 136.), peak], Stroke::new(1.2, TEAL));
    p.circle_filled(peak, 6. * m.scale, TEAL.gamma_multiply(0.14));
    p.circle_filled(peak, 2. * m.scale, GOLD);
    // Public library: independent civic architecture with an open-book roof.
    m.block(p, 444., 242., 111., 90., 50.);
    m.poly(
        p,
        &[
            (440., 238., 55.),
            (495., 250., 70.),
            (495., 340., 70.),
            (440., 332., 55.),
        ],
        Color32::from_rgb(28, 79, 104),
        TEAL.gamma_multiply(0.8),
    );
    m.poly(
        p,
        &[
            (495., 250., 70.),
            (561., 238., 55.),
            (561., 332., 55.),
            (495., 340., 70.),
        ],
        Color32::from_rgb(21, 62, 88),
        TEAL.gamma_multiply(0.8),
    );
    for x in [450., 470., 490., 510., 530., 550.] {
        p.line_segment(
            [m.at(x, 333., 5.), m.at(x, 333., 45.)],
            Stroke::new(2. * m.scale, BLUE_DIM),
        );
    }
    for y in [348., 357., 366.] {
        m.slab(
            p,
            447.,
            y,
            107.,
            6.,
            4.,
            Color32::from_rgb(20, 49, 70),
            BLUE_DIM,
        );
    }
    // Collections, reading spaces and shared library services.
    m.block(p, 25., 441., 112., 70., 38.);
    m.block(p, 31., 447., 32., 58., 46.);
    m.block(p, 72., 447., 55., 58., 51.);
    for x in [36., 78., 120.] {
        m.block(p, x, 536., 27., 30., 18.);
    }
    // Reserve: three secure, stepped cash stores.
    for (x, y, h) in [(246., 454., 31.), (286., 478., 42.), (326., 503., 53.)] {
        m.block(p, x, y, 38., 55., h);
        m.slab(
            p,
            x + 6.,
            y + 6.,
            26.,
            43.,
            h + 1.,
            Color32::from_rgb(28, 88, 121),
            BLUE_DIM,
        );
    }
    // Communities: several small neighborhoods and shared green spaces.
    for (x, y, h) in [
        (439., 442., 44.),
        (488., 446., 59.),
        (542., 447., 34.),
        (444., 526., 38.),
        (502., 528., 47.),
    ] {
        m.block(p, x, y, 32., 39., h);
        m.slab(
            p,
            x + 4.,
            y + 4.,
            24.,
            31.,
            h + 1.,
            Color32::from_rgb(28, 77, 96),
            BLUE_DIM,
        );
    }
    for (x, y) in [
        (15., 173.),
        (167., 105.),
        (172., 350.),
        (216., 175.),
        (380., 250.),
        (380., 366.),
        (425., 380.),
        (583., 267.),
        (175., 550.),
        (385., 533.),
        (578., 540.),
        (477., 580.),
    ] {
        m.tree(p, x, y);
    }
    if let Some(i) = selected {
        let (x, y) = DISTRICTS[i];
        let c = m.at(x, y, 3.);
        for radius in [17., 23.] {
            p.circle_stroke(
                c,
                radius * m.scale,
                Stroke::new(1., TEAL.gamma_multiply(0.4)),
            );
        }
    }
}

fn dashed(p: &Painter, a: Pos2, b: Pos2, color: Color32) {
    let length = a.distance(b);
    if length < 1. {
        return;
    }
    let direction = (b - a) / length;
    for i in (0..length as i32).step_by(13) {
        p.line_segment(
            [
                a + direction * i as f32,
                a + direction * (i as f32 + 5.).min(length),
            ],
            Stroke::new(1.2, color),
        );
    }
}
