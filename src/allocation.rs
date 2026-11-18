use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Applicant {
    pub name: String,
    pub context: String,
    pub gap: f64,
    pub essential: f64,
    pub remote: f64,
    pub disadvantage: f64,
    pub access: f64,
    pub outcome: String,
}
pub fn applicants() -> Vec<Applicant> {
    [
        (
            "Example Urban Library A",
            "Urban",
            65_000.,
            4.,
            1.,
            4.,
            2.,
            "Retain evening opening and public computer access.",
        ),
        (
            "Example Rural Branch A",
            "Rural",
            32_500.,
            4.,
            3.,
            3.,
            4.,
            "Maintain weekly opening hours and interlibrary delivery.",
        ),
        (
            "Example Rural Branch B",
            "Rural",
            22_500.,
            3.,
            3.,
            2.,
            3.,
            "Support shared staffing and accessible reading services.",
        ),
        (
            "Example Northern Library A",
            "Northern",
            50_000.,
            5.,
            4.,
            4.,
            5.,
            "Maintain local staffing and community-selected programming.",
        ),
        (
            "Example Remote Branch A",
            "Remote",
            45_000.,
            5.,
            5.,
            5.,
            5.,
            "Maintain essential access and reliable material delivery.",
        ),
        (
            "Example Regional Service A",
            "Mixed / regional",
            35_000.,
            4.,
            3.,
            3.,
            4.,
            "Support mobile services and accessible formats.",
        ),
    ]
    .into_iter()
    .map(
        |(name, context, gap, essential, remote, disadvantage, access, outcome)| Applicant {
            name: name.into(),
            context: context.into(),
            gap,
            essential,
            remote,
            disadvantage,
            access,
            outcome: outcome.into(),
        },
    )
    .collect()
}
#[derive(Clone, Debug)]
pub struct Allocation {
    pub awards: Vec<f64>,
    pub baselines: Vec<f64>,
    pub scores: Vec<f64>,
    pub unallocated: f64,
}
pub fn allocate(
    pool: f64,
    baseline: f64,
    weights: [f64; 4],
    people: &[Applicant],
) -> Result<Allocation, String> {
    if !pool.is_finite() || pool < 0. || !baseline.is_finite() || baseline < 0. {
        return Err("Pool and baseline must be finite, nonnegative amounts.".into());
    }
    if weights.iter().any(|v| !v.is_finite() || *v < 0.) || weights.iter().sum::<f64>() <= 0. {
        return Err("Use nonnegative weights with a positive total.".into());
    }
    if people.iter().any(|a| {
        !a.gap.is_finite()
            || a.gap < 0.
            || [a.essential, a.remote, a.disadvantage, a.access]
                .iter()
                .any(|s| !s.is_finite() || !(0. ..=5.).contains(s))
    }) {
        return Err("Applicant gaps and scores are invalid.".into());
    }
    let sum_w = weights.iter().sum::<f64>();
    let scores: Vec<f64> = people
        .iter()
        .map(|a| {
            [a.essential, a.remote, a.disadvantage, a.access]
                .iter()
                .zip(weights)
                .map(|(s, w)| s * w)
                .sum::<f64>()
                / sum_w
        })
        .collect();
    let mut awards: Vec<f64> = people.iter().map(|a| baseline.min(a.gap)).collect();
    let sum = awards.iter().sum::<f64>();
    if sum > pool {
        for v in &mut awards {
            *v *= pool / sum;
        }
    }
    let baselines = awards.clone();
    let mut remainder = (pool - awards.iter().sum::<f64>()).max(0.);
    for _ in 0..people.len() + 1 {
        if remainder < 1e-7 {
            break;
        }
        let active: Vec<usize> = people
            .iter()
            .enumerate()
            .filter(|(j, a)| a.gap - awards[*j] > 1e-7)
            .map(|(j, _)| j)
            .collect();
        if active.is_empty() {
            break;
        }
        let total: f64 = active.iter().map(|j| scores[*j]).sum();
        let mut used = 0.;
        for j in &active {
            let proportion = if total > 0. {
                scores[*j] / total
            } else {
                1. / active.len() as f64
            };
            let amount = (remainder * proportion).min(people[*j].gap - awards[*j]);
            awards[*j] += amount;
            used += amount;
        }
        remainder = (remainder - used).max(0.);
    }
    Ok(Allocation {
        awards,
        baselines,
        scores,
        unallocated: remainder,
    })
}
