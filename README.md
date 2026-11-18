# Saskatchewan Private Equity Library Funding

An interactive concept demonstration for library leadership, legal advisers,
elected representatives and prospective investment partners. It explains a
proposed private-equity-funded mechanism; it is not an investment offering.

## Architecture

- **Rust:** the complete interface, original illustrations, financial engine,
  allocation engine, content, local state, exports and keyboard interactions.
  `eframe` / `egui` render the interface to a WebAssembly canvas.
- **Unison:** an independent, executable specification of the proposed 50/20/30
  cash policy, with base, lower-income, no-income and unfunded-deficit checks.
  This is verification code, not a browser runtime or a remote financial service.
- **Go:** static build assembly, minimal loader generation, a read-only local
  HTTP server and server tests. The deployed site needs no running Go service.

There is no Java, application JavaScript, CSS file or HTML interface template.
As agreed for browser delivery, Go generates the minimal HTML/WebAssembly loader.
It uses JavaScript only to size the canvas and load the Rust module;
`wasm-bindgen` generates the required browser bindings. Canvas sizing uses the
browser style API within that loader; all application visuals are drawn in Rust.

## Run locally

Prerequisites: Rust with the `wasm32-unknown-unknown` target, `wasm-pack`, Go,
and Unison UCM 1.3.0 for the independent policy checks.

```powershell
rustup target add wasm32-unknown-unknown
go run ./cmd/studio build
go run ./cmd/studio serve
```

Open **http://127.0.0.1:4387/**. For another unused port:

```powershell
go run ./cmd/studio serve -addr 127.0.0.1:4390
```

Optional native preview: `cargo run --bin library-funding`.

## Views

`#/overview` explains the proposition and its boundaries. `#/atlas` connects
private investors, the vehicle, businesses, reserve, allocation committee,
libraries and community outcomes. `#/investment` separates ownership from
funding agreements. `#/money` walks the cash waterfall. `#/lab` exposes every
assumption and stress preset. `#/allocation` shows capped needs-based awards
for six fictional applicants. `#/governance` explains proposed responsibilities.
`#/evidence` is a searchable, sourced readiness register. `#/roadmap` provides
five decision gates. `#/summary` supports director, attorney and MLA review.

The six-stop guided tour links these views. Section links are shareable.
Custom financial inputs stay in memory and are not placed in URLs, sent to a
server, stored in cookies, or persisted. Reloading starts the base illustration.

## Review and print

The summary downloads a readable `.txt` review brief with the current
assumptions, allocations, evidence, responsibilities and readiness gates, plus
optional JSON model data. Open the text brief in any text editor to print it.
The build also includes `review.txt`, a direct, screen-reader-friendly default
brief which can be shared without opening the canvas application.

The Rust canvas supports keyboard controls, visible focus and static,
motion-free rendering. An on-screen text table accompanies allocation charts.
It does **not** provide the same DOM semantics as a conventional HTML table;
the plain-text brief is the accessible reading and print alternative. There is
no CSS print stylesheet, consistent with the requested canvas-only application.

## Model and decisions

Defaults follow the supplied brief: CA$250M total equity includes CA$5M opening
reserve, leaving CA$245M invested in businesses. An 8% business cash yield and
1.5% management rate use that invested amount. Administration is CA$500k; the
vehicle tax placeholder is 20%; target reserve is CA$10M. Positive residual cash
is split 50% investors, 20% libraries and 30% retained reinvestment.

Base annual result: CA$1,468,000 for libraries, 29.36% of the illustrative CA$5M
need; CA$3,532,000 remains unfunded. Investor cash is CA$3,670,000 (1.468% of total
subscribed equity), retained reinvestment CA$2,202,000 and closing reserve CA$10M.
Cash yield is not IRR or total return. The model does not establish sustainability.

Capital, acquisitions, participation and endorsement are unsecured or unagreed.
Legal structure, tax treatment, investor rights, redemption terms, liquidity,
preferred returns, carried interest, reserve policy, eligibility, allocation
weights, appeals and legally effective protection of distribution terms require
decisions and professional assessment. No public library asset is investor
collateral. No new government funding, donations, sponsorships, crowdfunding,
library charges or borrowing are included.

## Verification

```powershell
cargo test --tests
go test ./...
go run ./cmd/studio policy
cargo run --example render_review
```

On Windows the Go policy command sets UTF-8 console input/output encoding for
UCM. If running UCM directly, run `chcp 65001` first. The transcript uses only
built-in Unison functions, with no remote library download.

`examples/render_review.rs` rasterizes the actual egui meshes to PNGs in
`target/review/`, for deterministic desktop and mobile visual inspection.
This is offscreen Rust UI QA; it is not a claim of browser automation coverage.
See [VERIFICATION.md](VERIFICATION.md) for the final verification record.

## Sources and publication

The reference at [Saber ecosystem atlas](https://wolfbreed1829.github.io/saber-ecosystem-demo/#/atlas)
informed the dark presentation, selectable network, relationship overlays and
inspector approach. The implementation, content and artwork here are original.
No assets or application code were copied from that project.

The [Government of Saskatchewan library system overview](https://www.saskatchewan.ca/residents/education-and-learning/library-system-in-saskatchewan)
was checked on 18 November 2026 for provincial context only. It does not verify
any invented funding gap, participation or endorsement. All business and
applicant figures are explicitly synthetic.

Static output in `dist/` is published through GitHub Pages. Application code
has no analytics, forms, payment processing or input collection. GitHub Pages
records visitor IP addresses for security, as described in its
[hosting documentation](https://docs.github.com/en/pages/getting-started-with-github-pages/what-is-github-pages).
