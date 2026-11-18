# Saskatchewan Library Sustainability Trust

A locally runnable concept demonstration for library directors, corporate donors,
community representatives and professional advisers.

“Building a lasting fund for Saskatchewan libraries through donations, responsible
investment, and transparent community support.”

The proposed fundraising target is **CA$250 million of donated principal from
corporations**. This is a target, not secured capital. The working investment
approach is money-market-style investing, subject to professional assessment of
specific instruments, custody, liquidity, costs, risks and legal suitability.
The proposed policy allocates **50% of positive net cash interest to libraries and
reinvests 50%**, after investment and administration costs. Actual grants also
respect eligible need, reserves and donor restrictions. Unused grant capacity
stays in the fund.

Donors receive no shares, equity, repayment rights or investment returns.
Libraries retain their existing ownership, governance and service authority.
Trustees oversee the donated assets; they do not personally own them.

“Trust” is a working description. The Canadian legal and charitable structure
remains unresolved. No charitable status, tax receipts, secured donations,
participation, endorsements or guaranteed funding are claimed. This demo does
not collect donations and has no payment processing.

## Shareable site

The public proposal is hosted on GitHub Pages:
https://sabertech-development.github.io/saskatchewan-library-trust/

The source is on `main`; the verified static build is on `gh-pages`. The site
contains only the concept presentation and calculations performed in the visitor's
browser. Publishing the proposal does not establish the trust, charitable status,
library participation or a fundraising service. No donation collection is enabled.

## Run locally

Prerequisites: Rust with `wasm32-unknown-unknown`, `wasm-pack`, Go, and Node.js.

```powershell
rustup target add wasm32-unknown-unknown
go run ./cmd/studio build
go run ./cmd/studio serve
```

Open **http://127.0.0.1:4387/#/overview**. The server serves files only and rejects
POST, PUT and DELETE. Use `serve -addr 127.0.0.1:4390` for an alternate local port.
No model inputs are transmitted, stored in URLs or persisted between visits.

- Visual presentation: `index.html#/overview`
- Semantic HTML reading and calculator: `reader.html`
- Designed default review PDF: `library-trust-review.pdf`
- Plain-text default review and complete ledger: `review.txt`
- Optional native visual preview: `cargo run --bin library-funding`

## Experience

Both the visual demo and the semantic HTML page now use a bounded desktop
workspace with a compact heading, aligned panels and tabs that replace content.
The funding view has How it works, Scenario calculator and Funding & accountability
views. Its chart describes the 50/50 split of **net interest**, not a division of
principal. Calculator inputs and outputs are grouped into separate tabs; financial
ledgers have their own views. At small widths or high zoom the layout reflows so
text and controls remain usable. The no-script narrative is retained as a fallback.

The five main questions cover why a lasting source matters, how corporate gifts
build the fund, how interest supports libraries and reinvestment, how communities
receive fair support, and who oversees and reports the money. The library atlas
connects donors, communities, the fund, reserves, oversight, libraries and service
outcomes. Opening hours, staffing, collections, digital access, accessibility and
programming are illustrative uses, not verified local needs or promised results.

Supporting views explain scenarios, allocation criteria, evidence, readiness and
review. Existing links `#/investment`, `#/money` and `#/lab` resolve to the new
donations, spending and scenarios views. The five-stop tour, keyboard navigation,
responsive menu, focus outlines and Escape-to-close behavior are retained.

## Financial model

The default scenario assumes the proposed target has already been raised. All
amounts and rates are editable. Interest and cost assumptions are hypothetical,
not quoted yields, approved budgets or forecasts. Additional annual donations
may be zero. Restricted long-term gifts and immediate-use gifts have independent
ledgers, and unused immediate-use gifts never become endowment capital.

The shared Rust engine models 1–50 years. It separates cash interest from capital
value changes, including weak-income and loss paths. Net interest is cash interest
less all investment and administration costs due, floored at zero. The proposed
library budget is 50% of net interest; the other 50% is reinvested. The share is
editable for comparisons. Grant caps account for reserve and principal floors,
need and separately funded immediate-use grants. Retained interest is already
included in closing assets and is not counted twice.

