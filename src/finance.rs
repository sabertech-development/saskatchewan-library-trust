use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Inputs {
    pub capital: f64,
    pub annual_capital_donations: f64,
    pub annual_immediate_donations: f64,
    pub interest_rate: f64,
    pub library_interest_share: f64,
    pub interest_only: bool,
    pub return_rate: f64,
    pub first_year_return: Option<f64>,
    pub investment_cost_rate: f64,
    pub administration: f64,
    pub inflation: f64,
    pub spending_rate: f64,
    pub reserve_rate: f64,
    pub protect_principal: bool,
    pub library_need: f64,
    pub years: usize,
}
impl Default for Inputs {
    fn default() -> Self {
        Self {
            capital: 250_000_000.,
            annual_capital_donations: 0.,
            annual_immediate_donations: 0.,
            interest_rate: 0.03,
            library_interest_share: 0.50,
            interest_only: true,
            return_rate: 0.,
            first_year_return: None,
            investment_cost_rate: 0.005,
            administration: 100_000.,
            inflation: 0.02,
            spending_rate: 0.035,
            reserve_rate: 0.10,
            protect_principal: true,
            library_need: 5_000_000.,
            years: 20,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Year {
    pub year: usize,
    pub opening: f64,
    pub capital_donations: f64,
    pub immediate_donations: f64,
    pub return_rate: f64,
    pub interest_income: f64,
    pub valuation_change: f64,
    pub net_interest: f64,
    pub retained_interest: f64,
    pub investment_return: f64,
    pub costs_due: f64,
    pub costs_paid: f64,
    pub unpaid_costs: f64,
    pub spending_target: f64,
    pub reserve_target: f64,
    pub reserve_shortfall: f64,
    pub contributed_principal: f64,
    pub available_for_spending: f64,
    pub endowment_distribution: f64,
    pub immediate_distribution: f64,
    pub immediate_balance: f64,
    pub libraries: f64,
    pub cumulative_support: f64,
    pub closing: f64,
    pub real_closing: f64,
    pub real_contributed_principal: f64,
    pub library_need: f64,
    pub gap: f64,
    pub from_accumulated_assets: f64,
    pub principal_used_for_grants: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Projection {
    pub years: Vec<Year>,
}
impl Projection {
    pub fn last(&self) -> &Year {
        self.years.last().expect("validated nonempty projection")
    }
}

pub const RULES: &[(&str, &str)] = &[
    (
        "Target and hypothetical inputs",
        "The proposed corporate-donation fundraising target is CA$250 million of principal. It is not secured capital. The starting scenario illustrates that target as if already funded; it does not report actual receipts. Every amount and rate is editable. The proposed interest-sharing policy is 50% for libraries and 50% reinvested after costs. Rates and costs are hypothetical, not quoted market rates; edits to the split are scenario comparisons.",
    ),
    (
        "Timing and separate gifts",
        "Opening capital is the balance before year 1. Equal annual donations arrive at the start of each year, including year 1. Donations do not automatically grow with inflation. Immediate-use gifts stay in a separate, uninvested ledger and never fund investment or administration costs.",
    ),
    (
        "Interest and capital value",
        "Gross cash interest = (opening long-term balance + new long-term gifts) × assumed interest yield. Capital value change uses the same base × the separate value-change assumption. The optional first-year shock changes capital value in year 1 only. Total investment return is interest plus capital value change; unrealized gains are not treated as spendable interest.",
    ),
    (
        "Costs",
        "Investment costs = (opening balance + new long-term gifts) × cost rate. Administration rises with inflation from year 2. Costs are paid from long-term assets up to the available balance; unpaid costs are reported. The model assumes permitted cost charges under gift terms. Net interest = max(0, gross cash interest − all costs due). No tax treatment is assumed.",
    ),
    (
        "Proposed interest-sharing policy",
        "The library budget = the editable library percentage × positive net interest after investment and administration costs. Actual grants are further capped by eligible need and assets above the reserve and donor-restriction floors. The remaining net interest is reinvested in the fund. No principal is distributed to donors.",
    ),
    (
        "Reinvestment",
        "Net interest retained = max(0, net interest − actual endowment grants). This includes the reinvestment share and any library budget withheld for reserves, restrictions or lack of eligible need. Retained interest stays inside the closing fund balance; it is not added a second time. Capital losses can outweigh retained interest.",
    ),
    (
        "Reserve policy",
        "The retained reserve floor is the reserve rate × opening long-term balance. It is an accounting retention floor within the fund, not a separate cash account. It is not added to fund assets a second time. Losses or costs can leave the target underfunded.",
    ),
    (
        "Principal restrictions",
        "With principal protection selected, grants cannot reduce assets below cumulative long-term contributions. The retained floor is the greater of that contribution basis and the reserve target; these floors overlap. Losses and permitted costs may still erode principal. Selecting different hypothetical gift terms does not release an actual donor restriction.",
    ),
    (
        "Capital-spending comparison only",
        "An optional comparison uses spending rate × opening annual balance, before that year's donations or return. This is not the proposed interest-only model. It can spend previously accumulated gains and, if gift terms allow, contributed principal. The interface labels this as a comparison, never the default proposal.",
    ),
    (
        "Immediate library support",
        "Immediate-use gifts and their carryover first meet that year's eligible need. Endowment grants then meet remaining need within the policy limits. Excess immediate money keeps its original restrictions; it is never moved into long-term capital. The example assumes all immediate gifts permit these same eligible needs.",
    ),
    (
        "Liquidity and uncertainty",
        "The interest illustration assumes interest is received in cash and available on the annual schedule. Total returns and capital value are separate. Actual instruments, settlement, liquidity, risks, costs and legal requirements need professional review. Money-market-style investing does not establish a guaranteed interest rate or guaranteed principal.",
    ),
    (
        "Purchasing power",
        "Annual need and administration rise with inflation from year 2. Closing purchasing power = closing fund ÷ (1 + inflation)^year, in starting-year dollars. The real contribution benchmark values gifts at their arrival dates. Cumulative support is nominal. Remaining gap = max(0, need − actual support). Reinvestment does not guarantee that the fund keeps pace with inflation.",
    ),
];
pub fn calculate(i: &Inputs) -> Result<Projection, String> {
    for (name, value) in [
        ("Opening donated capital", i.capital),
        ("Annual long-term donations", i.annual_capital_donations),
        (
            "Annual immediate-use donations",
            i.annual_immediate_donations,
        ),
        ("Administration", i.administration),
        ("Library need", i.library_need),
    ] {
        if !value.is_finite() || !(0. ..=1e12).contains(&value) {
            return Err(format!("{name} must be between zero and CA$1 trillion."));
        }
    }
    for (name, value, min, max) in [
        ("Interest yield", i.interest_rate, 0., 1.),
        (
            "Library share of net interest",
            i.library_interest_share,
            0.,
            1.,
        ),
        ("Capital value change", i.return_rate, -1., 1.),
        ("Investment costs", i.investment_cost_rate, 0., 1.),
        ("Inflation", i.inflation, 0., 0.25),
        ("Spending rate", i.spending_rate, 0., 1.),
        ("Reserve rate", i.reserve_rate, 0., 1.),
    ] {
        if !value.is_finite() || !(min..=max).contains(&value) {
            return Err(format!(
                "{name} must be between {:.0}% and {:.0}%.",
                min * 100.,
                max * 100.
            ));
        }
    }
    if i.first_year_return
        .is_some_and(|v| !v.is_finite() || !(-1. ..=1.).contains(&v))
    {
        return Err("First-year performance must be between −100% and 100%.".into());
    }
    if !(1..=50).contains(&i.years) {
        return Err("Use a horizon from 1 to 50 years.".into());
    }
    let mut balance = i.capital;
    let mut basis = i.capital;
    let mut real_basis = i.capital;
    let mut immediate = 0.;
    let mut cumulative = 0.;
    let mut years = Vec::with_capacity(i.years);
    for year in 1..=i.years {
        let price_start = (1. + i.inflation).powi(year as i32 - 1);
        let opening = balance;
        let invested = opening + i.annual_capital_donations;
        basis += i.annual_capital_donations;
        real_basis += i.annual_capital_donations / price_start;
        let rate = if year == 1 {
            i.first_year_return.unwrap_or(i.return_rate)
        } else {
            i.return_rate
        };
        let interest_income = invested * i.interest_rate;
        let valuation_change = invested * rate;
        let investment_return = interest_income + valuation_change;
        let before_costs = (invested + investment_return).max(0.);
        let costs_due = invested * i.investment_cost_rate + i.administration * price_start;
        let costs_paid = costs_due.min(before_costs);
        let unpaid_costs = costs_due - costs_paid;
        let after_costs = before_costs - costs_paid;
        let net_interest = (interest_income - costs_due).max(0.);
        let spending_target = if i.interest_only {
            net_interest * i.library_interest_share
        } else {
            opening * i.spending_rate
        };
        let reserve_target = opening * i.reserve_rate;
        let floor = reserve_target.max(if i.protect_principal { basis } else { 0. });
        let available_for_spending = (after_costs - floor).max(0.);
        let library_need = i.library_need * price_start;
        immediate += i.annual_immediate_donations;
        let immediate_distribution = immediate.min(library_need);
        immediate -= immediate_distribution;
        let endowment_distribution = spending_target
            .min(available_for_spending)
            .min((library_need - immediate_distribution).max(0.));
        balance = (after_costs - endowment_distribution).max(0.);
        let retained_interest = (net_interest - endowment_distribution).max(0.);
        let libraries = immediate_distribution + endowment_distribution;
        cumulative += libraries;
        let real_closing = balance / (price_start * (1. + i.inflation));
        let from_accumulated_assets = (endowment_distribution - net_interest).max(0.);
        let principal_used_for_grants =
            ((basis - balance).max(0.) - (basis - after_costs).max(0.)).max(0.);
        if [
            balance,
            immediate,
            cumulative,
            real_closing,
            basis,
            costs_due,
        ]
        .iter()
        .any(|v| !v.is_finite() || *v > 1e18)
        {
            return Err(
                "The scenario exceeds the supported numeric range. Reduce inputs or horizon."
                    .into(),
            );
        }
        years.push(Year {
            year,
            opening,
            capital_donations: i.annual_capital_donations,
            immediate_donations: i.annual_immediate_donations,
            return_rate: rate,
            interest_income,
            valuation_change,
            net_interest,
            retained_interest,
            investment_return,
            costs_due,
            costs_paid,
            unpaid_costs,
            spending_target,
            reserve_target,
            reserve_shortfall: (reserve_target - balance).max(0.),
            contributed_principal: basis,
            available_for_spending,
            endowment_distribution,
            immediate_distribution,
            immediate_balance: immediate,
            libraries,
            cumulative_support: cumulative,
            closing: balance,
            real_closing,
            real_contributed_principal: real_basis,
            library_need,
            gap: (library_need - libraries).max(0.),
            from_accumulated_assets,
            principal_used_for_grants,
        });
    }
    Ok(Projection { years })
}
pub const SCENARIOS: [&str; 6] = [
    "Full-target hypothetical illustration",
    "No additional donations",
    "Weak interest income",
    "Continuing losses",
    "Early loss, then assumed recovery",
    "Custom",
];
pub fn scenario(index: usize) -> Inputs {
    let mut i = Inputs::default();
    match index {
        1 => {
            i.annual_capital_donations = 0.;
            i.annual_immediate_donations = 0.;
        }
        2 => i.interest_rate = 0.005,
        3 => {
            i.interest_rate = 0.005;
            i.return_rate = -0.08;
        }
        4 => i.first_year_return = Some(-0.20),
        _ => (),
    }
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
