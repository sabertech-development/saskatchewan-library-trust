pub const TITLE: &str = "Saskatchewan Library Sustainability Trust";
pub const PRINCIPAL_TARGET: f64 = 250_000_000.;
pub const TARGET_MESSAGE: &str = "A proposed CA$250 million principal fund, raised through corporate donations. This is a fundraising target, not money already secured.";
pub const MESSAGE: &str = "Building a lasting fund for Saskatchewan libraries through donations, responsible investment, and transparent community support.";
pub const STATUS: &str =
    "Proposed concept · No donations collected · Structure and charitable status unresolved";
pub const QUALIFICATION: &str = "“Trust” is a working description. The appropriate Canadian legal and charitable structure remains to be determined. No charitable status, secured donations, library participation, endorsements or approved legal structure are claimed. No tax receipts are promised.";
pub const ROUTES: [(&str, &str); 10] = [
    ("overview", "Why a lasting fund"),
    ("atlas", "Library Atlas"),
    ("donations", "How donations help"),
    ("spending", "Supporting libraries"),
    ("scenarios", "Endowment Scenarios"),
    ("allocation", "Fair community funding"),
    ("governance", "Oversight & reporting"),
    ("evidence", "Evidence & open questions"),
    ("roadmap", "Readiness steps"),
    ("summary", "Review Summary"),
];
pub const QUESTIONS: [(&str, &str, usize); 5] = [
    (
        "Why a lasting funding source?",
        "Libraries plan services over many years. A supplementary long-term fund could help sustain access as needs and costs change. Actual funding gaps still need local evidence.",
        0,
    ),
    (
        "How do donations build the fund?",
        "Corporate donations build a proposed CA$250 million principal fund. Donors receive no shares, equity, repayment rights or investment returns. Immediate-use gifts are accounted for separately.",
        2,
    ),
    (
        "How could support last?",
        "Invest donated principal using a professionally reviewed money-market-style approach. The proposed split sends 50% of net interest to libraries and reinvests 50%. Rates, funding and purchasing power vary.",
        3,
    ),
    (
        "How would communities benefit fairly?",
        "Published eligibility, documented needs, donor-compatible awards and an appeal process would connect support to local priorities.",
        5,
    ),
    (
        "Who would oversee the money?",
        "Trustees or a governing board would oversee purpose, restrictions and professional management. Library authorities retain their service decisions.",
        6,
    ),
];
pub const OUTCOMES: [(&str, &str); 6] = [
    (
        "Opening hours",
        "Illustrative use: maintain evening or weekend access where locally needed.",
    ),
    (
        "Staffing",
        "Illustrative use: support staff capacity for reliable public service.",
    ),
    (
        "Collections",
        "Illustrative use: refresh books, local resources and accessible formats.",
    ),
    (
        "Digital access",
        "Illustrative use: provide public computers, connectivity and digital learning.",
    ),
    (
        "Accessibility",
        "Illustrative use: reduce physical, sensory and communication barriers.",
    ),
    (
        "Programming",
        "Illustrative use: support community-led reading, learning and cultural activities.",
    ),
];

