use saskatchewan_library_funding::{
    allocation::{self, Applicant},
    finance::{self, Inputs},
};
fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-5, "{a} != {b}");
}
#[test]
fn base_waterfall_and_denominators() {
    let c = finance::calculate(&Inputs::default()).unwrap();
    near(c.invested, 245_000_000.);
    near(c.received, 19_600_000.);
    near(c.management, 3_675_000.);
    near(c.tax, 3_085_000.);
    near(c.reserve_topup, 5_000_000.);
    near(c.residual, 7_340_000.);
    near(c.investors, 3_670_000.);
    near(c.libraries, 1_468_000.);
    near(c.reinvestment, 2_202_000.);
    near(c.closing_reserve, 10_000_000.);
    near(c.coverage.unwrap(), 0.2936);
    near(c.investor_yield.unwrap(), 0.01468);
}
#[test]
fn zero_income_reserve_draw_never_funds_awards() {
    let c = finance::calculate(&finance::scenario(2)).unwrap();
    near(c.reserve_used, 4_175_000.);
    near(c.closing_reserve, 825_000.);
    near(c.libraries, 0.);
    near(c.investors, 0.);
    near(c.tax, 0.);
}
#[test]
fn lower_yield_replenishes_without_distributing() {
    let c = finance::calculate(&finance::scenario(1)).unwrap();
    near(c.reserve_topup, 4_500_000.);
    near(c.closing_reserve, 9_500_000.);
    near(c.libraries, 0.);
}
#[test]
fn unfunded_deficit_is_visible() {
    let i = Inputs {
        reserve: 500_000.,
        yield_rate: 0.,
        ..Inputs::default()
    };
    let c = finance::calculate(&i).unwrap();
    near(c.unfunded, 3_742_500.);
    near(c.closing_reserve, 0.);
    near(c.residual, 0.);
}
#[test]
fn reserve_above_target_is_not_released() {
    let i = Inputs {
        reserve: 20_000_000.,
        ..Inputs::default()
    };
    let c = finance::calculate(&i).unwrap();
    near(c.closing_reserve, 20_000_000.);
    near(c.reserve_topup, 0.);
}
#[test]
fn zero_capital_and_zero_need() {
    let i = Inputs {
        capital: 0.,
        reserve: 0.,
        library_need: 0.,
        ..Inputs::default()
    };
    let c = finance::calculate(&i).unwrap();
    assert!(c.coverage.is_none());
    assert!(c.investor_yield.is_none());
    near(c.unfunded, 500_000.);
}
#[test]
fn rejects_invalid_values() {
    for i in [
        Inputs {
            capital: -1.,
            ..Inputs::default()
        },
        Inputs {
            reserve: 300_000_000.,
            ..Inputs::default()
        },
        Inputs {
            tax_rate: 1.1,
            ..Inputs::default()
        },
        Inputs {
            library_share: 0.4,
            ..Inputs::default()
        },
        Inputs {
            administration: f64::NAN,
            ..Inputs::default()
        },
        Inputs {
            capital: f64::INFINITY,
            ..Inputs::default()
        },
    ] {
        assert!(finance::calculate(&i).is_err());
    }
}
#[test]
fn all_scenarios_reconcile() {
    for i in 0..5 {
        let input = finance::scenario(i);
        let c = finance::calculate(&input).unwrap();
        near(
            c.received + c.reserve_used + c.unfunded,
            c.management
                + input.administration
                + c.tax
                + c.reserve_topup
                + c.investors
                + c.libraries
                + c.reinvestment,
        );
        assert!(
            [c.libraries, c.investors, c.reinvestment, c.closing_reserve]
                .iter()
                .all(|v| *v >= 0.)
        );
    }
}
#[test]
fn financial_invariants_across_stresses() {
    for capital in [0., 1_000_000., 250_000_000.] {
        for y in [0., 0.01, 0.04, 0.08, 0.2] {
            for reserve in [0., capital * 0.02, capital] {
                let i = Inputs {
                    capital,
                    reserve,
                    yield_rate: y,
                    ..Inputs::default()
                };
                let c = finance::calculate(&i).unwrap();
                near(
                    c.received + c.reserve_used + c.unfunded,
                    c.management
                        + i.administration
                        + c.tax
                        + c.reserve_topup
                        + c.investors
                        + c.libraries
                        + c.reinvestment,
                );
                near(
                    reserve - c.reserve_used + c.reserve_topup,
                    c.closing_reserve,
                );
                assert!(c.libraries >= 0.);
                assert!(c.reserve_used <= reserve);
                if c.deficit > 0. {
                    near(c.residual, 0.);
                }
            }
        }
    }
}
#[test]
fn allocation_never_exceeds_pool_or_gap() {
    let p = allocation::applicants();
    for pool in [0., 1., 100_000., 1_468_000., 5_000_000., 10_000_000.] {
        let a = allocation::allocate(pool, 100_000., [40., 25., 20., 15.], &p).unwrap();
        near(a.awards.iter().sum::<f64>() + a.unallocated, pool);
        for (i, v) in a.awards.iter().enumerate() {
            assert!(*v <= p[i].gap + 1e-6);
            assert!(*v >= 0.);
        }
        assert!(a.baselines.iter().sum::<f64>() <= pool + 1e-6);
    }
}
#[test]
fn insufficient_pool_scales_baselines() {
    let p = allocation::applicants();
    let a = allocation::allocate(60_000., 100_000., [1.; 4], &p).unwrap();
    for v in a.awards {
        near(v, 10_000.);
    }
}
#[test]
fn capped_weighted_award_redistributes() {
    let mut p = allocation::applicants();
    p.truncate(2);
    p[0].gap = 20.;
    p[1].gap = 200.;
    let a = allocation::allocate(180., 0., [1.; 4], &p).unwrap();
    near(a.awards[0], 20.);
    near(a.awards[1], 160.);
    near(a.unallocated, 0.);
}
#[test]
fn excess_pool_remains_unallocated() {
    let p = allocation::applicants();
    let a = allocation::allocate(6_000_000., 100_000., [1.; 4], &p).unwrap();
    near(a.unallocated, 1_000_000.);
}
#[test]
fn allocation_rejects_invalid_inputs() {
    let p = allocation::applicants();
    assert!(allocation::allocate(-1., 0., [1.; 4], &p).is_err());
    assert!(allocation::allocate(1., 0., [0.; 4], &p).is_err());
    assert!(allocation::allocate(1., 0., [f64::NAN; 4], &p).is_err());
}
#[test]
fn zero_scores_split_equally_and_empty_pool_is_safe() {
    let p = vec![
        Applicant {
            name: "A".into(),
            context: "synthetic".into(),
            gap: 100.,
            essential: 0.,
            remote: 0.,
            disadvantage: 0.,
            access: 0.,
            outcome: "test".into()
        };
        2
    ];
    let a = allocation::allocate(100., 0., [1.; 4], &p).unwrap();
    near(a.awards[0], 50.);
    near(a.awards[1], 50.);
    let e = allocation::allocate(100., 0., [1.; 4], &[]).unwrap();
    near(e.unallocated, 100.);
}