The reserve is an accounting retention floor inside the fund, expressed as a
percentage of opening balance, not a separate cash account. With principal
protection, the retained floor is the greater of cumulative contributions and
the reserve target. Losses and permitted costs can still erode principal.

A separately labelled **comparison** can use a spending rate times the opening
annual balance. This is not the default interest-sharing proposal. It illustrates
how grants may use past accumulated earnings or, if hypothetical gift terms allow,
contributed capital. Changing the illustration never releases an actual restriction.

Outputs include annual interest, costs, library grants by source, reinvestment,
nominal and inflation-adjusted fund balances, cumulative support, reserve and
funding gaps, unpaid costs and capital used for grants. The full timing and
calculation rules appear beside the tool and in every review export. No tax
treatment or investment product is assumed. Results do not guarantee permanent
funding, capital preservation or preservation of purchasing power.

## Architecture and accessibility

- Rust/egui renders the visual presentation and original architectural artwork.
- `src/finance.rs` is the single calculation engine for both interfaces.
- `src/reader.rs` generates semantic HTML with headings, native labelled controls,
  fieldsets, status/error announcements and captioned tables. A small JavaScript
  adapter calls the compiled Rust engine; it contains no duplicate financial math.
- Go builds and serves static output in `dist/`; no production service is required.
- Unison independently checks selected one-period grant and interest policies.
- Space Grotesk is bundled under the SIL Open Font License in `assets/fonts/`.

The canvas has keyboard support and a first-focus link to the semantic companion.
The companion supplies the core proposal, text atlas, scenario tool and exact
data tables without requiring interpretation of a chart. The narrative also works
without JavaScript. These improvements are not a claim of WCAG conformance or a
substitute for assistive-technology user testing.

## Verification

```powershell
cargo test --tests
go test ./...
go run ./cmd/studio policy
cargo run --example render_review
go run ./cmd/studio build
npm install --prefix target/semantic-tests --no-audit --no-fund jsdom
node tests/semantic.mjs
```

Unison UCM 1.3.0 is needed for the optional independent policy checks. The DOM
integration test runs the actual WASM engine in a DOM simulator; it does not
automate a browser. Rendered desktop/mobile QA artifacts live in `target/review/`.
See `VERIFICATION.md` for coverage and limitations.

## References

Visual direction follows the requested [Saber atlas reference](https://wolfbreed1829.github.io/saber-ecosystem-demo/#/atlas).
Library-specific content and Rust artwork are original.

The evidence view links the [Saskatchewan library system overview](https://www.saskatchewan.ca/residents/education-and-learning/library-system-in-saskatchewan),
[CRA guidance before giving](https://www.canada.ca/en/revenue-agency/services/charities-giving/giving-charity-information-donors/before-you-give.html),
and [BC Securities Commission information on mutual funds](https://www.investright.org/understand-investing/types-of-investments/common-investments/mutual-funds/).
These sources establish context only; they do not endorse or verify the proposed fund.

## Designed review PDF

The review download renders the selected, validated scenario locally. Black pages,
Space Grotesk type, cyan charts and the Saber Tech Inc. header are shared by the
canvas and semantic HTML exports. The default 20-year report is nine pages; the
annual ledger paginates for longer horizons. Static default PDF and plain-text
review downloads remain available. No scenario values leave the device.

`src/report.rs` supplies inputs, projections and stress comparisons from the shared
Rust engine. The PDF layer formats that data without recalculating finances.
`assets/report-template.pdf` is the ReportLab vector master; the checked-in local
runtime bundles pdf-lib and fontkit, with licenses alongside it. Ordinary builds
need Node.js, but no npm download or Python installation.

To edit the PDF design and regenerate its assets:

```powershell
python scripts/create_report_template.py  # Requires reportlab
npm ci
npm run build:report
go run ./cmd/studio build
node tests/report.mjs
node tests/semantic.mjs
```

Render the resulting PDF with Poppler and inspect every page after layout changes.
The report is selectable text and vector graphics; it is not a tagged PDF/UA
document. The semantic HTML and plain-text review provide accessible alternatives.

The visual loader presents immediate HTML content while WebAssembly starts, awaits
renderer startup, and routes failures or a 25-second timeout to the semantic
proposal. The accessible view keeps its narrative available if graphics or the
calculation engine cannot start. Verify this path with `node tests/loader.mjs`.
