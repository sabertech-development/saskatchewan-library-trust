use super::*;

impl ProposalApp {
    pub(super) fn money(&mut self, ui: &mut Ui) {
        page_title(
            ui,
            "MONEY FLOW",
            "Every dollar has a sequence.",
            "An illustrative annual waterfall. Costs and reserve requirements come before the residual split.",
        );
        let c = match finance::calculate(&self.inputs) {
            Ok(c) => c,
            Err(e) => {
                badge(ui, &e, RED);
                if primary(ui, "Correct the inputs") {
                    self.navigate(4);
                }
                return;
            }
        };
        ui.horizontal_wrapped(|ui| {
            if ui.button("Show next step").clicked() {
                self.flow_step = (self.flow_step + 1).min(5);
            }
            if ui.button("Restart walkthrough").clicked() {
                self.flow_step = 0;
            }
            if ui.button("Show all").clicked() {
                self.flow_step = 5;
            }
            small(ui, &format!("Step {} of 6", self.flow_step + 1));
        });
        ui.add_space(16.);
        let stages = [
            (
                "Business cash received",
                c.received,
                "After business operating expenses, business taxes, maintenance capital expenditure and working-capital needs.",
            ),
            (
                "Vehicle management costs",
                -c.management,
                "Additional vehicle expense, calculated on opening invested business equity.",
            ),
            (
                "Vehicle administration",
                -self.inputs.administration,
                "Annual administration is an additional vehicle expense.",
            ),
            (
                "Illustrative vehicle tax provision",
                -c.tax,
                "Applied to positive cash before vehicle tax only. This is a placeholder, not Canadian tax treatment.",
            ),
            (
                "Reserve replenishment",
                -c.reserve_topup,
                "Only positive cash tops up the target. Opening reserves are not used to increase library awards.",
            ),
            (
                "Residual cash available",
                c.residual,
                "The positive cash left after the deductions above.",
            ),
        ];
        let max = c
            .received
            .max(c.management + self.inputs.administration)
            .max(1.);
        let mut running = 0.;
        for (idx, (name, amount, detail)) in stages.iter().enumerate() {
            if idx > self.flow_step {
                break;
            }
            if idx == 0 {
                running = *amount;
            } else if idx < 5 {
                running += amount;
            }
            frame().show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new(*name).size(18.));
                    ui.label(
                        RichText::new(finance::cad(*amount))
                            .color(if *amount < 0. { GOLD } else { TEAL })
                            .size(24.),
                    );
                });
                bar(
                    ui,
                    amount.abs() / max,
                    if *amount < 0. { GOLD } else { TEAL },
                    &format!("{}: {}", name, finance::cad(*amount)),
                );
                small(ui, detail);
                if idx > 0 && idx < 5 {
                    small(
                        ui,
                        &format!("Cash remaining after this step: {}", finance::cad(running)),
                    );
                }
            });
            ui.add_space(10.);
        }
        if c.deficit > 0. {
            frame().show(ui, |ui| {
                badge(ui, "OPERATING DEFICIT · NO RESIDUAL DISTRIBUTIONS", RED);
                ui.label(format!(
                    "Deficit: {} · Reserve drawn: {} · Unfunded: {}",
                    finance::cad(c.deficit),
                    finance::cad(c.reserve_used),
                    finance::cad(c.unfunded)
                ));
            });
        }
        if self.flow_step >= 5 {
            ui.add_space(20.);
            section(ui, "Divide only the remaining positive cash");
            metric_row(
                ui,
                &[
                    (
                        "Investor cash distribution",
                        finance::compact(c.investors),
                        &format!("{:.0}% of residual", self.inputs.investor_share * 100.),
                    ),
                    (
                        "Library funding",
                        finance::compact(c.libraries),
                        &format!("{:.0}% of residual", self.inputs.library_share * 100.),
                    ),
                    (
                        "Retained reinvestment",
                        finance::compact(c.reinvestment),
                        &format!("{:.0}% of residual", self.inputs.reinvestment_share * 100.),
                    ),
                ],
            );
            ui.add_space(16.);
            small(
                ui,
                "Base design assumption: 50% investors / 20% libraries / 30% reinvestment. This is not an agreed commercial arrangement. If you change shares in the lab, every view follows those custom assumptions.",
            );
        }
        ui.add_space(20.);
        section(ui, "Reconcile sources and uses");
        let sources = c.received + c.reserve_used + c.unfunded;
        let uses = c.management
            + self.inputs.administration
            + c.tax
            + c.reserve_topup
            + c.investors
            + c.libraries
            + c.reinvestment;
        paragraph(
            ui,
            &format!(
                "Business cash {} + reserve draw {} + unfunded obligation {} = total uses {}. Reconciliation difference: {}.",
                finance::cad(c.received),
                finance::cad(c.reserve_used),
                finance::cad(c.unfunded),
                finance::cad(uses),
                finance::cad(sources - uses)
            ),
        );
        small(
            ui,
            "An unfunded obligation is a shortfall, not an actual cash source. It is included here to reconcile the obligations still requiring funding.",
        );
        item(
            ui,
            "Excluded from annual cash",
            "Unrealized valuation increases, new investor subscriptions, business sale proceeds and return of investor capital are excluded. New subscriptions are not investment earnings. Retained reinvestment is not a payment to investors or libraries.",
        );
        if primary(ui, "Test these assumptions") {
            self.navigate(4);
        }
    }

    pub(super) fn lab(&mut self, ui: &mut Ui) {
        page_title(
            ui,
            "SUSTAINABILITY LAB",
            "Test the assumptions.",
            "One annual cash period. Adjust the inputs to see what is available, what is retained and what remains unfunded.",
        );
        frame().show(ui,|ui|{eyebrow(ui,"STRESS ASSUMPTIONS · NOT FORECASTS");ui.horizontal_wrapped(|ui|{let old=self.preset;ComboBox::from_id_salt("scenario").selected_text(finance::SCENARIOS[self.preset]).width(270.).show_ui(ui,|ui|{for(i,name)in finance::SCENARIOS.iter().enumerate(){ui.selectable_value(&mut self.preset,i,*name);}});if old!=self.preset&&self.preset<5{self.inputs=finance::scenario(self.preset);}if ui.button("Reset all inputs").clicked(){self.inputs=Inputs::default();self.preset=0;}});small(ui,match self.preset{1=>"Business cash yield: 8% → 4%. All other base assumptions unchanged.",2=>"Business cash yield: 8% → 0%. Vehicle costs continue; reserves may be depleted.",3=>"Management rate: 1.5% → 2.5%. Administration: CA$500,000 → CA$1,000,000.",4=>"Annual library need: CA$5,000,000 → CA$10,000,000.",5=>"Custom assumptions. Changes affect every dependent view in this browser session.",_=>"Base: CA$250M equity · CA$5M opening reserve · 8% business cash yield · 1.5% management."});});
        ui.add_space(20.);
        if ui.available_width() > 980. {
            ui.columns(2, |cols| {
                self.lab_inputs(&mut cols[0]);
                self.lab_results(&mut cols[1]);
            });
        } else {
            self.lab_results(ui);
            ui.add_space(20.);
            self.lab_inputs(ui);
        }
        ui.add_space(24.);
        CollapsingHeader::new("Inspect the exact calculation rules").show(ui,|ui|{for s in["E = total subscribed equity − opening cash reserve.","Business cash = E × business yield. Management cost = E × management rate.","Cash before vehicle tax = business cash − management − administration.","Tax provision = max(0, cash before vehicle tax) × provision rate.","Cash after tax = cash before vehicle tax − tax provision.","Operating deficit = max(0, −cash after tax). Reserve use = min(opening reserve, deficit).","Unfunded deficit = deficit − reserve use. Reserve after deficit = opening reserve − reserve use.","Top-up = min(max(0, cash after tax), max(0, target reserve − reserve after deficit)).","Residual = max(0, cash after tax − reserve top-up). Each distribution = residual × its share.","Closing reserve = reserve after deficit + top-up. Coverage = library allocation ÷ need when need is positive.","Remaining gap = max(0, annual library need − library allocation)."]{bullet(ui,s);}});
        small(
            ui,
            "All calculations stay in this browser. Inputs are not sent to a server or persisted. Investor cash yield is not total investment return or IRR. This single-period model does not estimate exit values, reinvestment compounding or future performance.",
        );
    }
    fn lab_inputs(&mut self, ui: &mut Ui) {
        frame().show(ui,|ui|{section(ui,"Capital & reserves");small(ui,"Use a slider or click an amount to enter a precise value.");let mut changed=false;
            changed|=amount_input(ui,"Total subscribed equity",&mut self.inputs.capital,0.,1_000_000_000.,1_000_000.);
            if self.inputs.reserve>self.inputs.capital{self.inputs.reserve=self.inputs.capital;self.toast="Opening reserve adjusted so it cannot exceed subscribed equity.".into();}
            changed|=amount_input(ui,"Opening cash reserve",&mut self.inputs.reserve,0.,self.inputs.capital,100_000.);
            small(ui,&format!("Opening invested business equity: {}. Total equity includes the opening reserve.",finance::cad(self.inputs.capital-self.inputs.reserve)));
            changed|=amount_input(ui,"Required reserve balance",&mut self.inputs.reserve_target,0.,50_000_000.,100_000.);
            section(ui,"Annual cash assumptions");
            changed|=rate_input(ui,"Business cash distribution yield",&mut self.inputs.yield_rate);
            small(ui,&format!("Applied to opening invested business equity: {}",finance::cad(self.inputs.capital-self.inputs.reserve)));
            changed|=rate_input(ui,"Management cost rate",&mut self.inputs.management_rate);
            small(ui,&format!("Applied to opening invested business equity: {}",finance::cad(self.inputs.capital-self.inputs.reserve)));
            changed|=amount_input(ui,"Annual administration",&mut self.inputs.administration,0.,10_000_000.,25_000.);
            changed|=rate_input(ui,"Illustrative vehicle tax provision",&mut self.inputs.tax_rate);
            small(ui,"Simplified placeholder; Canadian tax treatment remains unresolved.");
            changed|=amount_input(ui,"Illustrative annual library funding need",&mut self.inputs.library_need,0.,50_000_000.,100_000.);
            section(ui,"Residual cash shares");
            changed|=rate_input(ui,"Investor share",&mut self.inputs.investor_share);
            changed|=rate_input(ui,"Library share",&mut self.inputs.library_share);
            changed|=rate_input(ui,"Reinvestment share",&mut self.inputs.reinvestment_share);
            let sum=(self.inputs.investor_share+self.inputs.library_share+self.inputs.reinvestment_share)*100.;
            ui.label(RichText::new(format!("Total: {sum:.2}% · must equal 100%")).color(if(sum-100.).abs()<1e-7{TEAL}else{RED}));
            if changed{self.preset=5;}
        });
    }
    fn lab_results(&self, ui: &mut Ui) {
        frame().show(ui,|ui|{
            let c=match finance::calculate(&self.inputs){Ok(c)=>c,Err(e)=>{badge(ui,"RESULTS PAUSED",RED);paragraph(ui,&e);return;}};
            eyebrow(ui,"ANNUAL MODEL OUTPUT");
            if c.deficit>0.{badge(ui,"OPERATING DEFICIT",RED);}else if c.gap>0.{badge(ui,"LIBRARY FUNDING GAP REMAINS",GOLD);}else if self.inputs.library_need==0.{badge(ui,"NO LIBRARY NEED ENTERED",GOLD);}else{badge(ui,"ILLUSTRATIVE NEED COVERED IN THIS PERIOD",TEAL);}
            ui.add_space(12.);small(ui,"Available for libraries");ui.label(RichText::new(finance::cad(c.libraries)).size(38.).color(TEAL));
            let coverage=c.coverage.map(|v|format!("{:.2}% of illustrative need covered",v*100.)).unwrap_or("Coverage is not defined when need is zero".into());
            bar(ui,c.coverage.unwrap_or(0.),TEAL,&coverage);ui.label(&coverage);
            ui.add_space(10.);for(label,amount)in[("Remaining library gap",c.gap),("Investor cash distribution",c.investors),("Retained reinvestment",c.reinvestment),("Opening invested business equity",c.invested),("Business cash received",c.received),("Management costs",c.management),("Administration",self.inputs.administration),("Vehicle tax provision",c.tax),("Reserve drawn for deficit",c.reserve_used),("Reserve replenishment",c.reserve_topup),("Closing reserve",c.closing_reserve),("Unfunded operating deficit",c.unfunded)]{value_row(ui,label,&finance::cad(amount));}
            value_row(ui,"Investor cash yield on total equity",&c.investor_yield.map(|v|format!("{:.3}%",v*100.)).unwrap_or("Not defined".into()));
            if self.inputs.capital==0.{small(ui,"Zero subscribed equity: no businesses are funded. Administration costs can still create a deficit.");}
            if self.inputs.library_need==0.{small(ui,"Zero library need: coverage is not applicable. The library pool still follows the residual policy; an allocation may remain unassigned.");}
            if c.deficit>0.{small(ui,"The reserve is being used for vehicle obligations, not library payments. A deficit is not a successful sustainability result.");}
            if c.closing_reserve<self.inputs.reserve_target{small(ui,"Required reserve target is not fully funded under these assumptions.");}
            ui.add_space(10.);small(ui,"No distributions, investment returns or capital preservation are guaranteed.");
        });
    }

    pub(super) fn allocate_view(&mut self, ui: &mut Ui) {
        page_title(
            ui,
            "LIBRARY ALLOCATION",
            "One pool. A transparent method.",
            "The investment model determines how much is available. The proposed allocation method determines how that amount is divided.",
        );
        let c = match finance::calculate(&self.inputs) {
            Ok(c) => c,
            Err(e) => {
                badge(ui, &e, RED);
                if primary(ui, "Correct financial inputs") {
                    self.navigate(4);
                }
                return;
            }
        };
        let people = allocation::applicants();
        metric_row(
            ui,
            &[
                (
                    "Available library pool",
                    finance::cad(c.libraries),
                    "Live output from Sustainability Lab",
                ),
                (
                    "Synthetic applicants",
                    people.len().to_string(),
                    "No actual participation implied",
                ),
                (
                    "Total synthetic requests",
                    finance::compact(people.iter().map(|p| p.gap).sum()),
                    "Independent of the lab need input",
                ),
            ],
        );
        ui.add_space(20.);
        frame().show(ui,|ui|{section(ui,"Proposed allocation controls");amount_input(ui,"Baseline per applicant before pool adjustment",&mut self.baseline,0.,500_000.,10_000.);
            small(ui,"First, each baseline is capped at the applicant's gap. If those baselines exceed the pool, they are reduced proportionally. Remaining cash is distributed by weighted need, capped again and redistributed until needs are met or cash is exhausted.");
            ui.add_space(10.);for(i,label)in["Essential service requirement","Remoteness","Community disadvantage","Lack of alternative access"].iter().enumerate(){ui.horizontal_wrapped(|ui|{ui.label(*label);ui.add(Slider::new(&mut self.weights[i],0. ..=100.).suffix(" weight").step_by(1.));});}
            small(ui,"Applicant scores range from 0 to 5. Higher means greater need, including less alternative access. Weighted score = Σ(score × weight) ÷ total weight. If all eligible scores are zero, the remainder is shared equally, subject to caps.");
        });
        let result = match allocation::allocate(c.libraries, self.baseline, self.weights, &people) {
            Ok(a) => a,
            Err(e) => {
                badge(ui, &e, RED);
                return;
            }
        };
        ui.add_space(20.);
        badge(
            ui,
            "ALL APPLICANTS AND ALL ‘VERIFIED’ FIELDS ARE SYNTHETIC EXAMPLE DATA",
            GOLD,
        );
        for (i, a) in people.iter().enumerate() {
            frame().show(ui,|ui|{section(ui,&a.name);small(ui,&format!("{} service context · synthetic profile",a.context));bar(ui,if a.gap>0.{result.awards[i]/a.gap}else{0.},TEAL,&format!("{} awarded of {} requested",finance::cad(result.awards[i]),finance::cad(a.gap)));
            ui.horizontal_wrapped(|ui|{ui.label(format!("Requested {}",finance::cad(a.gap)));ui.label(RichText::new(format!("Award {}",finance::cad(result.awards[i]))).color(TEAL));ui.label(format!("Remaining {}",finance::cad((a.gap-result.awards[i]).max(0.))));});
            small(ui,&format!("Baseline award: {} · weighted score: {:.3} / 5",finance::cad(result.baselines[i]),result.scores[i]));
            small(ui,&format!("Score = ({:.0}×{:.0} + {:.0}×{:.0} + {:.0}×{:.0} + {:.0}×{:.0}) / {:.0}",a.essential,self.weights[0],a.remote,self.weights[1],a.disadvantage,self.weights[2],a.access,self.weights[3],self.weights.iter().sum::<f64>()));
            paragraph(ui,&format!("Proposed service outcome: {} Delivery and scope depend on the award and a recipient agreement.",a.outcome));
        });
            ui.add_space(12.);
        }
        section(ui, "Allocation table");
        small(
            ui,
            "Text table accompanies the charts. All amounts CAD; annual funding gap fields are synthetic examples of evidence an actual application would need to verify.",
        );
        ScrollArea::horizontal().show(ui, |ui| {
            Grid::new("allocation table")
                .striped(true)
                .min_col_width(110.)
                .show(ui, |ui| {
                    for h in [
                        "Synthetic applicant",
                        "Requested / gap",
                        "Baseline",
                        "Weighted score",
                        "Awarded",
                        "Remaining gap",
                    ] {
                        ui.strong(h);
                    }
                    ui.end_row();
                    for (i, a) in people.iter().enumerate() {
                        ui.label(&a.name);
                        ui.label(finance::cad(a.gap));
                        ui.label(finance::cad(result.baselines[i]));
                        ui.label(format!("{:.3}", result.scores[i]));
                        ui.label(finance::cad(result.awards[i]));
                        ui.label(finance::cad((a.gap - result.awards[i]).max(0.)));
                        ui.end_row();
                    }
                });
        });
        ui.add_space(16.);
        value_row(ui, "Unallocated balance", &finance::cad(result.unallocated));
        paragraph(
            ui,
            "Final eligibility, weights, evidence requirements and appeals procedures require consultation. The six synthetic requests total CA$5 million; changing the lab's overall need does not invent or alter individual applicant budgets.",
        );
        if primary(ui, "Inspect proposed oversight") {
            self.navigate(6);
        }
    }

    pub(super) fn governance(&mut self, ui: &mut Ui) {
        page_title(
            ui,
            "GOVERNANCE",
            "Responsibility, made visible.",
            "All roles and delegations below are proposed. Authority, independence and consent requirements must be established in governing documents.",
        );
        frame().show(ui, |ui| {
            eyebrow(ui, "SELECT A PROPOSED ROLE");
            ui.horizontal_wrapped(|ui| {
                for (i, r) in ROLES.iter().enumerate() {
                    ui.selectable_value(&mut self.role, i, r.title);
                }
            });
            ui.add_space(16.);
            let r = &ROLES[self.role];
            badge(ui, "PROPOSED ROLE", GOLD);
            section(ui, r.title);
            paragraph(ui, r.mandate);
            item(ui, "Decisions & responsibilities", r.decisions);
            item(ui, "Controls & accountability", r.controls);
        });
        ui.add_space(20.);
        section(ui, "Proposed responsibility chain");
        for (title, text) in [
            (
                "Governing board → investment committee → investment manager",
                "The board sets the mandate and reserve policy. The committee approves acquisitions within delegation. The manager oversees business operations and prepares cash forecasts.",
            ),
            (
                "Governing board → allocation committee → participating library boards",
                "Eligibility follows consultation. The committee approves awards within available cash. Library boards retain local authority and report use of funds. Authorized officers release approved payments with dual sign-off.",
            ),
            (
                "Independent auditor and professional advisers → governing board",
                "Independent financial reporting review and legal and tax assessment inform oversight. Neither substitutes for investor acceptance or recipient agreements.",
            ),
        ] {
            item(ui, title, text);
        }
        section(ui, "Controls requiring agreement");
        for (title, text) in [
            (
                "Conflicts & related parties",
                "Disclose interests; recuse conflicted members; independently approve related-party transactions; document reasons.",
            ),
            (
                "Fees & independent valuations",
                "Disclose all vehicle and management fees; obtain independent valuation evidence for acquisitions and material transactions.",
            ),
            (
                "Financial audits & reporting failures",
                "Audit annual accounts. Escalate missed reports, require remediation and apply agreed suspension provisions where appropriate.",
            ),
            (
                "Allocation appeals & recipient reporting",
                "Provide written award reasons and an independent appeal channel. Review recipient use of funds without collecting patron information.",
            ),
            (
                "Changes to the library distribution policy",
                "Define investor and governing-body consent, recipient protections and disclosure before any change. No legal protection is claimed at this stage.",
            ),
        ] {
            CollapsingHeader::new(title).show(ui, |ui| paragraph(ui, text));
        }
        if primary(ui, "Review unresolved evidence") {
            self.navigate(7);
        }
    }

    pub(super) fn evidence(&mut self, ui: &mut Ui) {
        page_title(
            ui,
            "EVIDENCE & READINESS",
            "A proposal with open questions.",
            "Inspect what is known, what is illustrative and what must be established before commitments.",
        );
        ui.horizontal_wrapped(|ui| {
            ui.label("Search evidence");
            ui.add(
                TextEdit::singleline(&mut self.search)
                    .hint_text("Capital, tax, participation…")
                    .desired_width(300.),
            );
        });
        const FILTERS: [&str; 6] = [
            "All statuses",
            "Verified external information",
            "Illustrative assumption",
            "Proposed policy",
            "Requires professional assessment",
            "Not yet secured",
        ];
        ui.horizontal_wrapped(|ui| {
            for (i, status) in FILTERS.iter().enumerate() {
                ui.selectable_value(&mut self.evidence_filter, i, *status);
            }
        });
        ui.add_space(20.);
        let query = self.search.to_lowercase();
        let mut count = 0;
        for e in EVIDENCE
            .iter()
            .filter(|e| self.evidence_filter == 0 || e.status == FILTERS[self.evidence_filter])
            .filter(|e| {
                format!("{} {} {} {}", e.claim, e.detail, e.owner, e.next)
                    .to_lowercase()
                    .contains(&query)
            })
        {
            count += 1;
            frame().show(ui, |ui| {
                badge(
                    ui,
                    e.status,
                    if e.status == "Verified external information" {
                        TEAL
                    } else {
                        GOLD
                    },
                );
                section(ui, e.claim);
                paragraph(ui, e.detail);
                item(ui, "Responsible role", e.owner);
                item(ui, "Next verification step", e.next);
                if e.source.starts_with("https://") {
                    ui.hyperlink_to(
                        if e.source.contains("saskatchewan.ca") {
                            "Source: Government of Saskatchewan"
                        } else {
                            "Source: GitHub documentation"
                        },
                        e.source,
                    );
                } else {
                    small(ui, &format!("Source: {}", e.source));
                }
                small(ui, &format!("Date checked: {}", e.date));
            });
            ui.add_space(14.);
        }
        small(ui, &format!("{count} evidence records shown."));
        if count == 0 {
            paragraph(
                ui,
                "No matching records. Try another term or choose All statuses.",
            );
        }
        ui.add_space(12.);
        small(
            ui,
            "External verification dates are shown only for sources actually checked. Model assumptions, synthetic data and unresolved professional assessments are not presented as externally verified.",
        );
    }

    pub(super) fn roadmap(&mut self, ui: &mut Ui) {
        page_title(
            ui,
            "IMPLEMENTATION ROADMAP",
            "Progress through evidence.",
            "Five proposed stages, with decision gates rather than invented launch dates. No stage is represented as completed.",
        );
        for (s, stage) in STAGES.iter().enumerate() {
            frame().show(ui, |ui| {
                eyebrow(ui, &format!("PROPOSED STAGE {} · NOT COMPLETED", s + 1));
                CollapsingHeader::new(RichText::new(stage.title).size(23.))
                    .default_open(s == 0)
                    .show(ui, |ui| {
                        item(ui, "Prerequisites", stage.prerequisite);
                        item(ui, "Deliverables", stage.deliverables);
                        item(ui, "Responsible roles", stage.owner);
                        item(ui, "Decision required to proceed", stage.gate);
                    });
            });
            ui.add_space(16.);
        }
        paragraph(
            ui,
            "Each gate may result in proceeding, revising the proposal or stopping. Capitalization and payments follow evidence and agreements; they are not implied by this demonstration.",
        );
        if primary(ui, "Open the review summary") {
            self.navigate(9);
        }
    }

    pub(super) fn summary(&mut self, ui: &mut Ui) {
        page_title(
            ui,
            "REVIEW SUMMARY",
            "A clear basis for discussion.",
            "For the library director, legal advisers, elected representatives and prospective investment partners.",
        );
        section(ui, TITLE);
        paragraph(
            ui,
            "How could a privately capitalized portfolio of operating businesses generate recurring support for libraries across Saskatchewan while providing an agreed economic return to investors?",
        );
        if let Ok(c) = finance::calculate(&self.inputs) {
            metric_row(
                ui,
                &[
                    (
                        "Illustrative subscribed equity",
                        finance::compact(self.inputs.capital),
                        "Private subscriptions only",
                    ),
                    (
                        "Annual library allocation",
                        finance::compact(c.libraries),
                        "Under current browser assumptions",
                    ),
                    (
                        "Remaining annual library gap",
                        finance::compact(c.gap),
                        "Not a verified provincial shortfall",
                    ),
                ],
            );
            ui.add_space(16.);
            paragraph(
                ui,
                &format!(
                    "Opening invested business equity: {}. Business cash yield: {:.2}%. Management cost rate: {:.2}%. Administration: {}. Vehicle tax placeholder: {:.2}%. Opening reserve: {}; target: {}. Annual need: {}.",
                    finance::cad(c.invested),
                    self.inputs.yield_rate * 100.,
                    self.inputs.management_rate * 100.,
                    finance::cad(self.inputs.administration),
                    self.inputs.tax_rate * 100.,
                    finance::cad(self.inputs.reserve),
                    finance::cad(self.inputs.reserve_target),
                    finance::cad(self.inputs.library_need)
                ),
            );
            paragraph(
                ui,
                &format!(
                    "After costs, tax and reserve replenishment: residual {}. Proposed current split {:.1}% investors / {:.1}% libraries / {:.1}% reinvestment. Investor cash: {}; reinvestment: {}; closing reserve: {}; unfunded operating deficit: {}.",
                    finance::cad(c.residual),
                    self.inputs.investor_share * 100.,
                    self.inputs.library_share * 100.,
                    self.inputs.reinvestment_share * 100.,
                    finance::cad(c.investors),
                    finance::cad(c.reinvestment),
                    finance::cad(c.closing_reserve),
                    finance::cad(c.unfunded)
                ),
            );
            ui.horizontal_wrapped(|ui|{if primary(ui,"Download review brief"){download("saskatchewan-library-review.txt","text/plain;charset=utf-8",&self.review_text(&c));self.toast="Review brief prepared with the current assumptions. Open the text file to print.".into();}if ui.button("Download model data").clicked(){let data=serde_json::json!({"title":TITLE,"status":STATUS,"inputs":self.inputs,"outputs":c,"synthetic_applicants":allocation::applicants(),"allocation_baseline":self.baseline,"allocation_weights":self.weights});download("saskatchewan-library-model.json","application/json",&serde_json::to_string_pretty(&data).unwrap());}});
            small(
                ui,
                "The review brief is a plain-text, printable alternative to the canvas interface. Downloaded figures reflect this browser session. Shared section links open the base illustration and do not transmit financial inputs.",
            );
        } else {
            badge(
                ui,
                "Financial assumptions are invalid. Correct them in Sustainability Lab before exporting.",
                RED,
            );
        }
        ui.add_space(24.);
        self.boundaries(ui);
        section(ui, "Questions for legal review");
        for s in [
            "Appropriate Canadian legal structure and tax treatment; the earlier trust name is only a working title.",
            "Investor eligibility, disclosure, economic rights, liquidity, transfer and redemption terms.",
            "How the library distribution policy is incorporated into governing agreements, and consent for changes.",
            "Conflicts, related parties, independent oversight, valuation, audit and reporting responsibilities.",
            "Library eligibility, award agreements, appeals, service authority and protection of public assets and patron information.",
            "Treatment of preferred returns, carried interest, exits and capital redemptions if introduced; each requires revised modelling.",
        ] {
            bullet(ui, s);
        }
        section(ui, "Questions for provincial discussion");
        for s in [
            "What verified service and funding gaps exist across urban, rural, northern and remote contexts?",
            "Which eligibility and allocation rules would be fair, practical and publicly explainable?",
            "How can local library authority and independence be respected while financial and service results are reported?",
        ] {
            bullet(ui, s);
        }
        section(ui, "Before commitments");
        paragraph(
            ui,
            "Investor capital is not secured. Businesses are not selected. Library participation is not agreed. Financial sustainability is not established. No government or library endorsement is documented. This demonstration is not an offering and collects no commitments.",
        );
        ui.horizontal_wrapped(|ui| {
            if ui.button("Evidence register").clicked() {
                self.navigate(7);
            }
            if ui.button("Implementation gates").clicked() {
                self.navigate(8);
            }
        });
    }
    pub fn review_text(&self, c: &Cash) -> String {
        let inputs = format!(
            "Total subscribed equity: {}\nOpening cash reserve: {}\nOpening invested business equity: {}\nBusiness cash yield: {:.2}% of invested equity\nManagement cost: {:.2}% of invested equity\nAnnual administration: {}\nVehicle tax provision: {:.2}% (placeholder)\nRequired reserve: {}\nIllustrative annual library need: {}\nResidual shares: {:.1}% investors / {:.1}% libraries / {:.1}% reinvestment",
            finance::cad(self.inputs.capital),
            finance::cad(self.inputs.reserve),
            finance::cad(c.invested),
            self.inputs.yield_rate * 100.,
            self.inputs.management_rate * 100.,
            finance::cad(self.inputs.administration),
            self.inputs.tax_rate * 100.,
            finance::cad(self.inputs.reserve_target),
            finance::cad(self.inputs.library_need),
            self.inputs.investor_share * 100.,
            self.inputs.library_share * 100.,
            self.inputs.reinvestment_share * 100.
        );
        let outputs = [
            ("Business cash received", c.received),
            ("Management costs", c.management),
            ("Administration", self.inputs.administration),
            ("Tax provision", c.tax),
            ("Reserve replenishment", c.reserve_topup),
            ("Reserve draw for operating deficit", c.reserve_used),
            ("Residual cash", c.residual),
            ("Investor distribution", c.investors),
            ("Library allocation", c.libraries),
            ("Retained reinvestment", c.reinvestment),
            ("Closing reserve", c.closing_reserve),
            ("Remaining library funding gap", c.gap),
            ("Unfunded operating deficit", c.unfunded),
        ]
        .into_iter()
        .map(|(label, value)| format!("{label}: {}", finance::cad(value)))
        .collect::<Vec<_>>()
        .join("\n");
        let mut t = format!(
            "{}\n{}\n\nPrepared for discussion, not an investment offering. No commitments are collected.\n\nPROPOSED MECHANISM\nPrivate equity subscriptions -> interests in a proposed vehicle -> ownership of operating businesses -> available cash -> investors, library agreements and retained reinvestment.\n\nCURRENT ILLUSTRATIVE INPUTS (CAD)\n{}\n\nANNUAL OUTPUTS (CAD)\n{}\n\nBOUNDARIES\nPrivate equity and privately owned business proceeds only. No borrowing, new government funding, donations, crowdfunding, sponsorship revenue or library user charges. Library assets, services and patron information are not transferred or pledged. No returns, capital preservation or payments are guaranteed. Existing library revenues only establish funding gaps.\n\nMODEL LIMITS\nBusiness cash is after business-level operating costs, taxes, maintenance capital expenditure and working capital. Vehicle costs are additional. Tax is a placeholder. New subscriptions, unrealized gains, exits and capital returns are excluded. Investor cash yield is not total return or IRR.\n\nEVIDENCE REGISTER\n",
            TITLE,
            STATUS,
            inputs,
            format!(
                "{}\nFunding need covered: {}\nInvestor cash yield on subscribed equity: {}",
                outputs,
                c.coverage
                    .map(|v| format!("{:.2}%", v * 100.))
                    .unwrap_or("Not applicable (zero need)".into()),
                c.investor_yield
                    .map(|v| format!("{:.3}%", v * 100.))
                    .unwrap_or("Not applicable (zero capital)".into())
            )
        );
        for e in EVIDENCE {
            t.push_str(&format!(
                "\n{} [{}]\n{}\nOwner: {}\nSource: {}\nChecked: {}\nNext: {}\n",
                e.claim, e.status, e.detail, e.owner, e.source, e.date, e.next
            ));
        }
        t.push_str("\nSYNTHETIC ALLOCATION EXAMPLES\nAll applicant data and verified-gap fields are synthetic. Higher scores indicate greater need.\n");
        let people = allocation::applicants();
        if let Ok(a) = allocation::allocate(c.libraries, self.baseline, self.weights, &people) {
            t.push_str(&format!(
                "Baseline {}. Weights {:?}.\n",
                finance::cad(self.baseline),
                self.weights
            ));
            for (i, p) in people.iter().enumerate() {
                t.push_str(&format!("{} | {} | Request {} | Award {} | Gap {} | Score {:.3}\nOutcome subject to awarded resources: {}\n",p.name,p.context,finance::cad(p.gap),finance::cad(a.awards[i]),finance::cad(p.gap-a.awards[i]),a.scores[i],p.outcome));
            }
            t.push_str(&format!("Unallocated: {}\n", finance::cad(a.unallocated)));
        }
        t.push_str("\nPROPOSED RESPONSIBILITIES\n");
        for r in ROLES {
            t.push_str(&format!(
                "\n{}\n{}\n{}\n{}\n",
                r.title, r.mandate, r.decisions, r.controls
            ));
        }
        t.push_str("\nREADINESS GATES\n");
        for s in STAGES {
            t.push_str(&format!(
                "\n{}\nPrerequisites: {}\nOutputs: {}\nOwner: {}\nGate: {}\n",
                s.title, s.prerequisite, s.deliverables, s.owner, s.gate
            ));
        }
        t
    }
}

