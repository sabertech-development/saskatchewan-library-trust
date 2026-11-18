use saskatchewan_library_funding::allocation::{self, Applicant};
fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-5, "{a} != {b}");
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
    let a = allocation::allocate(6_000., 10_000., [1.; 4], &p).unwrap();
    for v in a.awards {
        near(v, 1_000.);
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
    near(a.unallocated, 5_750_000.);
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
