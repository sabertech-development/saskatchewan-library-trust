use crate::{
    allocation,
    content::*,
    finance::{self, Cash, Inputs},
};
use eframe::egui::*;

mod views;
pub(super) const BG: Color32 = Color32::from_rgb(7, 15, 25);
pub(super) const PANEL: Color32 = Color32::from_rgb(12, 25, 37);
pub(super) const LINE: Color32 = Color32::from_rgb(38, 58, 70);
pub(super) const INK: Color32 = Color32::from_rgb(238, 235, 222);
pub(super) const MUTED: Color32 = Color32::from_rgb(161, 180, 188);
pub(super) const TEAL: Color32 = Color32::from_rgb(115, 204, 183);
pub(super) const GOLD: Color32 = Color32::from_rgb(214, 184, 120);
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
    pub(super) evidence_filter: usize,
    pub(super) tour: Option<usize>,
    pub(super) mobile_menu: bool,
    pub(super) toast: String,
    pub(super) flow_step: usize,
}
impl ProposalApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::with_context(&cc.egui_ctx)
    }
    /// Shared initialization for the browser and deterministic offscreen QA.
    pub fn with_context(ctx: &Context) -> Self {
        let mut style = (*ctx.style()).clone();
        style.visuals = Visuals::dark();
        style.visuals.panel_fill = BG;
        style.visuals.window_fill = PANEL;
        style.visuals.extreme_bg_color = Color32::from_rgb(5, 12, 20);
        style.visuals.override_text_color = Some(INK);
        style.visuals.hyperlink_color = TEAL;
        style.visuals.selection.bg_fill = Color32::from_rgb(28, 80, 74);
        style.visuals.selection.stroke = Stroke::new(1.5, TEAL);
        style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(20, 38, 50);
        style.visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(20, 38, 50);
        style.visuals.widgets.inactive.bg_stroke = Stroke::new(1., LINE);
        style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(31, 65, 71);
        style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(31, 65, 71);
        style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, TEAL);
        style.visuals.widgets.active.bg_stroke = Stroke::new(2., TEAL);
        style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1., LINE);
        style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1., INK);
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
        style.spacing.item_spacing = vec2(12., 12.);
        style.spacing.button_padding = vec2(16., 10.);
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
            baseline: 100_000.,
            weights: [40., 25., 20., 15.],
            search: String::new(),
            evidence_filter: 0,
            tour: None,
            mobile_menu: false,
            toast: String::new(),
            flow_step: 5,
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
            RichText::new("PRIVATE CAPITAL\nPUBLIC PURPOSE")
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
    fn header(&mut self, ui: &mut Ui, mobile: bool) {
        ui.horizontal_wrapped(|ui| {
            let (r, _) = ui.allocate_exact_size(vec2(40., 42.), Sense::hover());
            draw_book(ui.painter(), r.center(), 26., TEAL);
            ui.vertical(|ui| {
                ui.label(RichText::new("SASKATCHEWAN").size(16.).color(INK).strong());
                ui.label(
                    RichText::new("Private Equity Library Funding")
                        .size(if mobile { 14. } else { 16. })
                        .color(MUTED),
                );
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if mobile && ui.button("Menu").clicked() {
                    self.mobile_menu = !self.mobile_menu;
                }
                if !mobile && ui.button("Review summary").clicked() {
                    self.navigate(9);
                }
                if !mobile && ui.button("Copy section link").clicked() {
                    copy_link(ui.ctx());
                    self.toast =
                        "Section link copied. Custom model inputs are not included.".into();
                }
            });
        });
    }
    fn overview(&mut self, ui: &mut Ui) {
        eyebrow(ui, "A PROVINCIAL PROPOSAL / FOR DISCUSSION");
        ui.add_space(14.);
        ui.label(
            RichText::new("Private enterprise.\nLasting library access.")
                .size(if ui.available_width() < 600. {
                    38.
                } else {
                    58.
                })
                .color(INK),
        );
        ui.add_space(12.);
        paragraph(
            ui,
            "A proposed private equity model connecting ownership of productive businesses with recurring support for libraries across Saskatchewan.",
        );
        ui.add_space(16.);
        ui.horizontal_wrapped(|ui| {
            if primary(ui, "Explore the model") {
                self.navigate(1);
            }
            if ui.button("Take the guided tour").clicked() {
                self.start_tour();
            }
            badge(ui, "PROPOSED · NOT AN INVESTMENT OFFERING", GOLD);
        });
        ui.add_space(25.);
        frame().show(ui, |ui| {
            eyebrow(ui, "THE CONNECTION");
            let (rect, _) = ui.allocate_exact_size(
                vec2(
                    ui.available_width(),
                    if ui.available_width() < 600. {
                        260.
                    } else {
                        160.
                    },
                ),
                Sense::hover(),
            );
            let narrow = rect.width() < 600.;
            let steps = [
                ("Private investors", "Equity subscriptions", 0),
                ("Business ownership", "Commercial operations", 2),
                ("Available cash", "After costs and reserves", 3),
                ("Shared proceeds", "Investors · libraries · reinvestment", 5),
            ];
            for (i, (name, sub, kind)) in steps.iter().enumerate() {
                let x = if narrow {
                    rect.left() + rect.width() * (0.25 + 0.5 * (i % 2) as f32)
                } else {
                    rect.left() + rect.width() * (0.125 + 0.25 * i as f32)
                };
                let y = rect.top()
                    + if narrow {
                        38. + (i / 2) as f32 * 126.
                    } else {
                        43.
                    };
                let p = pos2(x, y);
                draw_icon(
                    ui.painter(),
                    p,
                    28.,
                    *kind,
                    if i == 3 { TEAL } else { GOLD },
                );
                ui.painter().text(
                    p + vec2(0., 39.),
                    Align2::CENTER_CENTER,
                    *name,
                    FontId::proportional(17.),
                    INK,
                );
                let subtitle = if narrow {
                    match i {
                        2 => "After costs & reserve",
                        3 => "Three proposed shares",
                        _ => *sub,
                    }
                } else {
                    *sub
                };
                ui.painter().text(
                    p + vec2(0., 65.),
                    Align2::CENTER_CENTER,
                    subtitle,
                    FontId::proportional(13.),
                    MUTED,
                );
                if !narrow && i < 3 {
                    line_arrow(
                        ui.painter(),
                        p + vec2(40., 0.),
                        p + vec2(rect.width() / 4. - 40., 0.),
                        LINE,
                    );
                }
            }
        });
        ui.add_space(20.);
        if let Ok(c) = finance::calculate(&self.inputs) {
            metric_row(
                ui,
                &[
                    (
                        "Subscribed equity",
                        finance::compact(self.inputs.capital),
                        "Illustrative private capital",
                    ),
                    (
                        "Annual library allocation",
                        finance::compact(c.libraries),
                        "Cash-supported model output",
                    ),
                    (
                        "Funding need covered",
                        format!("{:.1}%", c.coverage.unwrap_or(0.) * 100.),
                        "Of the current illustrative need",
                    ),
                ],
            );
        }
        ui.add_space(24.);
        section(ui, "Three proposed commitments");
        for (title, copy) in [
            (
                "Provincial reach",
                "Published eligibility rules across urban, rural, northern and remote service contexts.",
            ),
            (
                "An explicit library share",
                "An agreed portion of available proceeds, accepted by investors and written into governing agreements.",
            ),
            (
                "Accountability in view",
                "Transparent assumptions, independent oversight and reporting on both cash and service outcomes.",
            ),
        ] {
            item(ui, title, copy);
        }
        frame().show(ui,|ui|{section(ui,"What this proposal needs to prove");for s in ["Investors will accept the proposed terms.","Suitable businesses can be acquired at viable prices.","Businesses can generate sufficient distributable cash.","Reserves can support the timing of approved payments.","Governance can protect the investment mandate and library allocation process."]{bullet(ui,s);}small(ui,"These conditions have not been established. Capital preservation, investor returns and library payments are not guaranteed.");});
        ui.add_space(20.);
        self.boundaries(ui);
    }
    pub(super) fn boundaries(&self, ui: &mut Ui) {
        section(ui, "The boundaries are explicit");
        paragraph(
            ui,
            "All investment capital would come from private equity subscriptions. The baseline uses no borrowing, new government funding, charitable donations, crowdfunding, sponsorship revenue or library user charges.",
        );
        paragraph(
            ui,
            "Library buildings, collections, patron information and public services are not investor collateral and are not transferred to investors. Existing library revenues establish funding gaps; they are not vehicle revenue.",
        );
        small(
            ui,
            "This is a concept for review by library leadership, professional advisers, prospective investment partners and elected representatives. No government or library endorsement has been obtained. No investor commitments or visitor financial inputs are collected.",
        );
    }
    fn atlas(&mut self, ui: &mut Ui) {
        page_title(
            ui,
            "PROVINCIAL ATLAS",
            "One connected proposal.",
            "Select a node to inspect its purpose, relationships and outstanding evidence.",
        );
        ui.horizontal_wrapped(|ui| {
            for (i, name) in ["Money", "Authority", "Evidence"].iter().enumerate() {
                ui.selectable_value(&mut self.overlay, i, *name);
            }
        });
        ui.add_space(12.);
        let width = ui.available_width();
        if width > 1050. {
            ui.columns(2, |cols| {
                self.network(&mut cols[0], 480.);
                self.inspector(&mut cols[1]);
            });
        } else {
            self.network(ui, if width < 500. { 390. } else { 500. });
            ui.add_space(16.);
            self.inspector(ui);
        }
        ui.add_space(20.);
        section(ui, "Across different service contexts");
        small(
            ui,
            "Synthetic examples only. Names, funding gaps and service needs below do not describe actual libraries.",
        );
        for a in allocation::applicants() {
            CollapsingHeader::new(format!("{}  /  {}",a.name,a.context)).id_salt(&a.name).show(ui,|ui|{badge(ui,"SYNTHETIC PROFILE",GOLD);paragraph(ui,&a.outcome);ui.label(format!("Illustrative annual gap: {}",finance::cad(a.gap)));small(ui,"Participation is not agreed. Final needs and service outcomes require local consultation.");});
        }
    }
    fn network(&mut self, ui: &mut Ui, height: f32) {
        frame().show(ui,|ui|{
            eyebrow(ui,match self.overlay{0=>"CASH RELATIONSHIPS",1=>"PROPOSED DECISION AUTHORITY",_=>"EVIDENCE & READINESS"});
            let (r,_)=ui.allocate_exact_size(vec2(ui.available_width(),height),Sense::hover());
            let p=ui.painter().with_clip_rect(r);
            // Original schematic; no geographic coordinates or province outline.
            for x in (0..=(r.width()/30.)as i32).map(|v|r.left()+v as f32*30.) {for y in(0..=(height/30.)as i32).map(|v|r.top()+v as f32*30.){p.circle_filled(pos2(x,y),0.8,Color32::from_rgb(28,46,57));}}
            let at=|idx:usize|pos2(r.left()+NODES[idx].pos[0]*r.width(),r.top()+NODES[idx].pos[1]*r.height());
            let links:Vec<(usize,usize,&str)>=match self.overlay{
                0=>vec![(0,1,"Equity"),(1,0,"Investor cash"),(1,2,"Ownership"),(2,1,"Business cash"),(1,3,"Reserve"),(1,4,"Library pool"),(4,5,"Awards"),(5,6,"Services")],
                1=>vec![(1,2,"Approve / oversee"),(1,3,"Set policy"),(1,4,"Delegate mandate"),(4,5,"Approve awards"),(5,6,"Deliver / report")],
                _=>vec![(0,1,"Terms unresolved"),(2,1,"Not selected"),(1,4,"Proposed policy"),(4,5,"Not agreed"),(5,6,"Synthetic outcomes")]
            };
            for(a,b,label)in links{let start=at(a);let end=at(b);let v=(end-start).normalized();let parallel=self.overlay==0&&((a==1&&(b==0||b==2))||(b==1&&(a==0||a==2)));let offset=if parallel{vec2(-v.y,v.x)*if r.width()>650.{15.}else{5.}}else{Vec2::ZERO};let start=start+offset;let end=end+offset;let color=if a==self.node||b==self.node{match self.overlay{1=>GOLD,2=>MUTED,_=>TEAL}}else{LINE};line_arrow(&p,start+v*32.,end-v*37.,color);if r.width()>650.{let mid=start.lerp(end,0.51);p.rect_filled(Rect::from_center_size(mid,vec2(label.len() as f32*6.7+12.,21.)),3.,PANEL);p.text(mid,Align2::CENTER_CENTER,label,FontId::proportional(12.),color);}}
            for(i,n)in NODES.iter().enumerate(){let point=at(i);let selected=self.node==i;let hit=Rect::from_center_size(point+vec2(0.,19.),vec2(if r.width()<500.{110.}else{150.},94.));let resp=ui.interact(hit,ui.id().with(("node",i)),Sense::click());resp.widget_info(||WidgetInfo::labeled(WidgetType::Button,true,format!("Select {}",n.name)));
                if selected||resp.hovered()||resp.has_focus(){p.circle_filled(point,41.,Color32::from_rgb(23,49,56));p.circle_stroke(point,42.,Stroke::new(if resp.has_focus(){3.}else{1.},TEAL));}
                draw_icon(&p,point,28.,n.kind,if selected{TEAL}else{GOLD});
                let label=if r.width()<450.{match i{0=>"Investors",1=>"Vehicle",2=>"Businesses",3=>"Reserve",4=>"Committee",5=>"Libraries",_=>"Communities"}}else{n.short};
                p.text(point+vec2(0.,48.),Align2::CENTER_CENTER,label,FontId::proportional(if r.width()<500.{13.}else{15.}),if selected{TEAL}else{INK});
                if resp.clicked(){self.node=i;}
            }
            small(ui,"Conceptual layout — not a geographic map");
            small(ui,match self.overlay{0=>"Cash lines show the mechanism, not commitments. Select a node for the terms.",1=>"Board → manager / investment committee → businesses; allocation committee → awards.",_=>"Every node is illustrative, proposed or unresolved. No operating arrangement is verified."});
        });
    }
    fn inspector(&mut self, ui: &mut Ui) {
        let n = &NODES[self.node];
        frame().show(ui, |ui| {
            badge(ui, n.readiness, GOLD);
            section(ui, n.name);
            paragraph(ui, n.purpose);
            for (title, body) in [
                ("Inputs", n.inputs),
                ("Outputs", n.outputs),
                ("Responsible party", n.owner),
                ("Dependencies", n.dependencies),
                ("Outstanding question", n.question),
            ] {
                item(ui, title, body);
            }
            ui.horizontal_wrapped(|ui| {
                if ui.button("Inspect the cash model").clicked() {
                    self.navigate(4);
                }
                if ui.button("View evidence").clicked() {
                    self.navigate(7);
                }
            });
        });
    }
    fn investment(&mut self, ui: &mut Ui) {
        page_title(
            ui,
            "INVESTMENT MODEL",
            "Ownership with an agreed purpose.",
            "Private investors would own interests in a proposed holding vehicle. The vehicle would own commercial businesses.",
        );
        frame().show(ui,|ui|{eyebrow(ui,"OWNERSHIP");section(ui,"Investors  →  Proposed vehicle  →  Portfolio businesses");paragraph(ui,"Investors own vehicle interests; they do not own library assets. Long-term ownership is the proposed approach, with business management overseen by the investment manager and investment committee.");});
        ui.add_space(15.);
        frame().show(ui,|ui|{eyebrow(ui,"SEPARATE FUNDING RELATIONSHIP");section(ui,"Vehicle  →  Funding agreements  →  Eligible libraries");paragraph(ui,"The library allocation must be accepted by investors and incorporated into governing agreements. A proposed policy is not legally protected until those documents establish its protection.");});
        ui.add_space(22.);
        section(ui, "What an acquisition would need to demonstrate");
        for s in [
            "Established operations and demonstrated cash generation.",
            "An understandable business model and capable management.",
            "Manageable capital expenditure and working-capital requirements.",
            "Diversification across businesses and sectors.",
            "Long-term ownership suitability and responsible operating practices.",
        ] {
            bullet(ui, s);
        }
        ui.add_space(18.);
        section(ui, "Illustrative business profiles");
        small(
            ui,
            "Entirely fictional. These examples are separate from the aggregate calculator and are not selected investments. Cash shown is after business operating costs, taxes, maintenance capital expenditure and working capital.",
        );
        for (name, sector, equity, revenue, cash) in [
            (
                "Prairie Service Works",
                "Business & maintenance services",
                18_000_000.,
                14_500_000.,
                1_350_000.,
            ),
            (
                "Northern Route Logistics",
                "Established logistics services",
                24_000_000.,
                22_000_000.,
                1_800_000.,
            ),
            (
                "Aspen Component Works",
                "Light manufacturing",
                30_000_000.,
                26_000_000.,
                2_250_000.,
            ),
        ] {
            frame().show(ui, |ui| {
                badge(ui, "SYNTHETIC BUSINESS", GOLD);
                section(ui, name);
                paragraph(ui, sector);
                metric_row(
                    ui,
                    &[
                        (
                            "Illustrative equity",
                            finance::compact(equity),
                            "Not an acquisition price opinion",
                        ),
                        (
                            "Annual revenue",
                            finance::compact(revenue),
                            "Synthetic operating figure",
                        ),
                        (
                            "Annual business cash",
                            finance::compact(cash),
                            "Separate from calculator",
                        ),
                    ],
                );
            });
            ui.add_space(12.);
        }
        section(ui, "Commercial matters to resolve");
        for (title, text) in [
            (
                "Due diligence & business oversight",
                "Independent diligence must assess financial quality, customers, staff, liabilities, management, capital needs and responsible operations before an acquisition.",
            ),
            (
                "Reinvestment & operating risk",
                "Retained cash supports future commercial investment; it is not a payment to investors or libraries. Concentration, cash conversion and operating losses can reduce or eliminate distributions.",
            ),
            (
                "Liquidity & potential exits",
                "Long-term ownership can restrict liquidity. Investor redemption rights, transfer limits and potential business exits remain unresolved. Sale proceeds and capital returns are excluded from the baseline annual model.",
            ),
            (
                "Investor economics",
                "Preferred returns, carried interest and capital redemptions are not in the baseline. Introducing them would require revised agreements and a revised model.",
            ),
        ] {
            item(ui, title, text);
        }
        self.boundaries(ui);
    }
}

