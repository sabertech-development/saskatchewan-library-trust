//! One validated data contract for every PDF chart and comparison.
use crate::{
    content::TITLE,
    finance::{self, Inputs},
};
pub fn data(i: &Inputs) -> Result<String, String> {
    let selected = finance::calculate(i)?;
    let mut comparisons = Vec::new();
    for index in 0..5 {
        let mut c = i.clone();
        let (name, change) = match index {
            0 => ("Selected scenario", "All selected inputs"),
            1 => {
                c.annual_capital_donations = 0.;
                c.annual_immediate_donations = 0.;
                (
                    "No additional gifts",
                    "Both annual donation sources set to zero",
                )
            }
            2 => {
                c.interest_rate = 0.005;
                c.return_rate = 0.;
                c.first_year_return = None;
                (
                    "Weak interest",
                    "0.5% cash interest; no capital value change",
                )
            }
            3 => {
                c.interest_rate = 0.005;
                c.return_rate = -0.08;
                c.first_year_return = None;
                (
                    "Continuing losses",
                    "0.5% cash interest; -8% annual capital value change",
                )
            }
            _ => {
                c.first_year_return = Some(-0.20);
                (
                    "Early capital loss",
                    "-20% first-year value change; selected assumptions thereafter",
                )
            }
        };
        comparisons.push(match finance::calculate(&c) {
            Ok(projection)=>serde_json::json!({"name":name,"change":change,"inputs":c,"projection":projection}),
            Err(error)=>serde_json::json!({"name":name,"change":change,"inputs":c,"projection":null,"error":error}),
        });
    }
    serde_json::to_string(&serde_json::json!({"version":1,"title":TITLE,"hypothetical":true,"inputs":i,"projection":selected,"comparisons":comparisons})).map_err(|e|e.to_string())
}
pub fn from_json(input: &str) -> Result<String, String> {
    let i: Inputs = serde_json::from_str(input).map_err(|e| format!("Invalid scenario: {e}"))?;
    data(&i)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn report_preserves_selection_and_isolates_comparison_changes() {
        let inputs = Inputs {
            capital: 12_000_000.,
            annual_capital_donations: 20_000.,
            annual_immediate_donations: 30_000.,
            years: 50,
            library_interest_share: 0.4,
            ..Inputs::default()
        };
        let value: serde_json::Value = serde_json::from_str(&data(&inputs).unwrap()).unwrap();
        assert_eq!(value["inputs"], serde_json::to_value(&inputs).unwrap());
        assert_eq!(
            value["projection"],
            serde_json::from_str::<serde_json::Value>(
                &serde_json::to_string(&finance::calculate(&inputs).unwrap()).unwrap()
            )
            .unwrap()
        );
        let cs = value["comparisons"].as_array().unwrap();
        assert_eq!(cs.len(), 5);
        for comparison in cs {
            let i: Inputs = serde_json::from_value(comparison["inputs"].clone()).unwrap();
            assert_eq!(i.years, 50);
            assert_eq!(i.capital, inputs.capital);
            assert_eq!(i.library_interest_share, 0.4);
            assert_eq!(
                comparison["projection"],
                serde_json::from_str::<serde_json::Value>(
                    &serde_json::to_string(&finance::calculate(&i).unwrap()).unwrap()
                )
                .unwrap()
            );
        }
        assert_eq!(cs[1]["inputs"]["annual_capital_donations"], 0.);
        assert_eq!(cs[1]["inputs"]["annual_immediate_donations"], 0.);
        assert_eq!(cs[2]["inputs"]["interest_rate"], 0.005);
        assert_eq!(cs[3]["inputs"]["return_rate"], -0.08);
        assert_eq!(cs[4]["inputs"]["first_year_return"], -0.20);
    }
    #[test]
    fn invalid_inputs_cannot_be_exported() {
        assert!(
            data(&Inputs {
                years: 0,
                ..Inputs::default()
            })
            .is_err()
        );
        assert!(from_json("{\"years\":0}").is_err());
        assert!(from_json("{\"capital\":\"invalid\"}").is_err());
    }
    #[test]
    fn out_of_range_comparison_does_not_block_a_valid_selection() {
        let i = Inputs {
            capital: 1e12,
            interest_rate: 0.,
            return_rate: 1.,
            first_year_return: Some(-1.),
            administration: 0.,
            years: 50,
            ..Inputs::default()
        };
        let report: serde_json::Value = serde_json::from_str(&data(&i).unwrap()).unwrap();
        assert_eq!(report["projection"]["years"].as_array().unwrap().len(), 50);
        assert!(report["comparisons"][4]["projection"].is_null());
        assert!(report["comparisons"][4]["error"].is_string());
    }
}
