use saskatchewan_library_funding::finance::{self, Inputs};
fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() < 1e-6 * a.abs().max(b.abs()).max(1.),
        "{a} != {b}"
    );
}
fn simple() -> Inputs {
    Inputs {
        capital: 1000.,
        interest_rate: 0.,
        interest_only: false,
        annual_capital_donations: 0.,
        annual_immediate_donations: 0.,
        return_rate: 0.10,
        investment_cost_rate: 0.01,
        administration: 10.,
        inflation: 0.,
        spending_rate: 0.05,
        reserve_rate: 0.10,
        library_need: 500.,
        years: 1,
        ..Inputs::default()
    }
}
#[test]
fn known_single_year_ledger() {
    let p = finance::calculate(&simple()).unwrap();
    let y = p.last();
    near(y.investment_return, 100.);
    near(y.costs_paid, 20.);
    near(y.spending_target, 50.);
    near(y.available_for_spending, 80.);
    near(y.endowment_distribution, 50.);
    near(y.closing, 1030.);
    near(y.gap, 450.);
    near(y.principal_used_for_grants, 0.);
}

#[test]
fn proposed_policy_shares_cash_interest_and_reinvests_the_rest() {
    let i = Inputs {
        interest_only: true,
        interest_rate: 0.10,
        return_rate: 0.,
        library_interest_share: 0.50,
        ..simple()
    };
    let p = finance::calculate(&i).unwrap();
    let y = p.last();
    near(y.interest_income, 100.);
    near(y.valuation_change, 0.);
    near(y.net_interest, 80.);
    near(y.spending_target, 40.);
    near(y.endowment_distribution, 40.);
    near(y.retained_interest, 40.);
    near(y.closing, 1040.);
    near(y.from_accumulated_assets, 0.);
    near(y.principal_used_for_grants, 0.);
}
#[test]
fn valuation_gains_cannot_masquerade_as_cash_interest() {
    let i = Inputs {
        interest_only: true,
        interest_rate: 0.,
        return_rate: 0.50,
        ..simple()
    };
    let p = finance::calculate(&i).unwrap();
    near(p.last().interest_income, 0.);
    near(p.last().valuation_change, 500.);
    near(p.last().endowment_distribution, 0.);
}
#[test]
fn interest_not_used_for_grants_is_reinvested_without_double_counting() {
    let i = Inputs {
        interest_only: true,
        interest_rate: 0.10,
        return_rate: 0.,
        library_interest_share: 0.75,
        library_need: 10.,
        ..simple()
    };
    let p = finance::calculate(&i).unwrap();
    near(p.last().spending_target, 60.);
    near(p.last().endowment_distribution, 10.);
    near(p.last().retained_interest, 70.);
    near(p.last().closing, 1070.);
}
#[test]
fn interest_policy_never_pays_more_than_net_cash_interest() {
    for share in [0., 0.25, 0.5, 1.] {
        for interest in [0., 0.005, 0.10] {
            for loss in [-0.5, 0., 0.20] {
                let i = Inputs {
                    interest_only: true,
                    interest_rate: interest,
                    return_rate: loss,
                    library_interest_share: share,
                    years: 20,
                    ..simple()
                };
                let p = finance::calculate(&i).unwrap();
                for y in p.years {
                    assert!(y.endowment_distribution <= y.net_interest * share + 1e-6);
                    near(
                        y.endowment_distribution + y.retained_interest,
                        y.net_interest,
                    );
                    near(y.principal_used_for_grants, 0.);
                }
            }
        }
    }
}
#[test]
fn timing_does_not_spend_new_gifts_as_a_rate_base() {
    let p = finance::calculate(&Inputs {
        capital: 0.,
        annual_capital_donations: 1000.,
        annual_immediate_donations: 0.,
        return_rate: 0.,
        investment_cost_rate: 0.,
        administration: 0.,
        spending_rate: 0.10,
        protect_principal: false,
        years: 2,
        ..simple()
    })
    .unwrap();
    near(p.years[0].spending_target, 0.);
    near(p.years[0].closing, 1000.);
    near(p.years[1].spending_target, 100.);
    near(p.years[1].closing, 1900.);
}
#[test]
fn principal_restriction_blocks_grants_from_capital() {
    let mut i = Inputs {
        return_rate: 0.,
        investment_cost_rate: 0.,
        administration: 0.,
        ..simple()
    };
    near(
        finance::calculate(&i)
            .unwrap()
            .last()
            .endowment_distribution,
        0.,
    );
    i.protect_principal = false;
    let p = finance::calculate(&i).unwrap();
    near(p.last().endowment_distribution, 50.);
    near(p.last().principal_used_for_grants, 50.);
    near(p.last().from_accumulated_assets, 50.);
}
#[test]
fn returns_are_not_automatically_distributed() {
    let p = finance::calculate(&Inputs {
        return_rate: 0.50,
        ..simple()
    })
    .unwrap();
    near(p.last().investment_return, 500.);
    near(p.last().endowment_distribution, 50.);
    assert!(p.last().closing > 1000.);
}
#[test]
fn reserve_caps_spending_and_is_not_added_as_an_asset() {
    let p = finance::calculate(&Inputs {
        protect_principal: false,
        reserve_rate: 0.99,
        return_rate: 0.,
        investment_cost_rate: 0.,
        administration: 0.,
        ..simple()
    })
    .unwrap();
    near(p.last().reserve_target, 990.);
    near(p.last().endowment_distribution, 10.);
    near(p.last().closing, 990.);
}
#[test]
fn immediate_gifts_never_pay_losses_or_costs() {
    let p = finance::calculate(&Inputs {
        return_rate: -1.,
        administration: 500.,
        annual_immediate_donations: 80.,
        ..simple()
    })
    .unwrap();
    let y = p.last();
    near(y.closing, 0.);
    near(y.costs_paid, 0.);
    near(y.unpaid_costs, 510.);
    near(y.endowment_distribution, 0.);
    near(y.immediate_distribution, 80.);
    near(y.gap, 420.);
}
#[test]
fn unused_immediate_gifts_remain_in_their_own_ledger() {
    let p = finance::calculate(&Inputs {
        annual_immediate_donations: 600.,
        years: 2,
        ..simple()
    })
    .unwrap();
    near(p.years[0].immediate_balance, 100.);
    near(p.years[1].immediate_balance, 200.);
    near(p.years[0].endowment_distribution, 0.);
    near(p.years[0].closing, 1080.);
}
#[test]
fn no_donation_preset_sets_both_sources_to_zero() {
    let i = finance::scenario(1);
    assert_eq!(i.annual_capital_donations, 0.);
    assert_eq!(i.annual_immediate_donations, 0.);
    let p = finance::calculate(&i).unwrap();
    for y in p.years {
        near(y.capital_donations, 0.);
        near(y.immediate_donations, 0.);
    }
}
#[test]
fn inflation_changes_need_costs_and_real_value() {
    let i = Inputs {
        inflation: 0.10,
        years: 2,
        ..simple()
    };
    let p = finance::calculate(&i).unwrap();
    near(p.years[0].real_closing, 1030. / 1.1);
    near(p.years[1].library_need, 550.);
    near(p.years[1].costs_due, 10.3 + 11.);
    near(p.years[1].real_closing, p.years[1].closing / 1.21);
}
#[test]
fn shock_applies_only_to_first_year() {
    let i = Inputs {
        first_year_return: Some(-0.20),
        years: 3,
        ..simple()
    };
    let p = finance::calculate(&i).unwrap();
    near(p.years[0].return_rate, -0.20);
    near(p.years[1].return_rate, 0.10);
    assert!(p.years[0].closing < 1000.);
    near(p.years[0].endowment_distribution, 0.);
}
#[test]
fn zero_capital_and_zero_need_do_not_divide_by_zero() {
    let p = finance::calculate(&Inputs {
        capital: 0.,
        annual_capital_donations: 0.,
        annual_immediate_donations: 0.,
        library_need: 0.,
        ..simple()
    })
    .unwrap();
    near(p.last().closing, 0.);
    near(p.last().gap, 0.);
    near(p.last().unpaid_costs, 10.);
}
#[test]
fn invalid_and_extreme_inputs_are_rejected() {
    for i in [
        Inputs {
            capital: f64::NAN,
            ..simple()
        },
        Inputs {
            return_rate: -1.01,
            ..simple()
        },
        Inputs {
            inflation: -0.01,
            ..simple()
        },
        Inputs {
            reserve_rate: 1.1,
            ..simple()
        },
        Inputs {
            years: 0,
            ..simple()
        },
        Inputs {
            years: 51,
            ..simple()
        },
        Inputs {
            first_year_return: Some(f64::INFINITY),
            ..simple()
        },
        Inputs {
            annual_immediate_donations: -1.,
            ..simple()
        },
        Inputs {
            capital: 1e12,
            return_rate: 1.,
            years: 50,
            ..simple()
        },
    ] {
        assert!(finance::calculate(&i).is_err());
    }
}
#[test]
fn annual_and_cumulative_accounts_reconcile_across_stresses() {
    for rate in [-1., -0.25, 0., 0.01, 0.10] {
        for protected in [true, false] {
            for immediate in [0., 100., 1000.] {
                let i = Inputs {
                    return_rate: rate,
                    protect_principal: protected,
                    annual_immediate_donations: immediate,
                    annual_capital_donations: 25.,
                    inflation: 0.03,
                    years: 30,
                    ..simple()
                };
                let p = finance::calculate(&i).unwrap();
                let mut prior_immediate = 0.;
                let mut cumulative = 0.;
                let mut prior_closing = i.capital;
                for y in p.years {
                    near(y.opening, prior_closing);
                    near(
                        y.closing + y.costs_paid + y.endowment_distribution,
                        y.opening + y.capital_donations + y.investment_return,
                    );
                    near(
                        y.immediate_balance + y.immediate_distribution,
                        prior_immediate + y.immediate_donations,
                    );
                    near(y.costs_paid + y.unpaid_costs, y.costs_due);
                    cumulative += y.libraries;
                    near(y.cumulative_support, cumulative);
                    near(y.libraries + y.gap, y.library_need);
                    assert!(y.endowment_distribution <= y.spending_target + 1e-6);
                    assert!(y.endowment_distribution <= y.available_for_spending + 1e-6);
                    assert!(y.closing >= 0. && y.immediate_balance >= 0.);
                    if protected {
                        near(y.principal_used_for_grants, 0.);
                    }
                    prior_immediate = y.immediate_balance;
                    prior_closing = y.closing;
                }
            }
        }
    }
}
