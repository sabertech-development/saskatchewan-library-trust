use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub capital: f64,
    pub reserve: f64,
    pub yield_rate: f64,
    pub management_rate: f64,
    pub administration: f64,
    pub tax_rate: f64,
    pub reserve_target: f64,
    pub library_need: f64,
    pub investor_share: f64,
    pub library_share: f64,
    pub reinvestment_share: f64,
}
impl Default for Inputs {
    fn default() -> Self {
        Self {
            capital: 250_000_000.,
            reserve: 5_000_000.,
            yield_rate: 0.08,
            management_rate: 0.015,
            administration: 500_000.,
            tax_rate: 0.20,
            reserve_target: 10_000_000.,
            library_need: 5_000_000.,
            investor_share: 0.50,
            library_share: 0.20,
            reinvestment_share: 0.30,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cash {
    pub invested: f64,
    pub received: f64,
    pub management: f64,
    pub before_tax: f64,
    pub tax: f64,
    pub after_tax: f64,
    pub deficit: f64,
    pub reserve_used: f64,
    pub unfunded: f64,
    pub reserve_topup: f64,
    pub residual: f64,
    pub investors: f64,
    pub libraries: f64,
    pub reinvestment: f64,
    pub closing_reserve: f64,
    pub coverage: Option<f64>,
    pub gap: f64,
    pub investor_yield: Option<f64>,
}
pub fn calculate(i: &Inputs) -> Result<Cash, String> {
    for (name, v) in [
        ("Subscribed equity", i.capital),
        ("Opening reserve", i.reserve),
        ("Administration", i.administration),
        ("Reserve target", i.reserve_target),
        ("Library need", i.library_need),
    ] {
        if !v.is_finite() || v < 0. {
            return Err(format!("{name} must be a finite, nonnegative amount."));
        }
    }
    if i.reserve > i.capital {
        return Err("Opening reserve cannot exceed subscribed equity.".into());
    }
    for (name, v) in [
        ("Business yield", i.yield_rate),
        ("Management rate", i.management_rate),
        ("Tax provision", i.tax_rate),
        ("Investor share", i.investor_share),
        ("Library share", i.library_share),
        ("Reinvestment share", i.reinvestment_share),
    ] {
        if !v.is_finite() || !(0. ..=1.).contains(&v) {
            return Err(format!("{name} must be between 0% and 100%."));
        }
    }
    if (i.investor_share + i.library_share + i.reinvestment_share - 1.).abs() > 1e-9 {
        return Err(
            "Residual shares must total 100%. Adjust all three shares before results resume."
                .into(),
        );
    }
    let invested = i.capital - i.reserve;
    let received = invested * i.yield_rate;
    let management = invested * i.management_rate;
    let before_tax = received - management - i.administration;
    let tax = before_tax.max(0.) * i.tax_rate;
    let after_tax = before_tax - tax;
    let deficit = (-after_tax).max(0.);
    let reserve_used = i.reserve.min(deficit);
    let unfunded = deficit - reserve_used;
    let after_deficit = i.reserve - reserve_used;
    let reserve_topup = after_tax
        .max(0.)
        .min((i.reserve_target - after_deficit).max(0.));
    let residual = (after_tax - reserve_topup).max(0.);
    let libraries = residual * i.library_share;
    let result = Cash {
        invested,
        received,
        management,
        before_tax,
        tax,
        after_tax,
        deficit,
        reserve_used,
        unfunded,
        reserve_topup,
        residual,
        investors: residual * i.investor_share,
        libraries,
        reinvestment: residual * i.reinvestment_share,
        closing_reserve: after_deficit + reserve_topup,
        coverage: (i.library_need > 0.).then(|| libraries / i.library_need),
        gap: (i.library_need - libraries).max(0.),
        investor_yield: (i.capital > 0.).then(|| residual * i.investor_share / i.capital),
    };
    if [
        result.received,
        result.before_tax,
        result.after_tax,
        result.residual,
        result.closing_reserve,
    ]
    .iter()
    .any(|v| !v.is_finite())
    {
        return Err("Amounts exceed the calculator's numeric range.".into());
    }
    Ok(result)
}
pub const SCENARIOS: [&str; 6] = [
    "Base illustration",
    "Lower business distributions",
    "No business distributions",
    "Higher vehicle costs",
    "Higher library funding needs",
    "Custom",
];
pub fn scenario(index: usize) -> Inputs {
    let mut i = Inputs::default();
    match index {
        1 => i.yield_rate = 0.04,
        2 => i.yield_rate = 0.,
        3 => {
            i.management_rate = 0.025;
            i.administration = 1_000_000.;
        }
        4 => i.library_need = 10_000_000.,
        _ => (),
    };
    i
}
pub fn cad(v: f64) -> String {
    let raw = format!("{:.0}", v.abs());
    let mut grouped = String::new();
    for (n, c) in raw.chars().enumerate() {
        if n > 0 && (raw.len() - n) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(c);
    }
    format!("{}CA${grouped}", if v < 0. { "−" } else { "" })
}
pub fn compact(v: f64) -> String {
    if v.abs() >= 1_000_000. {
        format!("CA${:.2}M", v / 1_000_000.)
    } else {
        cad(v)
    }
}
