# Verification record

Verification date: 18 November 2026.

## Financial and allocation engines

Fifteen Rust tests cover the default waterfall and both rate denominators,
zero capital, zero need, no income, lower income, operating deficits, unfunded
deficits, reserves above target, invalid values and percentages, cash
reconciliation across stress combinations, nonnegative distributions, pool
and applicant caps, baseline scaling, redistribution, excess balances,
zero-score fallback and empty applicants.

Four independently executed Unison checks confirm the base, lower-yield,
zero-income and unfunded-deficit policy results. The Unison verifier uses
numeric Float comparisons explicitly, rather than universal structural ordering.

## Interface and delivery

Rust UI interaction tests exercise every route at 390, 820 and 1440 points,
all navigation entries, atlas overlays and node selection, mobile menu
navigation, the six-step tour, keyboard focus and activation, scenario changes
across dependent views, and reset. Go tests verify loader generation and that
the local server accepts read-only requests.

The actual UI meshes were rendered and visually inspected at desktop and
mobile widths. A content-width overflow and collapsed mobile navigation were
found and corrected. There are no decorative animations. Financial, business
and library examples are labelled, and no unsupported endorsement is asserted.

The browser access host was unavailable. Native Edge access later returned a
URL-confidence safety error, so browser automation stopped. Offscreen render
checks and Rust input-event tests are not substitutes for full browser and
screen-reader testing. No claim of WCAG conformance is made. The canvas does
not supply conventional semantic HTML tables; a direct plain-text review brief
provides an accessible reading and print alternative.

Local delivery was checked for the exact title and WebAssembly MIME type on
port 4387. Port 4173 belongs to an unrelated pre-existing app and is not used.

## Scope limits

Passing software tests does not verify acquisition economics, investor appetite,
Canadian tax treatment, legal enforceability, actual provincial library needs,
multi-year sustainability, or participation. Those remain in the evidence
register and require independent assessment.