fn amount_input(ui: &mut Ui, label: &str, v: &mut f64, min: f64, max: f64, step: f64) -> bool {
    let label_response = ui.label(label);
    let mut changed = false;
    ui.horizontal_wrapped(|ui| {
        changed |= ui
            .add(Slider::new(v, min..=max).show_value(false).step_by(step))
            .labelled_by(label_response.id)
            .changed();
        changed |= ui
            .add(
                DragValue::new(v)
                    .range(min..=max)
                    .speed(step / 10.)
                    .custom_formatter(|n, _| finance::cad(n))
                    .custom_parser(|s| s.replace("CA$", "").replace(',', "").trim().parse().ok())
                    .max_decimals(2),
            )
            .labelled_by(label_response.id)
            .changed();
    });
    ui.add_space(5.);
    changed
}
fn rate_input(ui: &mut Ui, label: &str, v: &mut f64) -> bool {
    let label_response = ui.label(label);
    let mut percent = *v * 100.;
    let mut changed = false;
    ui.horizontal_wrapped(|ui| {
        changed |= ui
            .add(
                Slider::new(&mut percent, 0. ..=100.)
                    .show_value(false)
                    .step_by(0.1),
            )
            .labelled_by(label_response.id)
            .changed();
        changed |= ui
            .add(
                DragValue::new(&mut percent)
                    .range(0. ..=100.)
                    .speed(0.1)
                    .suffix(" %")
                    .max_decimals(3),
            )
            .labelled_by(label_response.id)
            .changed();
    });
    if changed {
        *v = percent / 100.;
    }
    changed
}
fn value_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(label).size(15.).color(MUTED));
        ui.label(RichText::new(value).size(16.).color(INK).strong());
    });
    ui.separator();
}
fn bar(ui: &mut Ui, ratio: f64, color: Color32, label: &str) {
    let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 10.), Sense::hover());
    ui.painter().rect_filled(r, 4., LINE);
    if ratio > 0. {
        ui.painter().rect_filled(
            Rect::from_min_size(
                r.min,
                vec2(r.width() * ratio.clamp(0., 1.) as f32, r.height()),
            ),
            4.,
            color,
        );
    }
    resp.on_hover_text(label);
}
fn download(name: &str, mime: &str, content: &str) {
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