pub struct Node {
    pub name: &'static str,
    pub short: &'static str,
    pub purpose: &'static str,
    pub inputs: &'static str,
    pub outputs: &'static str,
    pub owner: &'static str,
    pub dependencies: &'static str,
    pub readiness: &'static str,
    pub question: &'static str,
    pub kind: usize,
}
pub const NODES: [Node; 7] = [
    Node {
        name: "Corporate donors",
        short: "Corporate donors",
        purpose: "Contribute gifts to the stated library purpose without receiving an ownership interest or financial return.",
        inputs: "Corporate donations toward the proposed CA$250 million principal target, with clearly documented gift terms.",
        outputs: "Long-term capital gifts or separately tracked immediate-use gifts.",
        owner: "Corporate donors and proposed gift-administration team",
        dependencies: "Approved structure, gift-acceptance policy and appropriate receipting arrangements.",
        readiness: "No donations secured or collected",
        question: "Which gift terms would support community needs and responsible stewardship?",
        kind: 0,
    },
    Node {
        name: "The long-term fund",
        short: "Long-term fund",
        purpose: "Hold and manage donated assets for the stated purpose of supporting Saskatchewan libraries.",
        inputs: "Donated principal, additional corporate gifts and interest income.",
        outputs: "The proposed 50% library share of net interest, with 50% reinvested and reserve and gift restrictions respected.",
        owner: "Proposed trustees or governing board; assets are not personally owned by them",
        dependencies: "Legal structure, professional investment mandate, spending policy and donor restrictions.",
        readiness: "Proposed; structure unresolved",
        question: "What structure and spending policy could balance present support and future capacity?",
        kind: 5,
    },
    Node {
        name: "Saskatchewan communities",
        short: "Communities",
        purpose: "Help identify practical service priorities across urban, rural, northern and remote contexts.",
        inputs: "Local experience, consultation and evidence of access barriers.",
        outputs: "Locally identified needs and feedback on service outcomes.",
        owner: "Community members and appropriate library authorities",
        dependencies: "Inclusive consultation, including appropriate engagement with Indigenous communities.",
        readiness: "Illustrative relationships; no participation agreed",
        question: "Whose needs are missing from the evidence, and how will their voices be heard?",
        kind: 6,
    },
    Node {
        name: "Reserves & future capacity",
        short: "Future capacity",
        purpose: "Retain part of available resources to support resilience and the fund's purchasing power over time.",
        inputs: "Retained investment proceeds and approved reserve arrangements.",
        outputs: "A buffer against variability, subject to losses, costs and policy limits.",
        owner: "Proposed governing board with professional advice",
        dependencies: "Spending, liquidity and reserve policies compatible with gift terms.",
        readiness: "Policy requires assessment",
        question: "How should current grants adjust when markets weaken or costs rise?",
        kind: 3,
    },
    Node {
        name: "Oversight & fair allocation",
        short: "Oversight",
        purpose: "Safeguard purpose and donor restrictions while making transparent, evidence-based award decisions.",
        inputs: "Accounts, investment reports, eligible applications and disclosed conflicts.",
        outputs: "Documented decisions, published criteria, independent review and public reporting.",
        owner: "Proposed governing board and independent allocation reviewers",
        dependencies: "Clear delegated responsibilities, conflict controls, appeals and reporting standards.",
        readiness: "Governance arrangements proposed",
        question: "How will decisions be independently checked and explained?",
        kind: 4,
    },
    Node {
        name: "Eligible libraries",
        short: "Libraries",
        purpose: "Receive approved support while retaining their existing ownership, governance and authority over library services.",
        inputs: "Donor-compatible grants and documented service needs.",
        outputs: "Local service delivery and proportionate use-of-funds reporting.",
        owner: "The appropriate library boards and service authorities",
        dependencies: "Published eligibility, confirmed participation and grant agreements.",
        readiness: "No library participation claimed",
        question: "Which library needs are verified, eligible and consistent with donor restrictions?",
        kind: 5,
    },
    Node {
        name: "Practical service outcomes",
        short: "Service outcomes",
        purpose: "Connect funding to opening hours, staffing, collections, digital access, accessibility and programming.",
        inputs: "Approved grants, library expertise and locally agreed priorities.",
        outputs: "Evidence of services supported and community feedback, without collecting patron data.",
        owner: "Appropriate library authorities and service teams",
        dependencies: "Available funding, feasible plans and agreed measures.",
        readiness: "Illustrative outcomes, not promised results",
        question: "What changes would communities value, and how could those changes be reported?",
        kind: 6,
    },
];
pub struct Role {
    pub title: &'static str,
    pub mandate: &'static str,
    pub decisions: &'static str,
    pub controls: &'static str,
}
pub const ROLES: [Role; 6] = [
    Role {
        title: "Trustees / governing board",
        mandate: "Oversee the fund's purpose and assets; trustees do not personally own those assets.",
        decisions: "Approve investment, spending, reserve, gift-acceptance and allocation policies within the eventual legal structure. Monitor risk, costs and liquidity.",
        controls: "Respect donor restrictions; separate ledgers; documented approvals; dual payment authorization; succession planning; publish accounts and policy decisions.",
    },
    Role {
        title: "Professional investment management",
        mandate: "Manage assets within a professionally established mandate. Money-market-style investing is the working approach; specific instruments and mandate require professional assessment.",
        decisions: "Advise on diversification, liquidity, responsible investment, performance measurement, costs and implementation of approved grants.",
        controls: "Transparent fees, appropriate custody, independent reporting, periodic mandate review and oversight by the governing board.",
    },
    Role {
        title: "Allocation reviewers",
        mandate: "Apply published criteria to verified, eligible needs within available and donor-compatible resources.",
        decisions: "Document scoring, recommend awards, give written reasons and provide a fair review or appeal route.",
        controls: "Disclose conflicts; recuse affected reviewers; separate assessment from payment approval; publish understandable allocation results.",
    },
    Role {
        title: "Library authorities",
        mandate: "Retain existing ownership and governance arrangements, including decisions about library services.",
        decisions: "Identify needs, set service priorities, accept appropriate grant terms, deliver services and report use of support.",
        controls: "Proportionate reporting, local accountability, privacy protection and respect for library independence.",
    },
    Role {
        title: "Independent oversight",
        mandate: "Provide independent scrutiny of financial reporting, controls and use of restricted funds.",
        decisions: "Agree an appropriate external audit or review scope, report findings and track remediation.",
        controls: "Independence from management and award decisions. The proposed review does not guarantee future funding or performance.",
    },
    Role {
        title: "Legal and charitable advisers",
        mandate: "Determine an appropriate Canadian legal and charitable structure and assess applicable requirements.",
        decisions: "Review charitable eligibility, receipting, grant eligibility, investment powers, spending requirements, gift terms and governance documents before launch.",
        controls: "Written professional advice and documented decisions. No charitable tax receipts are promised before relevant status and arrangements are established.",
    },
];
pub struct Evidence {
    pub claim: &'static str,
    pub status: &'static str,
    pub detail: &'static str,
    pub next: &'static str,
    pub source: &'static str,
}
pub const EVIDENCE: [Evidence; 8] = [
    Evidence {
        claim: "Money-market-style approach",
        status: "Working approach",
        detail: "The proposal would seek interest income from professionally selected money-market-style instruments. These are investment products; low rates can fail to keep pace with inflation. No product, yield or principal guarantee is promised.",
        next: "Obtain advice on suitable instruments, liquidity, custody, costs and risk before investing donated assets.",
        source: "https://www.investright.org/understand-investing/types-of-investments/common-investments/mutual-funds/",
    },
    Evidence {
        claim: "Provincial library context",
        status: "External context",
        detail: "The Government of Saskatchewan describes municipal, regional and northern library arrangements. That context does not establish any funding gap, participation or endorsement.",
        next: "Consult appropriate library authorities and gather current service and budget evidence.",
        source: "https://www.saskatchewan.ca/residents/education-and-learning/library-system-in-saskatchewan",
    },
    Evidence {
        claim: "Legal and charitable structure",
        status: "Open decision",
        detail: "Trust is a working description. No legal structure or charitable status is established by this proposal.",
        next: "Obtain Canadian professional advice on the appropriate structure and obligations.",
        source: "",
    },
    Evidence {
        claim: "Tax receipts",
        status: "Open decision",
        detail: "CRA explains that qualified donees may issue official donation receipts. This demo makes no claim that the proposed fund is qualified and promises no receipts.",
        next: "Establish applicable status and gift/receipting arrangements before making representations.",
        source: "https://www.canada.ca/en/revenue-agency/services/charities-giving/giving-charity-information-donors/before-you-give.html",
    },
    Evidence {
        claim: "Needs and library participation",
        status: "Not established",
        detail: "No provincial funding gap or library participation is verified here. Service profiles are illustrative.",
        next: "Invite consultation and verify local needs, eligibility and consent before considering grants.",
        source: "",
    },
    Evidence {
        claim: "Donations and donor restrictions",
        status: "Not established",
        detail: "The proposed principal fundraising target is CA$250 million, raised through corporate donations. No gifts or donor commitments are represented as secured or collected.",
        next: "Develop a gift-acceptance policy and separate restricted-capital and immediate-use accounting.",
        source: "",
    },
    Evidence {
        claim: "Investment and spending policies",
        status: "Open decision",
        detail: "Money-market-style investment is the working approach. Specific instruments, costs, reserves and legal suitability require assessment; the proposed net-interest split is 50% for libraries and 50% reinvested. Scenario rates are hypothetical and editable, not quoted market yields.",
        next: "Assess risks, liquidity, inflation, allowable capital spending and applicable distribution requirements.",
        source: "",
    },
    Evidence {
        claim: "Governance and reporting",
        status: "Proposed",
        detail: "Roles, conflicts controls, allocation criteria, independent oversight and reporting are proposals, not operating arrangements.",
        next: "Approve governing documents, responsibilities, reporting standards and an independent review process.",
        source: "",
    },
];
pub const STAGES: [(&str, &str); 5] = [
    (
        "Listen and establish needs",
        "Consult library authorities and communities; substantiate service gaps and participation preferences. No engagement is represented as completed.",
    ),
    (
        "Determine the structure",
        "Obtain legal and charitable advice, define the purpose and confirm governance and receipting arrangements. Trust remains a working description.",
    ),
    (
        "Set stewardship policies",
        "Approve gift restrictions, investment management, spending, reserves, costs, controls and independent oversight.",
    ),
    (
        "Establish fair grant arrangements",
        "Publish eligibility, evidence requirements, allocation criteria, reasons, appeals and use-of-funds reporting. Respect library service authority.",
    ),
    (
        "Consider a separate future launch",
        "Only after approvals and operational readiness would donation collection or grantmaking be considered. This local demonstration does neither.",
    ),
];
pub const TOUR: [(usize, &str, &str); 5] = [
    (0, "Why a lasting fund?", QUESTIONS[0].1),
    (2, "Donations build shared capacity", QUESTIONS[1].1),
    (3, "Balance present and future support", QUESTIONS[2].1),
    (5, "Reach communities fairly", QUESTIONS[3].1),
    (6, "Make oversight visible", QUESTIONS[4].1),
];

