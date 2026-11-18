use super::workspace::{body_scroll, columns, heading, panel, tabs};
use super::*;

impl ProposalApp {
    pub(super) fn lab(&mut self, ui: &mut Ui) {
        heading(
            ui,
            "ILLUSTRATIVE ENDOWMENT SCENARIOS",
            "Explore capacity over time.",
            "Editable assumptions. The opening CA$250M illustrates the target as if raised; it is not secured money.",
        );
        self.scenario_workspace(ui);
    }
    pub(super) fn scenario_workspace(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            let old = self.preset;
            ComboBox::from_id_salt("scenario")
                .selected_text(finance::SCENARIOS[self.preset])
                .width(280.)
                .show_ui(ui, |ui| {
                    for (i, label) in finance::SCENARIOS.iter().enumerate() {
                        ui.selectable_value(&mut self.preset, i, *label);
                    }
                });
            if old != self.preset && self.preset < 5 {
                self.inputs = finance::scenario(self.preset);
                self.selected_year = 0;
            }
            if ui.button("Reset hypothetical inputs").clicked() {
                self.inputs = Inputs::default();
                self.preset = 0;
                self.selected_year = 0;
            }
            if ui.button("Accessible scenario form").clicked() {
                open_reader();
            }
            if ui.button("Download scenario data").clicked(){
                if let Ok(p)=finance::calculate(&self.inputs){super::views::download("library-trust-scenario.json","application/json",&serde_json::to_string_pretty(&serde_json::json!({"title":TITLE,"hypothetical":true,"inputs":self.inputs,"projection":p,"rules":finance::RULES})).unwrap());}
            }
        });
        let (left, right, full) = columns(ui, 0.39);
        panel(ui, "scenario-inputs", left, |ui| {
            eyebrow(ui, "HYPOTHETICAL INPUTS / CAD");
            tabs(
                ui,
                &mut self.input_tab,
                &["Donations", "Returns", "Policy", "Horizon"],
            );
            body_scroll(ui, ("input", self.input_tab), |ui| self.scenario_inputs(ui));
        });
        panel(ui, "scenario-results", right, |ui| {
            tabs(
                ui,
                &mut self.result_tab,
                &[
                    "At a glance",
                    "Fund & value",
                    "Annual support",
                    "Year detail",
                    "Rules",
                ],
            );
            body_scroll(ui, ("result", self.result_tab), |ui| {
                self.scenario_results(ui)
            });
        });
        ui.advance_cursor_after_rect(full);
    }
    fn scenario_inputs(&mut self, ui: &mut Ui) {
        ui.spacing_mut().item_spacing.y = 6.;
        let mut changed = false;
        match self.input_tab {
            0 => {
                section(ui, "Two separate donation purposes.");
                changed |= amount(
                    ui,
                    "Opening donated long-term capital",
                    &mut self.inputs.capital,
                );
                changed |= amount(
                    ui,
                    "Additional annual long-term donations",
                    &mut self.inputs.annual_capital_donations,
                );
                changed |= amount(
                    ui,
                    "Annual donations for immediate library needs",
                    &mut self.inputs.annual_immediate_donations,
                );
                small(
                    ui,
                    "Annual gifts may be zero. Immediate-use gifts are not invested or used for fund costs. Unused amounts stay separate.",
                );
            }
            1 => {
                changed |= percent(
                    ui,
                    "Annual cash interest yield",
                    &mut self.inputs.interest_rate,
                    0.,
                    100.,
                );
                changed |= percent(
                    ui,
                    "Annual capital value change / loss",
                    &mut self.inputs.return_rate,
                    -100.,
                    100.,
                );
                changed |= percent(
                    ui,
                    "Annual investment cost rate",
                    &mut self.inputs.investment_cost_rate,
                    0.,
                    100.,
                );
                changed |= amount(
                    ui,
                    "First-year administration costs",
                    &mut self.inputs.administration,
                );
                let mut shock = self.inputs.first_year_return.is_some();
                if ui
                    .checkbox(&mut shock, "First-year capital-value shock")
                    .changed()
                {
                    self.inputs.first_year_return = if shock { Some(-0.20) } else { None };
                    changed = true;
                }
                if let Some(v) = &mut self.inputs.first_year_return {
                    changed |= percent(ui, "First-year value change", v, -100., 100.);
                }
                small(
                    ui,
                    "Hypothetical rates and costs, not quoted yields. Capital-value gains are not cash interest.",
                );
            }
            2 => {
                changed |= percent(
                    ui,
                    "Library share of positive net interest",
                    &mut self.inputs.library_interest_share,
                    0.,
                    100.,
                );
                small(
                    ui,
                    &format!(
                        "{:.1}% reinvested. Proposed policy: 50% / 50% after costs.",
                        (1. - self.inputs.library_interest_share) * 100.
                    ),
                );
                let old = self.inputs.interest_only;
                ui.radio_value(
                    &mut self.inputs.interest_only,
                    true,
                    "Proposed: share net interest",
                );
                ui.radio_value(
                    &mut self.inputs.interest_only,
                    false,
                    "Comparison: spend % of opening fund",
                );
                changed |= old != self.inputs.interest_only;
                if !self.inputs.interest_only {
                    changed |= percent(
                        ui,
                        "Comparison spending rate",
                        &mut self.inputs.spending_rate,
                        0.,
                        100.,
                    );
                }
                changed |= percent(
                    ui,
                    "Reserve floor / % of opening fund",
                    &mut self.inputs.reserve_rate,
                    0.,
                    100.,
                );
                let old = self.inputs.protect_principal;
                ui.radio_value(
                    &mut self.inputs.protect_principal,
                    true,
                    "Principal restricted from grants",
                );
                ui.radio_value(
                    &mut self.inputs.protect_principal,
                    false,
                    "Gift terms permit capital grants",
                );
                changed |= old != self.inputs.protect_principal;
                small(
                    ui,
                    "Different hypothetical gift terms cannot release a real restriction. Losses and permitted costs may erode principal.",
                );
            }
            _ => {
                section(ui, "Need, inflation and time.");
                changed |= percent(ui, "Annual inflation", &mut self.inputs.inflation, 0., 25.);
                changed |= amount(
                    ui,
                    "First-year annual library funding need",
                    &mut self.inputs.library_need,
                );
                let l = ui.label("Projection horizon / years");
                changed |= ui
                    .add(Slider::new(&mut self.inputs.years, 1..=50))
                    .labelled_by(l.id)
                    .changed();
                small(
                    ui,
                    "Need and administration rise with inflation from year 2. Equal annual donations arrive at the start of each year and remain nominally constant.",
                );
            }
        }
        if changed {
            self.preset = 5;
            self.selected_year = self.selected_year.min(self.inputs.years - 1);
        }
    }
    fn scenario_results(&mut self, ui: &mut Ui) {
        ui.spacing_mut().item_spacing.y = 6.;
        let p = match finance::calculate(&self.inputs) {
            Ok(v) => v,
            Err(e) => {
                paragraph(ui, &e);
                return;
            }
        };
        let end = p.last();
        let first = &p.years[0];
        badge(
            ui,
            if self.inputs.interest_only {
                "PROPOSED INTEREST-SHARING ILLUSTRATION"
            } else {
                "CAPITAL-SPENDING COMPARISON ONLY"
            },
            if self.inputs.interest_only { TEAL } else { RED },
        );
        match self.result_tab {
            0 => {
                let m = [
                    ("Year 1 library support", first.libraries),
                    ("Year 1 net interest reinvested", first.retained_interest),
                    ("Final nominal fund", end.closing),
                    ("Final purchasing power", end.real_closing),
                    ("Cumulative library support", end.cumulative_support),
                    ("Final-year funding gap", end.gap),
                ];
                let count = if ui.available_width() >= 640. { 3 } else { 2 };
                for row in m.chunks(count) {
                    ui.columns(count, |cols| {
                        for (col, (label, v)) in cols.iter_mut().zip(row) {
                            small(col, label);
                            col.label(
                                RichText::new(finance::compact(*v))
                                    .font(display(29.))
                                    .color(TEAL),
                            );
                        }
                    });
                    ui.add_space(4.);
                }
                if end.real_closing < end.real_contributed_principal {
                    badge(ui, "PURCHASING POWER BELOW REAL CONTRIBUTIONS", RED);
                }
                small(
                    ui,
                    "Cumulative support is nominal. No permanent funding or capital guarantee. Open Fund & value or Annual support for charts.",
                );
            }
            1 => {
                section(ui, "Fund balance & purchasing power");
                chart(ui, &p, self.inputs.capital, false);
                ui.horizontal_wrapped(|ui| {
                    ui.colored_label(TEAL, "Solid: nominal fund");
                    ui.colored_label(GOLD, "Dashed: purchasing power");
                });
                small(
                    ui,
                    "Purchasing power is measured in starting-year dollars. Reinvestment may not keep pace with inflation. Exact values appear in Year detail.",
                );
            }
            2 => {
                section(ui, "Library support & remaining need");
                chart(ui, &p, 0., true);
                ui.horizontal_wrapped(|ui| {
                    ui.colored_label(TEAL, "Solid: total support");
                    ui.colored_label(RED, "Dashed: funding gap");
                });
                small(
                    ui,
                    "Support includes endowment grants and separately accounted immediate-use grants. Neither permanent funding nor capital preservation is guaranteed.",
                );
            }
            3 => {
                self.selected_year = self.selected_year.min(p.years.len() - 1);
                let mut year = self.selected_year + 1;
                if ui
                    .add(Slider::new(&mut year, 1..=self.inputs.years).text("Projection year"))
                    .changed()
                {
                    self.selected_year = year - 1;
                }
                let y = &p.years[self.selected_year];
                let fields = [
                    ("Gross cash interest", y.interest_income),
                    ("Capital value change", y.valuation_change),
                    ("Net interest after costs", y.net_interest),
                    ("Net interest reinvested", y.retained_interest),
                    ("Endowment grant", y.endowment_distribution),
                    ("Immediate-use grant", y.immediate_distribution),
                    ("Immediate carryover", y.immediate_balance),
                    ("Closing fund", y.closing),
                    ("Purchasing power", y.real_closing),
                    ("Annual funding gap", y.gap),
                    ("Reserve target", y.reserve_target),
                    ("Reserve shortfall", y.reserve_shortfall),
                    ("Costs paid", y.costs_paid),
                    ("Unpaid costs", y.unpaid_costs),
                    ("Spending target", y.spending_target),
                    ("Policy ceiling", y.available_for_spending),
                    ("Grant from accumulated assets", y.from_accumulated_assets),
                    ("Principal used for grants", y.principal_used_for_grants),
                ];
                for row in fields.chunks(2) {
                    ui.columns(2, |cols| {
                        for (col, (label, v)) in cols.iter_mut().zip(row) {
                            col.horizontal_wrapped(|ui| {
                                ui.label(RichText::new(*label).size(13.).color(MUTED));
                                ui.label(RichText::new(finance::cad(*v)).size(14.).color(INK));
                            });
                            col.separator();
                        }
                    });
                }
            }
            _ => {
                ComboBox::from_id_salt("calculation-rule")
                    .selected_text(finance::RULES[self.rule].0)
                    .width(280.)
                    .show_ui(ui, |ui| {
                        for (i, (name, _)) in finance::RULES.iter().enumerate() {
                            ui.selectable_value(&mut self.rule, i, *name);
                        }
                    });
                let (t, b) = finance::RULES[self.rule];
                section(ui, t);
                paragraph(ui, b);
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(self.rule > 0, Button::new("Previous rule"))
                        .clicked()
                    {
                        self.rule -= 1;
                    }
                    small(ui, &format!("{} / {}", self.rule + 1, finance::RULES.len()));
                    if ui
                        .add_enabled(
                            self.rule + 1 < finance::RULES.len(),
                            Button::new("Next rule"),
                        )
                        .clicked()
                    {
                        self.rule += 1;
                    }
                });
            }
        }
    }
}
fn amount(ui: &mut Ui, label: &str, v: &mut f64) -> bool {
    let mut changed = false;
    let width = ui.available_width();
    let render = |ui: &mut Ui| {
        let l = if width > 380. {
            ui.allocate_ui_with_layout(
                vec2(width - 175., 38.),
                Layout::top_down(Align::LEFT),
                |ui| ui.add(Label::new(label).wrap()),
            )
            .inner
        } else {
            ui.label(label)
        };
        changed = ui
            .add(
                DragValue::new(v)
                    .speed(1000.)
                    .range(0. ..=1e12)
                    .custom_formatter(|n, _| finance::cad(n))
                    .custom_parser(|s| s.replace("CA$", "").replace(',', "").parse().ok())
                    .max_decimals(2),
            )
            .labelled_by(l.id)
            .changed();
    };
    if width > 380. {
        ui.horizontal(render);
    } else {
        ui.scope(render);
    }
    ui.add_space(2.);
    changed
}
fn percent(ui: &mut Ui, label: &str, v: &mut f64, min: f64, max: f64) -> bool {
    let l = ui.label(label);
    let mut p = *v * 100.;
    let changed = ui
        .add(
            DragValue::new(&mut p)
                .speed(0.1)
                .range(min..=max)
                .suffix(" %")
                .max_decimals(3),
        )
        .labelled_by(l.id)
        .changed();
    if changed {
        *v = p / 100.;
    }
    ui.add_space(4.);
    changed
}

