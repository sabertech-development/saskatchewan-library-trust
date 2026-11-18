# Verification record

Revision date: 18 November 2026.

The revised proposal follows the user's latest direction: corporate donations
toward CA$250 million of principal, money-market-style investing as a working
approach, and a proposed 50% library / 50% reinvestment split of positive net
interest after investment and administration costs. The target is not secured
capital. All scenario figures are editable; rates and costs are hypothetical.

## Model and controls

34 Rust tests cover the shared engine, UI, allocation and delivery contracts.
The financial suite checks known annual ledgers; net-interest sharing;
reinvestment without double counting; cash interest versus valuation gains;
capital restrictions and explicitly permitted comparison spending; reserves;
timing of donations; zero additional donations; weak income and losses;
inflation; first-year shocks; unpaid costs; zero capital/need; invalid inputs;
and multi-year cash and asset reconciliation across stress combinations.

Immediate-use donations cannot pay investment costs or be silently transferred
to long-term capital. Unused immediate gifts remain in their own ledger. Capped
allocation tests check pools, gaps, baseline scaling and redistribution.

UI checks exercise all ten routes at mobile, tablet and desktop widths, the
five-question tour, route buttons, responsive menu, Escape, keyboard focus,
scenario selection/reset, atlas districts, inspector tabs, zoom and theme locking.

Go checks cover the generated loader, semantic-companion access and the
read-only local server. Five independent Unison checks verify grant caps,
restriction behavior under loss, explicitly permitted comparison capital use,
separate immediate-use carryover and the proposed net-interest sharing formula.

## Presentation and accessible companion

The desktop presentation has been rebuilt around fixed-height review workspaces.
A layout regression check verifies that the nine default narrative/calculator
views and the calculator embedded in Funding fit at 1366 ? 768 without content
scrolling. Tabs expose supporting material separately; detailed ledgers and small
screens may scroll within a view. Funding uses the requested proposal banner,
three views, net-interest donut and service-outcome panel.

The semantic page uses the same screen and tab structure through progressive
enhancement. DOM checks exercise navigation, tab selection, arrow-key behavior,
input groups and the actual calculation engine. Browser geometry of the HTML
companion remains unverified because browser capture is unavailable.

The actual egui meshes are rendered at desktop, tablet and mobile sizes under
`target/review/`. Content and navigation use the donation-funded trust model.
The intro exposes the proposed target and corporate focus without asserting
secured donations or charitable status. The atlas and service profiles are
clearly illustrative. The corrected font, dark/cyan palette and architectural
visual work are retained.

The semantic companion has native labelled inputs, fieldsets, headings, skip
navigation, live result/error announcements and captioned tables. It calls the
same compiled Rust calculation engine. `tests/semantic.mjs` checks its initial
results, custom inputs, separate pools, stress presets, reset and comparison
policy using a DOM simulator and the actual WebAssembly module.

Browser automation is unavailable: the browser host is absent and the available
Edge window is minimized, preventing native state capture. Mesh renders and DOM
integration checks are not full browser or screen-reader testing. No claim of
WCAG conformance is made. The readable narrative and default text ledger remain
available if the interactive engine cannot load.

## Local delivery and limits

The release WebAssembly bundle, HTML companion, adapter, bundled font and review
brief are generated in `dist/`. Local HTTP and WASM delivery are checked on
127.0.0.1:4387. At the user's request, the same static build is published through
GitHub Pages at `sabertech-development.github.io/saskatchewan-library-trust/`.
No donation collection is enabled, and no visitor scenario data is transmitted
or persisted by the application.

Software checks do not establish charitable status, legal suitability, tax
receipting eligibility, actual library needs, donations, participation, investment
performance, liquidity or long-term sustainability. Those remain explicit open
decisions in the evidence and readiness views.
