//! Semantic HTML companion. All financial calculations use the shared Rust/WASM engine.
use crate::{
    content::*,
    finance::{self, Inputs},
};
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
pub fn html() -> String {
    let mut page = String::from(
        r##"<!doctype html><html lang="en-CA"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="color-scheme" content="dark"><title>Saskatchewan Library Sustainability Trust — accessible reading and scenarios</title>
<style>
@font-face{font-family:Space;src:url('fonts/SpaceGrotesk.ttf')}*{box-sizing:border-box}body{margin:0;background:#030509;color:#e8f2fa;font:17px/1.65 Space,system-ui,sans-serif}a{color:#b4edff}a:hover{color:#3dcaff}header,main,footer{max-width:1200px;margin:auto;padding:24px}header{border-bottom:1px solid #1c3447}nav{display:flex;gap:18px;flex-wrap:wrap}h1{font-size:clamp(30px,5vw,52px);line-height:1.15;max-width:950px}h2{font-size:28px;line-height:1.3}h3{font-size:21px}p{max-width:85ch}section{margin:42px 0;scroll-margin-top:20px}.lead{font-size:23px;color:#b4edff}.note{color:#a8becf}.panel,fieldset,details{background:#08111d;border:1px solid #1c3447;padding:22px;margin:18px 0;min-width:0}.grid{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:18px}legend{color:#3dcaff;padding:0 8px;font-weight:bold}label{display:block;margin-top:14px}input,select,button{font:inherit;padding:10px 12px;border:1px solid #507d9d;background:#071523;color:#e8f2fa;max-width:100%;min-height:44px}input,select{width:100%}button{cursor:pointer}button[type=submit]{background:#3dcaff;color:#030509;font-weight:bold}.actions{display:flex;gap:12px;flex-wrap:wrap;margin:20px 0}:focus-visible{outline:3px solid #3dcaff;outline-offset:4px}.skip{position:absolute;left:16px;top:-100px;padding:10px;background:#08111d;z-index:10}.skip:focus{top:10px}.table-wrap{overflow:auto;border:1px solid #1c3447}table{border-collapse:collapse;width:100%;font-variant-numeric:tabular-nums}caption{text-align:left;padding:14px;color:#b4edff}th,td{padding:12px;text-align:right;border-bottom:1px solid #1c3447;white-space:nowrap}th:first-child,td:first-child{text-align:left}thead th{background:#102438}tbody th{font-weight:normal}tr:nth-child(even){background:#08111d}summary{cursor:pointer;font-weight:bold}#error{color:#ffb8ac}.metrics{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:14px}.metrics div{background:#0b1d2d;padding:18px}.metrics dt{font-size:15px}.metrics dd{font-size:26px;color:#3dcaff;margin:6px 0}footer{border-top:1px solid #1c3447;font-size:15px}@media(max-width:720px){.grid,.metrics{grid-template-columns:1fr}header,main,footer{padding:18px}}@media print{body{background:white;color:black}a{color:black}nav,form,.actions,.skip{display:none}.panel,details,fieldset{background:white;color:black;border-color:#aaa;break-inside:avoid}.table-wrap{overflow:visible}table{font-size:9px}th,td{padding:4px;white-space:normal}thead th,tr:nth-child(even){background:#eee}h1{font-size:30px}.metrics dd{color:black}}
</style><link rel="stylesheet" href="reader-layout.css"></head><body><a class="skip" href="#main">Skip to main content</a><header><p>PROPOSED / SASKATCHEWAN</p><nav aria-label="Page navigation"><a href="#purpose">Purpose</a><a href="#donations">Donations</a><a href="#spending">Spending</a><a href="#fairness">Fair funding</a><a href="#oversight">Oversight</a><a href="#scenario">Scenario tool</a><a href="#evidence">Evidence</a><a href="index.html#/overview">Visual demo</a></nav></header><main id="main"><h1>Saskatchewan Library Sustainability Trust</h1>"##,
    );
    page.push_str(&format!("<p class=\"lead\">{}</p><p class=\"note\">{}</p><p><strong>Corporate donations → Principal fund → Interest income → Library support + reinvestment</strong></p><p>The proposed principal fundraising target is CA$250 million from corporations. It is not secured capital. The working approach is money-market-style investment: 50% of net interest supports libraries and 50% is reinvested. Rates and funding vary. This local demo collects no donations.</p>",escape(MESSAGE),escape(QUALIFICATION)));
    for ((q, a, _), id) in
        QUESTIONS
            .iter()
            .zip(["purpose", "donations", "spending", "fairness", "oversight"])
    {
        page.push_str(&format!(
            "<section id=\"{id}\"><h2>{}</h2><p>{}</p>",
            escape(q),
            escape(a)
        ));
        match id{
            "purpose"=>page.push_str("<p>Reliable services need staff, collections, technology and accessible spaces over many years. A supplementary fund could help libraries plan for changing costs. Actual service gaps and participation must be verified locally; no province-wide shortfall is asserted.</p>"),
            "donations"=>page.push_str("<div class=\"grid\"><article class=\"panel\"><h3>Long-term capital gifts</h3><p>Hold and invest donated assets for the stated purpose under documented gift terms. Track any restrictions on contributed principal.</p></article><article class=\"panel\"><h3>Immediate-use gifts</h3><p>Keep these funds separate for eligible immediate library needs. Unused amounts retain their restrictions and are not moved into the endowment.</p></article></div><p>The proposed fund holds and manages assets. Trustees oversee them; they do not personally own them. Donors receive no shares, equity, repayment rights or investment returns. No tax receipts are promised before relevant status and arrangements are established.</p>"),
            "spending"=>page.push_str("<p>The working approach is to invest donated principal in money-market-style instruments. Professional advice must establish specific instruments, costs, liquidity and reserve arrangements. The proposed split sends 50% of positive net cash interest to libraries and reinvests 50%, after investment and administration costs. Capital-value gains are not treated as interest. Principal and interest rates are not guaranteed. Formal governing arrangements still need to be established.</p>"),
            "fairness"=>{page.push_str("<p>Publish eligibility, assess verified needs, disclose conflicts, explain awards and provide a fair appeal or review route. Respect donor restrictions and keep service decisions with the appropriate library authorities. The following uses are illustrative, not promised outcomes.</p><div class=\"grid\">");for(t,b)in OUTCOMES{page.push_str(&format!("<article class=\"panel\"><h3>{}</h3><p>{}</p></article>",escape(t),escape(b)));}page.push_str("</div>");},
            "oversight"=>for r in ROLES{page.push_str(&format!("<details><summary>{}</summary><p>{}</p><p>{}</p><p>{}</p></details>",escape(r.title),escape(r.mandate),escape(r.decisions),escape(r.controls)));},_=>()
        }
        page.push_str("</section>");
    }
    page.push_str("<section id=\"atlas\"><h2>Library atlas: a text equivalent</h2><p>Illustrative relationships link donors to the long-term fund and, separately, immediate library needs. The fund supports eligible libraries under spending and reserve policies. Oversight checks the fund and allocations. Community priorities inform libraries; library services contribute to practical outcomes. No participation is claimed.</p>");
    for n in NODES {
        page.push_str(&format!("<details><summary>{}</summary><p>{}</p><dl><dt>Inputs</dt><dd>{}</dd><dt>Outputs</dt><dd>{}</dd><dt>Responsible role</dt><dd>{}</dd><dt>Dependencies</dt><dd>{}</dd><dt>Status</dt><dd>{}</dd><dt>Open question</dt><dd>{}</dd></dl></details>",escape(n.name),escape(n.purpose),escape(n.inputs),escape(n.outputs),escape(n.owner),escape(n.dependencies),escape(n.readiness),escape(n.question)));
    }
    page.push_str("</section><section id=\"scenario\"><h2>Illustrative endowment scenario tool</h2><p id=\"scenario-help\">The proposed fundraising target is CA$250 million. The starting illustration assumes that principal has been raised, without claiming secured funds. All figures are editable; interest rates and costs are hypothetical, not quoted yields. The proposed net-interest split is 50% for libraries and 50% reinvested; changes are scenario comparisons. Calculations run locally using the same Rust engine as the visual demo. No values are sent to a server or saved between visits.</p><noscript><p>The narrative is available without scripts. Interactive calculation requires JavaScript and WebAssembly; <a href=\"review.txt\">read the default scenario ledger as plain text</a>.</p></noscript><label for=\"preset\">Load a hypothetical scenario</label><select id=\"preset\" disabled>");
    for (i, label) in finance::SCENARIOS[..5].iter().enumerate() {
        page.push_str(&format!("<option value=\"{i}\">{}</option>", escape(label)));
    }
    page.push_str("</select><form id=\"scenario-form\" aria-describedby=\"scenario-help\"><div class=\"grid\">");
    let i = Inputs::default();
    for (legend, fields) in [
        (
            "Separate donation purposes",
            vec![
                (
                    "capital",
                    "Opening donated long-term capital / CAD",
                    i.capital,
                    0.,
                    1e12,
                ),
                (
                    "annual_capital_donations",
                    "Additional annual long-term gifts / CAD",
                    i.annual_capital_donations,
                    0.,
                    1e12,
                ),
                (
                    "annual_immediate_donations",
                    "Annual immediate-use gifts / CAD",
                    i.annual_immediate_donations,
                    0.,
                    1e12,
                ),
            ],
        ),
        (
            "Performance, costs and inflation",
            vec![
                (
                    "interest_rate",
                    "Assumed annual cash interest yield / %",
                    i.interest_rate * 100.,
                    0.,
                    100.,
                ),
                (
                    "library_interest_share",
                    "Library share of positive net interest / % (hypothetical)",
                    i.library_interest_share * 100.,
                    0.,
                    100.,
                ),
                (
                    "return_rate",
                    "Annual capital value change / loss / %",
                    i.return_rate * 100.,
                    -100.,
                    100.,
                ),
                (
                    "investment_cost_rate",
                    "Annual investment costs / %",
                    i.investment_cost_rate * 100.,
                    0.,
                    100.,
                ),
                (
                    "administration",
                    "First-year administration / CAD",
                    i.administration,
                    0.,
                    1e12,
                ),
                (
                    "inflation",
                    "Annual inflation / %",
                    i.inflation * 100.,
                    0.,
                    25.,
                ),
            ],
        ),
        (
            "Spending and reserve policies",
            vec![
                (
                    "spending_rate",
                    "Comparison-only annual spending rate / %",
                    i.spending_rate * 100.,
                    0.,
                    100.,
                ),
                (
                    "reserve_rate",
                    "Retained reserve floor / % of opening balance",
                    i.reserve_rate * 100.,
                    0.,
                    100.,
                ),
            ],
        ),
        (
            "Need and horizon",
            vec![
                (
                    "library_need",
                    "First-year library need / CAD",
                    i.library_need,
                    0.,
                    1e12,
                ),
                ("years", "Projection years", i.years as f64, 1., 50.),
            ],
        ),
    ] {
        page.push_str(&format!("<fieldset><legend>{legend}</legend>"));
        for (name, label, value, min, max) in fields {
            page.push_str(&format!("<label for=\"{name}\">{label}</label><input id=\"{name}\" name=\"{name}\" type=\"number\" min=\"{min}\" max=\"{max}\" step=\"{}\" value=\"{value}\" required>",if name=="years"{"1"}else{"any"}));
        }
        if legend == "Performance, costs and inflation" {
            page.push_str("<label for=\"first_year_return\">Optional first-year capital value change / % (leave blank to use the annual value-change assumption)</label><input id=\"first_year_return\" name=\"first_year_return\" type=\"number\" min=\"-100\" max=\"100\" step=\"any\">");
        }
        if legend == "Spending and reserve policies" {
            page.push_str("<label for=\"interest_only\">Policy to illustrate</label><select id=\"interest_only\" name=\"interest_only\"><option value=\"true\">Proposed model: share net interest and reinvest the rest</option><option value=\"false\">Comparison only: spend a percentage of opening balance</option></select><p>The unused share of net interest is reinvested. The comparison spending rate applies only to the second policy.</p>");
            page.push_str("<label for=\"protect_principal\">Hypothetical gift terms for long-term capital</label><select id=\"protect_principal\" name=\"protect_principal\"><option value=\"true\">Contributed principal restricted from grants</option><option value=\"false\">Gift terms permit grants from capital</option></select><p class=\"note\">This models different gift terms; it cannot release a real restriction. Losses and permitted costs can still erode principal. The proposed library budget is the library share of positive net interest after costs. The comparison uses spending rate × opening balance. Actual grants also respect reserves, restrictions and need.</p>");
        }
        page.push_str("</fieldset>");
    }
    page.push_str(r##"</div><div class="actions"><button type="submit" id="calculate" disabled>Calculate hypothetical scenario</button><button type="button" id="reset" disabled>Reset inputs</button></div></form><p id="engine-status" role="status">Loading the local calculation engine…</p><p id="error" role="alert"></p><section id="results" aria-labelledby="results-title" hidden><h3 id="results-title">Hypothetical results</h3><p id="result-status" role="status" aria-live="polite" aria-atomic="true"></p><dl class="metrics" id="metrics"></dl><p>The proposed policy uses positive net cash interest after costs; unrealized gains are not interest. The separately labelled balance-spending comparison may draw on capital if gift terms permit. Cumulative support is nominal. Immediate-use gifts and carryover remain separate from long-term assets.</p><div class="actions"><button type="button" id="download">Download complete scenario data</button></div><div class="table-wrap" tabindex="0" role="region" aria-label="Yearly scenario ledger; scroll horizontally for all columns"><table><caption>Annual ledger: nominal CAD except purchasing power, shown in starting-year dollars</caption><thead id="table-head"></thead><tbody id="table-body"></tbody></table></div><details><summary>Detailed policy, cost and capital-use ledger</summary><div class="table-wrap" tabindex="0" role="region" aria-label="Detailed annual ledger"><table><caption>Annual policy ceilings and restrictions; CAD</caption><thead id="detail-head"></thead><tbody id="detail-body"></tbody></table></div></details></section><h3>Calculation rules and limits</h3>"##);
    for (title, body) in finance::RULES {
        page.push_str(&format!(
            "<details><summary>{}</summary><p>{}</p></details>",
            escape(title),
            escape(body)
        ));
    }
    page.push_str("</section><section id=\"evidence\"><h2>Evidence and open decisions</h2>");
    for e in EVIDENCE {
        page.push_str(&format!("<article class=\"panel\"><h3>{}</h3><p><strong>{}</strong></p><p>{}</p><p>Next: {}</p>",escape(e.claim),escape(e.status),escape(e.detail),escape(e.next)));
        if !e.source.is_empty() {
            page.push_str(&format!("<a href=\"{}\">{}</a><p class=\"note\">Checked 18 November 2026; external context does not verify this proposed fund.</p>",escape(e.source),if e.source.contains("saskatchewan.ca"){"Government of Saskatchewan: library system"}else if e.source.contains("investright.org"){"BC Securities Commission: mutual funds"}else{"Canada Revenue Agency: before you give"}));
        }
        page.push_str("</article>");
    }
    page.push_str("</section><section><h2>Proposed readiness steps</h2>");
    for (t, b) in STAGES {
        page.push_str(&format!("<h3>{}</h3><p>{}</p>", escape(t), escape(b)));
    }
    page.push_str("</section><p><a href=\"review.txt\">Download the default plain-text review brief</a> · <a href=\"#main\">Back to top</a></p></main><footer>Local concept demonstration. No donations collected. No charitable status, secured funds, participation or endorsement claimed.</footer><script src=\"reader-layout.js\"></script><script type=\"module\" src=\"reader.js\"></script></body></html>");
    page
}
pub const SCRIPT: &str = r##"
import init, {project_scenario, scenario_inputs} from './pkg/saskatchewan_library_funding.js';
const form=document.getElementById('scenario-form');
const percent=new Set(['interest_rate','library_interest_share','return_rate','investment_cost_rate','inflation','spending_rate','reserve_rate','first_year_return']);
const money=new Intl.NumberFormat('en-CA',{style:'currency',currency:'CAD',maximumFractionDigits:0});
let latest=null;
function inputs(){const data={};for(const [key,value] of new FormData(form)){if(key==='protect_principal'||key==='interest_only')data[key]=value==='true';else if(key==='first_year_return'&&value==='')data[key]=null;else data[key]=Number(value)/(percent.has(key)?100:1);}return data;}
function putInputs(data){for(const [key,value] of Object.entries(data)){const el=form.elements.namedItem(key);if(el)el.value=value===null?'':typeof value==='boolean'?String(value):value*(percent.has(key)?100:1);}}
function ledger(head,body,fields,years){const tr=document.createElement('tr');for(const [,label]of fields){const th=document.createElement('th');th.scope='col';th.textContent=label;tr.append(th);}document.getElementById(head).replaceChildren(tr);const rows=[];for(const y of years){const row=document.createElement('tr');for(const [key]of fields){const cell=document.createElement(key==='year'?'th':'td');if(key==='year')cell.scope='row';cell.textContent=key==='year'?String(y[key]):money.format(y[key]);row.append(cell);}rows.push(row);}document.getElementById(body).replaceChildren(...rows);}
function calculate(){if(!form.reportValidity())return;document.getElementById('error').textContent='';try{const i=inputs();const p=JSON.parse(project_scenario(JSON.stringify(i)));latest={hypothetical:true,inputs:i,projection:p};const y=p.years.at(-1);const metrics=document.getElementById('metrics');metrics.replaceChildren();for(const [label,value]of [['Closing long-term fund',y.closing],['Purchasing power in starting-year dollars',y.real_closing],['Cumulative library support (nominal)',y.cumulative_support],['Final-year funding gap',y.gap],['Immediate-use carryover (separate)',y.immediate_balance],['Real value of contributed capital at gift dates',y.real_contributed_principal]]){const block=document.createElement('div'),dt=document.createElement('dt'),dd=document.createElement('dd');dt.textContent=label;dd.textContent=money.format(value);block.append(dt,dd);metrics.append(block);}
const warnings=[];if(y.real_closing<y.real_contributed_principal)warnings.push('Purchasing power is below the real value of contributions.');if(p.years.some(v=>v.unpaid_costs>0))warnings.push('Some costs cannot be paid from the long-term fund.');if(p.years.some(v=>v.principal_used_for_grants>0))warnings.push('Grants draw on contributed capital under these hypothetical gift terms.');document.getElementById('result-status').textContent=(i.interest_only?'Proposed interest-sharing illustration. ':'Capital-spending comparison only. ')+i.years+' hypothetical years. '+warnings.join(' ');
ledger('table-head','table-body',[['year','Year'],['opening','Opening fund'],['capital_donations','Capital gifts'],['interest_income','Gross interest'],['valuation_change','Capital value change'],['retained_interest','Net interest reinvested'],['costs_paid','Costs paid'],['endowment_distribution','Endowment grant'],['immediate_distribution','Immediate grant'],['closing','Closing fund'],['real_closing','Purchasing power'],['cumulative_support','Cumulative support'],['library_need','Library need'],['gap','Funding gap'],['immediate_balance','Immediate carryover']],p.years);
ledger('detail-head','detail-body',[['year','Year'],['spending_target','Target grant'],['available_for_spending','Policy ceiling'],['reserve_target','Reserve target'],['reserve_shortfall','Reserve shortfall'],['contributed_principal','Contribution basis'],['costs_due','Costs due'],['unpaid_costs','Unpaid costs'],['from_accumulated_assets','Grant from accumulated assets'],['principal_used_for_grants','Principal used for grants']],p.years);
document.getElementById('results').hidden=false;
}catch(error){latest=null;document.getElementById('results').hidden=true;document.getElementById('error').textContent=String(error);}}
form.addEventListener('submit',event=>{event.preventDefault();calculate();});
form.addEventListener('input',()=>{document.getElementById('result-status').textContent='Inputs changed. Calculate again to update results.';latest=null;});
document.getElementById('reset').addEventListener('click',()=>{putInputs(JSON.parse(scenario_inputs(0)));document.getElementById('preset').value='0';calculate();});
document.getElementById('preset').addEventListener('change',event=>{putInputs(JSON.parse(scenario_inputs(Number(event.target.value))));calculate();});
document.getElementById('download').addEventListener('click',()=>{if(!latest){document.getElementById('error').textContent='Calculate the changed inputs before downloading.';return;}const blob=new Blob([JSON.stringify(latest,null,2)],{type:'application/json'});const url=URL.createObjectURL(blob);const a=document.createElement('a');a.href=url;a.download='library-trust-scenario.json';a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);});
try{await init();document.getElementById('engine-status').textContent='Local calculation engine ready. All inputs are hypothetical.';document.getElementById('preset').disabled=false;document.getElementById('calculate').disabled=false;document.getElementById('reset').disabled=false;calculate();}catch(error){document.getElementById('engine-status').textContent='The calculation engine could not load. The narrative and plain-text review remain available.';document.getElementById('error').textContent=String(error);}
"##;