fn chart(ui: &mut Ui, projection: &Projection, opening: f64, support: bool) {
    let (r, response) = ui.allocate_exact_size(vec2(ui.available_width(), 190.), Sense::hover());
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Image,
            true,
            if support {
                "Annual library support and gaps. Exact values in the Year detail tab."
            } else {
                "Nominal fund and purchasing power. Exact values in the Year detail tab."
            },
        )
    });
    let p = ui.painter().with_clip_rect(r);
    let plot = r.shrink2(vec2(7., 27.));
    let max = projection
        .years
        .iter()
        .map(|y| {
            if support {
                y.library_need
            } else {
                y.closing.max(y.real_closing)
            }
        })
        .fold(opening.max(1.), f64::max)
        * 1.06;
    for j in 0..=3 {
        let y = plot.bottom() - plot.height() * j as f32 / 3.;
        p.line_segment(
            [pos2(plot.left(), y), pos2(plot.right(), y)],
            Stroke::new(1., LINE),
        );
    }
    let point = |x: usize, y: f64| {
        pos2(
            plot.left() + plot.width() * x as f32 / projection.years.len() as f32,
            plot.bottom() - plot.height() * (y / max) as f32,
        )
    };
    for series in 0..2 {
        let color = if series == 0 {
            TEAL
        } else if support {
            RED
        } else {
            GOLD
        };
        let mut previous = point(0, if support { 0. } else { opening });
        for y in &projection.years {
            let amount = if support {
                if series == 0 { y.libraries } else { y.gap }
            } else if series == 0 {
                y.closing
            } else {
                y.real_closing
            };
            let next = point(y.year, amount);
            if series == 0 {
                p.line_segment([previous, next], Stroke::new(2., color));
            } else {
                let d = previous.distance(next);
                if d > 0. {
                    let v = (next - previous) / d;
                    for k in (0..d as i32).step_by(9) {
                        p.line_segment(
                            [
                                previous + v * k as f32,
                                previous + v * (k as f32 + 4.).min(d),
                            ],
                            Stroke::new(1.5, color),
                        );
                    }
                }
            }
            previous = next;
        }
    }
    p.text(
        r.left_top(),
        Align2::LEFT_TOP,
        finance::compact(max),
        FontId::proportional(12.),
        MUTED,
    );
    p.text(
        r.left_bottom(),
        Align2::LEFT_BOTTOM,
        "Start",
        FontId::proportional(12.),
        MUTED,
    );
    p.text(
        r.right_bottom(),
        Align2::RIGHT_BOTTOM,
        format!("Year {}", projection.years.len()),
        FontId::proportional(12.),
        MUTED,
    );
}
