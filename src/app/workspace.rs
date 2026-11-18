//! Screen-sized review views. Details replace one another instead of stacking.
use super::*;

pub(super) fn heading(ui: &mut Ui, label: &str, title: &str, sub: &str) {
    eyebrow(ui, label);
    ui.label(
        RichText::new(title)
            .font(display(if ui.available_width() < 700. {
                29.
            } else {
                36.
            }))
            .color(INK),
    );
    if !sub.is_empty() {
        small(ui, sub);
    }
    ui.add_space(8.);
}
pub(super) fn tabs(ui: &mut Ui, selected: &mut usize, names: &[&str]) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 18.;
        for (i, name) in names.iter().enumerate() {
            let r = ui.add(
                Button::new(RichText::new(*name).size(14.).color(if *selected == i {
                    GOLD
                } else {
                    MUTED
                }))
                .frame(false),
            );
            if *selected == i {
                ui.painter().line_segment(
                    [r.rect.left_bottom(), r.rect.right_bottom()],
                    Stroke::new(2., TEAL),
                );
            }
            if r.clicked() {
                *selected = i;
            }
        }
    });
    ui.separator();
    ui.add_space(6.);
}
/// Panels have explicit geometry. Content can reflow without extending the page.
pub(super) fn panel(ui: &mut Ui, id: &str, rect: Rect, body: impl FnOnce(&mut Ui)) {
    ui.scope_builder(
        UiBuilder::new()
            .id_salt(id)
            .max_rect(rect)
            .layout(Layout::top_down(Align::LEFT)),
        |ui| {
            ui.set_clip_rect(rect.intersect(ui.clip_rect()));
            let response = frame().inner_margin(20).show(ui, |ui| {
                let size = (rect.size() - vec2(42., 42.)).max(vec2(1., 1.));
                ui.set_min_size(size);
                ui.set_max_size(size);
                body(ui);
            });
            atlas::panel_corner(ui.painter(), response.response.rect);
        },
    );
}
pub(super) fn body_scroll(ui: &mut Ui, id: impl std::hash::Hash, body: impl FnOnce(&mut Ui)) {
    let height = ui.available_height().max(50.);
    let r = ScrollArea::vertical()
        .id_salt(id)
        .max_height(height)
        .auto_shrink([false, false])
        .show(ui, body);
    // Used by layout regression tests to distinguish fitting views from long pages.
    ui.ctx().data_mut(|d| {
        d.get_temp_mut_or_default::<Vec<f32>>(Id::new("panel-overflows"))
            .push((r.content_size.y - height).max(0.));
    });
}
pub(super) fn columns(ui: &mut Ui, fraction: f32) -> (Rect, Rect, Rect) {
    let w = ui.available_width();
    let h = (ui.max_rect().bottom().min(ui.clip_rect().bottom()) - ui.cursor().top()).max(180.);
    let start = ui.cursor().min;
    if w >= 960. {
        let left = (w - 20.) * fraction;
        (
            Rect::from_min_size(start, vec2(left, h)),
            Rect::from_min_size(start + vec2(left + 20., 0.), vec2(w - left - 20., h)),
            Rect::from_min_size(start, vec2(w, h)),
        )
    } else {
        let a = Rect::from_min_size(start, vec2(w, 340.));
        let b = Rect::from_min_size(start + vec2(0., 360.), vec2(w, h.max(500.)));
        (a, b, a.union(b))
    }
}
fn text_pair(ui: &mut Ui, title: &str, body: &str) {
    ui.label(RichText::new(title).font(display(20.)).color(INK));
    paragraph(ui, body);
    ui.separator();
}
fn outcomes(ui: &mut Ui) {
    let brief = [
        ("Opening hours", "Evening and weekend access"),
        ("Staffing", "Reliable public service teams"),
        ("Collections", "Books and accessible formats"),
        ("Digital access", "Computers and connectivity"),
        ("Accessibility", "Fewer barriers to library use"),
        ("Programming", "Community learning and culture"),
    ];
    for row in brief.chunks(2) {
        ui.columns(2, |cols| {
            for (col, (title, body)) in cols.iter_mut().zip(row) {
                col.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.spacing_mut().item_spacing.y = 5.;
                    ui.label(RichText::new(*title).font(display(19.)).color(GOLD));
                    ui.label(RichText::new(*body).size(14.).color(MUTED));
                });
            }
        });
        ui.add_space(6.);
    }
    ui.separator();
    small(
        ui,
        "Illustrative uses only; library authorities set priorities.",
    );
}
impl ProposalApp {
    pub(super) fn workspace_page(&mut self, ui: &mut Ui) {
        match self.route {
            1 => self.atlas(ui),
            3 => self.funding(ui),
            4 => self.lab(ui),
            _ => self.review_workspace(ui),
        }
    }
    fn review_workspace(&mut self, ui: &mut Ui) {
        let route = self.route;
        let (label, title, sub, names): (&str, &str, &str, &[&str]) = match route {
            0 => (
                "THE PROPOSAL / SASKATCHEWAN",
                "A lasting fund. Stronger library communities.",
                "Corporate donations. Responsible investment. Transparent community support.",
                &[
                    "Purpose",
                    "How it works",
                    "Service outcomes",
                    "Open decisions",
                ],
            ),
            2 => (
                "CORPORATE DONATIONS",
                "A gift for a shared purpose.",
                "Build long-term capacity while respecting every gift's terms.",
                &[
                    "Long-term capital",
                    "Immediate needs",
                    "Ownership & receipts",
                ],
            ),
            5 => (
                "FAIR COMMUNITY FUNDING",
                "Local needs. Published decisions.",
                "Support follows evidence, eligibility and donor-compatible purposes.",
                &[
                    "Allocation process",
                    "Service outcomes",
                    "Illustrative awards",
                ],
            ),
            6 => (
                "OVERSIGHT & TRANSPARENCY",
                "Stewardship with clear responsibilities.",
                "Proposed roles, independent review and public reporting.",
                &[
                    "Responsibilities",
                    "Donor protections",
                    "Conflicts & reporting",
                ],
            ),
            7 => (
                "EVIDENCE & OPEN QUESTIONS",
                "Make the open decisions visible.",
                "Context is separate from verified needs, commitments and approvals.",
                &["Evidence register"],
            ),
            8 => (
                "PROPOSED READINESS",
                "Evidence before a future launch.",
                "Decision gates for discussion. No steps are represented as complete.",
                &[
                    "Listen",
                    "Structure",
                    "Policies",
                    "Allocation",
                    "Future launch",
                ],
            ),
            _ => (
                "REVIEW SUMMARY",
                "The proposal at a glance.",
                "Five questions. One proposed purpose. No donations collected.",
                &["The model", "Current scenario", "Status & downloads"],
            ),
        };
        heading(ui, label, title, sub);
        let (left, right, full) = columns(ui, 0.30);
        panel(ui, "context", left, |ui| self.context_panel(ui));
        panel(ui, "review", right, |ui| {
            tabs(ui, &mut self.review_tabs[route], names);
            body_scroll(ui, (route, self.review_tabs[route]), |ui| {
                self.review_content(ui, route)
            });
        });
        ui.advance_cursor_after_rect(full);
    }
    fn context_panel(&mut self, ui: &mut Ui) {
        let art_height = (ui.available_height() - 410.).clamp(65., 240.);
        body_scroll(ui, "context", |ui| {
            eyebrow(ui, "A CONNECTED LIBRARY COMMUNITY");
            let (art, _) =
                ui.allocate_exact_size(vec2(ui.available_width(), art_height), Sense::hover());
            atlas::draw_preview(&ui.painter().with_clip_rect(art), art);
            ui.label(
                RichText::new("CA$250 million")
                    .font(display(30.))
                    .color(TEAL),
            );
            small(ui, "Proposed corporate donation target · Not secured");
            ui.separator();
            ui.columns(2, |c| {
                c[0].label(RichText::new("50%").font(display(30.)).color(TEAL));
                small(&mut c[0], "Net interest to libraries");
                c[1].label(RichText::new("50%").font(display(30.)).color(GOLD));
                small(&mut c[1], "Net interest reinvested");
            });
            small(ui, "After investment and administration costs.");
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                if ui.button("Library atlas →").clicked() {
                    self.navigate(1);
                }
                if ui.button("Funding model →").clicked() {
                    self.navigate(3);
                }
            });
            small(
                ui,
                "Proposed trust · Structure unresolved · No receipts promised",
            );
        });
    }
    fn review_content(&mut self, ui: &mut Ui, route: usize) {
        ui.spacing_mut().item_spacing.y = 7.;
        let tab = self.review_tabs[route];
        match route {
            0 => match tab {
                0 => {
                    eyebrow(ui, "THE PURPOSE");
                    section(ui, "Help libraries plan beyond the next budget.");
                    paragraph(ui, MESSAGE);
                    text_pair(ui, "Why a lasting source?", QUESTIONS[0].1);
                    text_pair(
                        ui,
                        "Local decisions stay local",
                        "Libraries retain ownership, governance and authority over staffing, collections and services. The proposed fund provides supplementary support.",
                    );
                    small(
                        ui,
                        "Actual needs and participation must be established with libraries and communities.",
                    );
                    ui.horizontal_wrapped(|ui| {
                        if primary(ui, "Explore the library atlas") {
                            self.navigate(1);
                        }
                        if ui.button("Take the guided tour").clicked() {
                            self.start_tour();
                        }
                    });
                }
                1 => {
                    for (q, a, r) in &QUESTIONS[1..] {
                        text_pair(ui, q, a);
                        if ui.small_button("Explore →").clicked() {
                            self.navigate(*r);
                        }
                    }
                }
                2 => outcomes(ui),
                _ => {
                    section(ui, "A working proposal, with decisions still ahead.");
                    paragraph(ui, QUALIFICATION);
                    text_pair(
                        ui,
                        "Investment and spending",
                        "Money-market-style investment is the working approach. Instruments, costs, liquidity, reserves and legal suitability require professional assessment.",
                    );
                    text_pair(
                        ui,
                        "Evidence before commitments",
                        "Confirm library needs, eligibility, donor terms and independent oversight before any future launch.",
                    );
                    if ui.button("Open evidence register →").clicked() {
                        self.navigate(7);
                    }
                }
            },
            2 => match tab {
                0 => {
                    eyebrow(ui, "LONG-TERM CAPITAL");
                    section(ui, "Corporate gifts build the principal fund.");
                    paragraph(ui, TARGET_MESSAGE);
                    text_pair(
                        ui,
                        "Hold and invest donated assets",
                        "The proposed fund manages assets for its stated purpose under documented gift terms. Track restrictions on contributed principal and permitted costs.",
                    );
                    text_pair(
                        ui,
                        "A donation, without ownership rights",
                        "Donors receive no shares, equity, repayment rights or investment returns.",
                    );
                    if primary(ui, "See the funding model") {
                        self.navigate(3);
                    }
                }
                1 => {
                    eyebrow(ui, "A SEPARATE PURPOSE");
                    section(ui, "Support immediate library needs.");
                    paragraph(
                        ui,
                        "Immediate-use donations are held separately for eligible, donor-compatible grants. They are not invested or used to cover fund investment and administration costs in this illustration.",
                    );
                    text_pair(
                        ui,
                        "Keep unused gifts separate",
                        "Carryover retains its restrictions and is never silently added to long-term capital.",
                    );
                    text_pair(
                        ui,
                        "Check purpose before payment",
                        "An eligible library need must also match the terms of the gift. The calculator illustrates both pools separately.",
                    );
                }
                _ => {
                    text_pair(
                        ui,
                        "The fund holds assets",
                        "The appropriate Canadian legal and charitable structure remains to be determined. Trust is a working description.",
                    );
                    text_pair(
                        ui,
                        "Trustees oversee; libraries govern services",
                        "Trustees do not personally own fund assets. Libraries retain existing ownership, governance and service decisions.",
                    );
                    text_pair(
                        ui,
                        "No receipts promised",
                        "Relevant charitable status and receipting arrangements must be established before representing that a donor will receive a charitable tax receipt.",
                    );
                }
            },
            5 => match tab {
                0 => {
                    section(ui, "A clear path from need to support.");
                    text_pair(
                        ui,
                        "01 / Publish eligibility",
                        "Consult libraries and communities. Confirm geography, legal eligibility, purposes and gift restrictions.",
                    );
                    text_pair(
                        ui,
                        "02 / Assess documented needs",
                        "Consider service requirements, access barriers, remoteness and alternatives. Verify evidence and disclose conflicts.",
                    );
                    text_pair(
                        ui,
                        "03 / Explain awards and use",
                        "Give written reasons, offer a fair review route and report source and use of grants. Library service decisions remain local.",
                    );
                }
                1 => outcomes(ui),
                _ => self.allocation_panel(ui),
            },
            6 => match tab {
                0 => {
                    ComboBox::from_id_salt("governance-role")
                        .selected_text(ROLES[self.role].title)
                        .show_ui(ui, |ui| {
                            for (i, r) in ROLES.iter().enumerate() {
                                ui.selectable_value(&mut self.role, i, r.title);
                            }
                        });
                    let r = &ROLES[self.role];
                    eyebrow(ui, "PROPOSED RESPONSIBILITY");
                    section(ui, r.title);
                    paragraph(ui, r.mandate);
                    text_pair(ui, "Decisions", r.decisions);
                    text_pair(ui, "Controls & reporting", r.controls);
                }
                1 => {
                    section(ui, "Keep every gift's purpose visible.");
                    text_pair(
                        ui,
                        "Separate records",
                        "Track long-term capital and immediate-use funds separately, with additional donor restrictions where required.",
                    );
                    text_pair(
                        ui,
                        "Check permitted uses",
                        "Check gift terms, eligible grants and permitted costs before authorizing payments. Changing a scenario cannot release a real restriction.",
                    );
                    text_pair(
                        ui,
                        "Library independence",
                        "Respect existing library ownership, governance and authority over services.",
                    );
                }
                _ => {
                    section(ui, "Independent judgement. Visible results.");
                    text_pair(
                        ui,
                        "Conflicts of interest",
                        "Declare interests, recuse conflicted decision-makers and document related-party decisions. Provide independent review or appeal.",
                    );
                    text_pair(
                        ui,
                        "Financial and service reporting",
                        "Publish performance, costs, allocations and practical outcomes. Arrange independent financial oversight and report action on findings.",
                    );
                }
            },
            7 => {
                ui.horizontal(|ui| {
                    let l = ui.label("Search evidence");
                    ui.add(TextEdit::singleline(&mut self.search).desired_width(300.))
                        .labelled_by(l.id);
                });
                let query = self.search.to_lowercase();
                let matches: Vec<usize> = EVIDENCE
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| {
                        format!("{} {} {}", e.claim, e.detail, e.next)
                            .to_lowercase()
                            .contains(&query)
                    })
                    .map(|(i, _)| i)
                    .collect();
                if matches.is_empty() {
                    paragraph(ui, "No matching records. Try another term.");
                    return;
                }
                if !matches.contains(&self.record) {
                    self.record = matches[0];
                }
                ComboBox::from_id_salt("evidence-record")
                    .selected_text(EVIDENCE[self.record].claim)
                    .width(300.)
                    .show_ui(ui, |ui| {
                        for &i in &matches {
                            ui.selectable_value(&mut self.record, i, EVIDENCE[i].claim);
                        }
                    });
                let e = &EVIDENCE[self.record];
                badge(ui, e.status, TEAL);
                section(ui, e.claim);
                paragraph(ui, e.detail);
                text_pair(ui, "Next step", e.next);
                if !e.source.is_empty() {
                    ui.hyperlink_to("Read the primary source ↗", e.source);
                    small(
                        ui,
                        "Context checked 18 November 2026. It does not verify or endorse the proposed fund.",
                    );
                }
                ui.horizontal(|ui| {
                    let pos = matches.iter().position(|&i| i == self.record).unwrap();
                    if ui
                        .add_enabled(pos > 0, Button::new("Previous record"))
                        .clicked()
                    {
                        self.record = matches[pos - 1];
                    }
                    small(ui, &format!("{} / {}", pos + 1, matches.len()));
                    if ui
                        .add_enabled(pos + 1 < matches.len(), Button::new("Next record"))
                        .clicked()
                    {
                        self.record = matches[pos + 1];
                    }
                });
            }
            8 => {
                let (t, b) = STAGES[tab];
                badge(ui, "PROPOSED / NOT COMPLETED", TEAL);
                section(ui, &format!("{:02} / {}", tab + 1, t));
                paragraph(ui, b);
                ui.add_space(15.);
                small(
                    ui,
                    "Each stage requires evidence and a separate decision. No engagement, approval, donation collection or grantmaking is represented as completed.",
                );
                if tab < 4 {
                    if ui.button("Next readiness stage →").clicked() {
                        self.review_tabs[8] += 1;
                    }
                } else if ui.button("Open review summary →").clicked() {
                    self.navigate(9);
                }
            }
            _ => match tab {
                0 => {
                    section(ui, "Corporate gifts. Lasting community capacity.");
                    for (q, a, _) in QUESTIONS {
                        ui.label(RichText::new(q).size(16.).color(INK));
                        small(ui, a);
                    }
                }
                1 => {
                    if let Ok(p) = finance::calculate(&self.inputs) {
                        let y = p.last();
                        badge(ui, "CURRENT HYPOTHETICAL SCENARIO", TEAL);
                        for (label, v) in [
                            ("Opening donated principal", self.inputs.capital),
                            ("Final fund", y.closing),
                            ("Purchasing power", y.real_closing),
                            ("Cumulative library support", y.cumulative_support),
                            ("Final-year funding gap", y.gap),
                        ] {
                            metric_line(ui, label, &finance::cad(v));
                        }
                        small(
                            ui,
                            "Editable assumptions, not secured funds or forecasts. No permanent funding or principal guarantee.",
                        );
                        if ui.button("Edit scenario →").clicked() {
                            self.navigate(4);
                        }
                    }
                }
                _ => {
                    section(ui, "Ready for discussion, not a fundraising launch.");
                    paragraph(ui, QUALIFICATION);
                    if primary(ui, "Download review PDF") {
                        if let Err(error) = super::views::download_pdf(&self.inputs) {
                            self.toast = error;
                        }
                    }
                    small(
                        ui,
                        "Prepared by Saber Tech Inc. · Charts use your selected hypothetical scenario.",
                    );
                    if ui.button("Accessible reading & scenarios").clicked() {
                        open_reader();
                    }
                }
            },
        }
    }
    fn allocation_panel(&mut self, ui: &mut Ui) {
        badge(ui, "SYNTHETIC APPLICANTS / NO PARTICIPATION CLAIM", TEAL);
        let p = match finance::calculate(&self.inputs) {
            Ok(v) => v,
            Err(e) => {
                paragraph(ui, &e);
                return;
            }
        };
        let y = &p.years[self.selected_year.min(p.years.len() - 1)];
        let people = allocation::applicants();
        let l = ui.label("Illustrative baseline per applicant / CAD");
        ui.add(
            DragValue::new(&mut self.baseline)
                .range(0. ..=100_000.)
                .speed(100.),
        )
        .labelled_by(l.id);
        CollapsingHeader::new("Adjust allocation weights").show(ui, |ui| {
            for (i, label) in [
                "Essential services",
                "Remoteness",
                "Disadvantage",
                "Lack of alternative access",
            ]
            .iter()
            .enumerate()
            {
                ui.add(Slider::new(&mut self.weights[i], 0. ..=100.).text(*label));
            }
        });
        let endowment = allocation::allocate(
            y.endowment_distribution,
            self.baseline,
            self.weights,
            &people,
        )
        .unwrap();
        let mut remaining = people.clone();
        for (j, a) in remaining.iter_mut().enumerate() {
            a.gap = (a.gap - endowment.awards[j]).max(0.);
        }
        let immediate = allocation::allocate(
            y.immediate_distribution,
            self.baseline,
            self.weights,
            &remaining,
        )
        .unwrap();
        ScrollArea::horizontal().show(ui, |ui| {
            Grid::new("allocation-grid").striped(true).show(ui, |ui| {
                for h in [
                    "Illustrative applicant",
                    "Need",
                    "Fund grant",
                    "Immediate grant",
                ] {
                    ui.strong(h);
                }
                ui.end_row();
                for (j, a) in people.iter().enumerate() {
                    ui.label(&a.name);
                    ui.label(finance::cad(a.gap));
                    ui.label(finance::cad(endowment.awards[j]));
                    ui.label(finance::cad(immediate.awards[j]));
                    ui.end_row();
                }
            });
        });
        small(
            ui,
            &format!(
                "Year {} · Unallocated fund source: {} · Separate immediate source: {}.",
                y.year,
                finance::cad(endowment.unallocated),
                finance::cad(immediate.unallocated)
            ),
        );
        small(
            ui,
            "Requests are independent of the scenario's total need. Each source remains separate. This illustration assumes gifts permit the same eligible needs; actual restrictions require checks.",
        );
    }
    pub(super) fn funding(&mut self, ui: &mut Ui) {
        heading(
            ui,
            "SASKATCHEWAN / LIBRARY FUNDING",
            "Build the fund. Sustain library access.",
            "",
        );
        Frame::new().fill(Color32::from_rgb(10,35,49)).inner_margin(14).show(ui,|ui|{ui.set_width(ui.available_width());ui.label(RichText::new("Proposed model.  CA$250 million in corporate donations would establish the principal fund. 50% of positive net interest would support libraries; 50% would be reinvested. The target is not secured capital.").size(15.).color(GOLD));});
        tabs(
            ui,
            &mut self.funding_tab,
            &[
                "01  How it works",
                "02  Scenario calculator",
                "03  Funding & accountability",
            ],
        );
        if self.funding_tab == 1 {
            self.scenario_workspace(ui);
            return;
        }
        let (left, right, full) = columns(ui, 0.53);
        if self.funding_tab == 0 {
            panel(ui, "funding-split", left, |ui| {
                let h = (ui.available_height() - 150.).clamp(130., 340.);
                body_scroll(ui, "split", |ui| {
                    eyebrow(ui, "PROPOSED ALLOCATION OF NET CASH INTEREST");
                    let w = ui.available_width();
                    if w >= 500. {
                        ui.allocate_ui_with_layout(
                            vec2(w, h),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                let (r, _) =
                                    ui.allocate_exact_size(vec2(w * 0.43, h), Sense::hover());
                                split_ring(ui, r);
                                ui.allocate_ui_with_layout(
                                    vec2(w * 0.53, h),
                                    Layout::top_down(Align::LEFT),
                                    |ui| {
                                        ui.add_space((h - 205.).max(0.) / 2.);
                                        funding_metrics(ui);
                                    },
                                );
                            },
                        );
                    } else {
                        let (r, _) = ui.allocate_exact_size(vec2(w, 170.), Sense::hover());
                        split_ring(ui, r);
                        funding_metrics(ui);
                    }
                    ui.separator();
                    eyebrow(
                        ui,
                        "DONATIONS → PRINCIPAL FUND → INTEREST → LIBRARY SUPPORT",
                    );
                    small(
                        ui,
                        "Net interest is cash interest after investment and administration costs. Grants respect reserves, donor restrictions and eligible need. Unused grant capacity stays in the fund.",
                    );
                });
            });
            panel(ui, "funding-outcomes", right, |ui| {
                body_scroll(ui, "outcomes", |ui| {
                    ui.spacing_mut().item_spacing.y = 7.;
                    eyebrow(ui, "WHAT LIBRARY SUPPORT COULD MAKE POSSIBLE");
                    section(ui, "The places we learn.\nThe services we share.");
                    outcomes(ui);
                    small(
                        ui,
                        "Interest, funding and purchasing power vary. Principal is not guaranteed.",
                    );
                })
            });
        } else {
            panel(ui, "funding-rules", left, |ui| {
                body_scroll(ui, "rules", |ui| {
                    eyebrow(ui, "A CLEAR RULE / TWO DISTINCT GIFT PURPOSES");
                    section(ui, "Protect the purpose of each gift.");
                    text_pair(
                        ui,
                        "Long-term donated capital",
                        "Invest under documented gift terms. The proposed spending budget uses positive net cash interest, with 50% for libraries and 50% reinvested.",
                    );
                    text_pair(
                        ui,
                        "Immediate-use donations",
                        "Keep separately accounted gifts available for eligible immediate needs. Unused funds retain their original restrictions.",
                    );
                    text_pair(
                        ui,
                        "Reserves and purchasing power",
                        "Retain resources under an approved policy. Losses, costs and inflation can reduce capacity despite reinvestment.",
                    );
                })
            });
            panel(ui, "funding-oversight", right, |ui| {
                body_scroll(ui, "oversight", |ui| {
                    eyebrow(ui, "PROPOSED OVERSIGHT");
                    section(ui, "Accountable to the stated purpose.");
                    text_pair(
                        ui,
                        "Trustees and professionals",
                        "The governing board oversees assets, policy, risk and costs. Professional managers operate within an approved mandate. Trustees do not personally own the assets.",
                    );
                    text_pair(
                        ui,
                        "Fair awards and visible reporting",
                        "Publish eligibility, conflicts controls, allocation decisions and financial reports. Arrange independent scrutiny. Library authorities keep service decisions.",
                    );
                    small(ui, QUALIFICATION);
                    if ui.button("Explore oversight →").clicked() {
                        self.navigate(6);
                    }
                })
            });
        }
        ui.advance_cursor_after_rect(full);
    }
}
pub(super) fn metric_line(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal_wrapped(|ui| {
        small(ui, label);
        ui.label(RichText::new(value).font(display(25.)).color(GOLD));
    });
    ui.separator();
}
fn funding_metrics(ui: &mut Ui) {
    ui.spacing_mut().item_spacing.y = 6.;
    for (label, value, detail) in [
        (
            "Corporate donation target",
            "CA$250M",
            "Target · Not secured",
        ),
        ("Library support", "50%", "Of net cash interest"),
        ("Reinvestment", "50%", "Of net cash interest"),
    ] {
        ui.label(RichText::new(label).size(16.).color(INK));
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(value).font(display(27.)).color(TEAL));
            small(ui, detail);
        });
        ui.separator();
    }
}
fn split_ring(ui: &Ui, r: Rect) {
    let p = ui.painter().with_clip_rect(r);
    let c = r.center();
    let radius = (r.width().min(r.height()) * 0.43).min(145.);
    for half in 0..2 {
        let pts: Vec<Pos2> = (0..=64)
            .map(|j| {
                let a = -std::f32::consts::FRAC_PI_2
                    + half as f32 * std::f32::consts::PI
                    + 0.025
                    + j as f32 / 64. * (std::f32::consts::PI - 0.05);
                c + vec2(a.cos(), a.sin()) * radius
            })
            .collect();
        p.add(Shape::line(
            pts,
            Stroke::new(
                20.,
                if half == 0 {
                    TEAL
                } else {
                    Color32::from_rgb(39, 100, 134)
                },
            ),
        ));
    }
    p.text(
        c + vec2(0., -35.),
        Align2::CENTER_CENTER,
        "NET INTEREST",
        FontId::monospace(10.),
        MUTED,
    );
    p.text(
        c,
        Align2::CENTER_CENTER,
        "50 / 50",
        display(if radius < 85. { 27. } else { 38. }),
        INK,
    );
    p.text(
        c + vec2(0., 33.),
        Align2::CENTER_CENTER,
        "after costs",
        FontId::proportional(13.),
        MUTED,
    );
}