pub fn narrative_text() -> String {
    let mut t = format!(
        "{TITLE}\n\n{MESSAGE}\n\n{TARGET_MESSAGE}\n\n{QUALIFICATION}\n\nCorporate donations → Long-term principal fund → Interest income → Library support + reinvestment\n50% of net interest supports libraries and 50% is reinvested. Performance and funding vary.\n"
    );
    for (q, a, _) in QUESTIONS {
        t.push_str(&format!("\n{q}\n{a}\n"));
    }
    t.push_str("\nOWNERSHIP AND GIFT TERMS\nDonors receive no shares, equity, repayment rights or investment returns. The proposed fund holds and manages donated assets for its stated purpose. Trustees oversee those assets; they do not personally own them. Libraries retain their existing ownership, governance and service decisions. Long-term capital gifts and immediate-use gifts are kept separate and used according to donor restrictions. No tax receipts are promised.\n\nILLUSTRATIVE OUTCOMES\n");
    for (title, body) in OUTCOMES {
        t.push_str(&format!("{title}: {body}\n"));
    }
    for r in ROLES {
        t.push_str(&format!(
            "\n{}\n{}\n{}\n{}\n",
            r.title, r.mandate, r.decisions, r.controls
        ));
    }
    for e in EVIDENCE {
        t.push_str(&format!(
            "\n{} — {}\n{}\nNext: {}\n{}\n",
            e.claim, e.status, e.detail, e.next, e.source
        ));
    }
    for (title, body) in STAGES {
        t.push_str(&format!("\n{title}\n{body}\n"));
    }
    t
}