impl ProposalApp {
    pub fn render(&mut self, ctx: &Context) {
        if let Some(route) = browser_route() {
            if route != self.route {
                self.route = route;
            }
        }
        let mobile = ctx.content_rect().width() < 820.;
        TopBottomPanel::top("masthead")
            .frame(
                Frame::new()
                    .fill(BG)
                    .inner_margin(Margin::symmetric(if mobile { 16 } else { 28 }, 18)),
            )
            .show(ctx, |ui| self.header(ui, mobile));
        TopBottomPanel::bottom("status")
            .frame(
                Frame::new()
                    .fill(BG)
                    .inner_margin(Margin::symmetric(18, 10)),
            )
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new(STATUS).size(13.).color(GOLD));
                    if !mobile {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(
                                RichText::new("CAD · No payments guaranteed")
                                    .size(13.)
                                    .color(MUTED),
                            );
                        });
                    }
                });
            });
        if !mobile {
            SidePanel::left("navigation")
                .exact_width(238.)
                .resizable(false)
                .frame(
                    Frame::new()
                        .fill(Color32::from_rgb(9, 20, 31))
                        .inner_margin(Margin::same(20)),
                )
                .show(ctx, |ui| {
                    ScrollArea::vertical().show(ui, |ui| self.navigation(ui));
                });
        }
        if self.mobile_menu {
            TopBottomPanel::top("mobile navigation")
                .exact_height(420.)
                .resizable(false)
                .show(ctx, |ui| {
                    ScrollArea::vertical()
                        .max_height(420.)
                        .show(ui, |ui| self.navigation(ui));
                });
        }
        if let Some(step) = self.tour {
            TopBottomPanel::top("guided tour")
                .frame(frame())
                .show(ctx, |ui| {
                    eyebrow(
                        ui,
                        &format!("GUIDED TOUR · {} OF 6 · ABOUT 5 MINUTES", step + 1),
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
                            if step == 5 {
                                "Finish in review summary"
                            } else {
                                "Next stop"
                            },
                        ) {
                            if step == 5 {
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
        CentralPanel::default().frame(Frame::new().fill(BG).inner_margin(Margin::same(if mobile{18}else{32}))).show(ctx,|ui|{
            ScrollArea::vertical().id_salt(("page",self.route)).auto_shrink([false,false]).show(ui,|ui|{
                ui.set_max_width(ui.available_width().min(1560.));
                if !self.toast.is_empty(){ui.label(RichText::new(&self.toast).color(TEAL));ui.add_space(12.);}
                match self.route{0=>self.overview(ui),1=>self.atlas(ui),2=>self.investment(ui),3=>self.money(ui),4=>self.lab(ui),5=>self.allocate_view(ui),6=>self.governance(ui),7=>self.evidence(ui),8=>self.roadmap(ui),_=>self.summary(ui)}
                ui.add_space(30.);ui.separator();small(ui,"Saskatchewan Private Equity Library Funding · Proposed arrangements for discussion. All financial amounts are illustrative Canadian dollars. No endorsement, funding, participation or investment return is represented as secured.");ui.add_space(12.);
                ui.horizontal_wrapped(|ui|{if ui.button("Copy section link").clicked(){copy_link(ui.ctx());self.toast="Section link copied. Custom financial inputs are not shared.".into();}small(ui,"Motion-free interface · Tab to navigate controls");});
            });
        });
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
        .corner_radius(8)
        .inner_margin(Margin::same(22))
}
pub(super) fn eyebrow(ui: &mut Ui, t: &str) {
    ui.label(RichText::new(t).color(TEAL).size(13.).strong());
}
pub(super) fn small(ui: &mut Ui, t: &str) {
    ui.label(RichText::new(t).color(MUTED).size(14.));
}
pub(super) fn paragraph(ui: &mut Ui, t: &str) {
    ui.label(RichText::new(t).color(MUTED).size(16.));
    ui.add_space(4.);
}
pub(super) fn section(ui: &mut Ui, t: &str) {
    ui.label(RichText::new(t).color(INK).size(24.));
    ui.add_space(5.);
}
pub(super) fn item(ui: &mut Ui, title: &str, text: &str) {
    ui.label(RichText::new(title).size(17.).color(INK).strong());
    paragraph(ui, text);
    ui.add_space(8.);
}
pub(super) fn bullet(ui: &mut Ui, t: &str) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("•").color(TEAL));
        ui.label(t);
    });
}
pub(super) fn badge(ui: &mut Ui, t: &str, c: Color32) {
    ui.label(RichText::new(t).color(c).size(13.).strong());
}
pub(super) fn primary(ui: &mut Ui, t: &str) -> bool {
    ui.add(
        Button::new(RichText::new(t).color(BG).strong())
            .fill(TEAL)
            .min_size(vec2(0., 44.)),
    )
    .clicked()
}
pub(super) fn page_title(ui: &mut Ui, k: &str, t: &str, d: &str) {
    eyebrow(ui, k);
    ui.add_space(10.);
    ui.label(
        RichText::new(t)
            .size(if ui.available_width() < 600. {
                34.
            } else {
                44.
            })
            .color(INK),
    );
    ui.add_space(8.);
    paragraph(ui, d);
    ui.add_space(20.);
}
pub(super) fn metric_row(ui: &mut Ui, items: &[(&str, String, &str)]) {
    if ui.available_width() < 650. {
        for (label, value, sub) in items {
            frame().show(ui, |ui| {
                small(ui, label);
                ui.label(RichText::new(value).size(32.).color(TEAL));
                small(ui, sub);
            });
            ui.add_space(8.);
        }
    } else {
        ui.columns(items.len(), |cols| {
            for (i, (label, value, sub)) in items.iter().enumerate() {
                frame().show(&mut cols[i], |ui| {
                    small(ui, label);
                    ui.label(RichText::new(value).size(29.).color(TEAL));
                    small(ui, sub);
                });
            }
        });
    }
}
pub(super) fn line_arrow(p: &Painter, a: Pos2, b: Pos2, color: Color32) {
    p.line_segment([a, b], Stroke::new(1.5, color));
    let v = (b - a).normalized();
    let n = vec2(-v.y, v.x);
    p.line_segment([b, b - v * 7. + n * 4.], Stroke::new(1.5, color));
    p.line_segment([b, b - v * 7. - n * 4.], Stroke::new(1.5, color));
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

#[cfg(test)]
mod ui_tests {
    use super::*;
    fn tick(app: &mut ProposalApp, ctx: &Context, w: f32, events: Vec<Event>) -> FullOutput {
        ctx.run(
            RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(w, 1200.))),
                events,
                ..Default::default()
            },
            |ctx| app.render(ctx),
        )
    }
    fn setup() -> (Context, ProposalApp) {
        let ctx = Context::default();
        ctx.enable_accesskit();
        let app = ProposalApp::with_context(&ctx);
        (ctx, app)
    }
    fn click(app: &mut ProposalApp, ctx: &Context, label: &str, w: f32) {
        let mut out = tick(app, ctx, w, vec![]);
        for _ in 0..2 {
            out = tick(app, ctx, w, vec![]);
        }
        let tree = out
            .platform_output
            .accesskit_update
            .expect("accessibility tree");
        let b = tree
            .nodes
            .iter()
            .find_map(|(_, n)| {
                if n.label() == Some(label) || n.value() == Some(label) {
                    n.bounds()
                } else {
                    None
                }
            })
            .unwrap_or_else(|| panic!("Missing control {label}"));
        let pos = pos2(((b.x0 + b.x1) / 2.) as f32, ((b.y0 + b.y1) / 2.) as f32);
        tick(app, ctx, w, vec![Event::PointerMoved(pos)]);
        tick(
            app,
            ctx,
            w,
            vec![
                Event::PointerMoved(pos),
                Event::PointerButton {
                    pos,
                    button: PointerButton::Primary,
                    pressed: true,
                    modifiers: Modifiers::NONE,
                },
            ],
        );
        tick(
            app,
            ctx,
            w,
            vec![Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed: false,
                modifiers: Modifiers::NONE,
            }],
        );
    }
    #[test]
    fn every_route_renders_desktop_and_mobile() {
        for w in [390., 820., 1440.] {
            let (ctx, mut app) = setup();
            for route in 0..ROUTES.len() {
                app.navigate(route);
                for _ in 0..3 {
                    let out = tick(&mut app, &ctx, w, vec![]);
                    assert!(out.shapes.len() > 20);
                }
            }
        }
    }
    #[test]
    fn navigation_controls_and_atlas_overlays() {
        let (ctx, mut app) = setup();
        for (i, (_, name)) in ROUTES.iter().enumerate() {
            click(&mut app, &ctx, name, 1440.);
            assert_eq!(app.route, i);
        }
        app.navigate(1);
        click(&mut app, &ctx, "Authority", 1440.);
        assert_eq!(app.overlay, 1);
        click(&mut app, &ctx, "Evidence", 1440.);
        assert_eq!(app.overlay, 2);
        click(&mut app, &ctx, "Select Private investors", 1440.);
        assert_eq!(app.node, 0);
    }
    #[test]
    fn mobile_menu_opens_and_navigates() {
        let (ctx, mut app) = setup();
        click(&mut app, &ctx, "Menu", 390.);
        assert!(app.mobile_menu);
        click(&mut app, &ctx, "Provincial Atlas", 390.);
        assert_eq!(app.route, 1);
        assert!(!app.mobile_menu);
    }
    #[test]
    fn guided_tour_visits_every_stop_and_exits() {
        let (ctx, mut app) = setup();
        click(&mut app, &ctx, "Take the guided tour", 1440.);
        assert_eq!(app.tour, Some(0));
        for step in 1..6 {
            click(&mut app, &ctx, "Next stop", 1440.);
            assert_eq!(app.tour, Some(step));
            assert_eq!(app.route, TOUR[step].0);
        }
        click(&mut app, &ctx, "Finish in review summary", 1440.);
        assert_eq!(app.route, 9);
        assert!(app.tour.is_none());
    }
    #[test]
    fn scenarios_and_reset_drive_shared_results() {
        let (ctx, mut app) = setup();
        app.navigate(4);
        click(&mut app, &ctx, "Base illustration", 1440.);
        click(&mut app, &ctx, "No business distributions", 1440.);
        assert_eq!(app.preset, 2);
        assert_eq!(app.inputs.yield_rate, 0.);
        let cash = finance::calculate(&app.inputs).unwrap();
        assert_eq!(cash.libraries, 0.);
        assert!(cash.reserve_used > 0.);
        for route in [0, 3, 5, 9] {
            app.navigate(route);
            tick(&mut app, &ctx, 1440., vec![]);
            assert_eq!(finance::calculate(&app.inputs).unwrap().libraries, 0.);
        }
        app.navigate(4);
        click(&mut app, &ctx, "Reset all inputs", 1440.);
        assert_eq!(app.preset, 0);
        assert_eq!(
            finance::calculate(&app.inputs).unwrap().libraries,
            1_468_000.
        );
    }
    #[test]
    fn tab_moves_focus_and_enter_activates() {
        let (ctx, mut app) = setup();
        tick(&mut app, &ctx, 1440., vec![]);
        let event = |key| Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        };
        tick(&mut app, &ctx, 1440., vec![event(Key::Tab)]);
        assert!(ctx.memory(|m| m.focused()).is_some());
        tick(&mut app, &ctx, 1440., vec![event(Key::Enter)]);
    }
}
