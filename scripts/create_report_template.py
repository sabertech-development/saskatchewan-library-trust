"""Create the designed, vector PDF master used by browser-local report exports."""
from pathlib import Path
from reportlab.pdfgen import canvas
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.lib.colors import HexColor
from reportlab.platypus import Paragraph
from reportlab.lib.styles import ParagraphStyle

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'assets' / 'report-template.pdf'
W, H = 612, 792
BG, PANEL, LINE = '#030509', '#091622', '#254456'
INK, MUTED, CYAN, ICE = '#E8F2FA', '#A8BECE', '#3DCAFF', '#B4EDFF'
pdfmetrics.registerFont(TTFont('Space', str(ROOT/'assets/fonts/SpaceGrotesk.ttf')))
pdfmetrics.registerFont(TTFont('SpaceMedium', str(ROOT/'assets/fonts/SpaceGrotesk-Medium.ttf')))
c = canvas.Canvas(str(OUT), pagesize=(W,H), pageCompression=1)
c.setTitle('Saskatchewan Library Sustainability Trust - Review')
c.setAuthor('Saber Tech Inc.')
c.setSubject('Proposed corporate-donation fund and hypothetical library support scenarios')

def text(x,y,t,size=11,color=INK,font='Space'):
    c.setFillColor(HexColor(color));c.setFont(font,size);c.drawString(x,H-y-size,t)
def paragraph(x,y,w,t,size=11,color=MUTED,leading=None):
    style=ParagraphStyle('body',fontName='Space',fontSize=size,leading=leading or size*1.45,textColor=HexColor(color),spaceAfter=0)
    p=Paragraph(t,style);_,h=p.wrap(w,1000);p.drawOn(c,x,H-y-h);return y+h
def line(x1,y1,x2,y2,color=LINE,width=.7):
    c.setStrokeColor(HexColor(color));c.setLineWidth(width);c.line(x1,H-y1,x2,H-y2)
def box(x,y,w,h):
    c.setFillColor(HexColor(PANEL));c.setStrokeColor(HexColor(LINE));c.setLineWidth(.7);c.rect(x,H-y-h,w,h,fill=1,stroke=1);line(x,y,x+22,y,CYAN,1)
def eyebrow(x,y,t):text(x,y,t.upper(),8,CYAN)
def base(label,title,sub=''):
    c.setFillColor(HexColor(BG));c.rect(0,0,W,H,fill=1,stroke=0)
    text(42,24,'Prepared by Saber Tech Inc.',10,ICE,'SpaceMedium')
    text(401,27,'PROPOSAL FOR DISCUSSION',7,MUTED)
    line(42,49,570,49)
    eyebrow(42,70,label);text(42,89,title,27,INK,'SpaceMedium')
    if sub:paragraph(42,127,528,sub,10.5)
def footer():
    line(42,743,570,743);text(42,755,'SASKATCHEWAN LIBRARY SUSTAINABILITY TRUST',7,MUTED)
def page():footer();c.showPage()

# 1 / Concise executive cover.
base('Saskatchewan Library Sustainability Trust / proposed','Saskatchewan libraries.','')
text(42,130,'Lasting community capacity.',30,CYAN,'SpaceMedium')
paragraph(42,187,500,'Building a lasting fund for Saskatchewan libraries through donations, responsible investment, and transparent community support.',16,INK,23)
box(42,282,528,130);eyebrow(62,301,'Proposed corporate donation target')
text(62,322,'CA$250 million',43,CYAN,'SpaceMedium')
paragraph(62,378,476,'Principal to raise - not secured donations or money already held.',11)
for x,value,title,copy in [(42,'50%','Library support','Of positive net cash interest after costs.'),(312,'50%','Reinvestment','Of positive net cash interest after costs.')]:
    box(x,432,258,112);text(x+20,447,value,35,ICE,'SpaceMedium');text(x+20,491,title,13,INK,'SpaceMedium');paragraph(x+20,514,220,copy,9.5)
eyebrow(42,568,'How the proposal works')
for i,(title,detail) in enumerate([('Donations','Corporate gifts'),('Principal fund','Long-term assets'),('Interest','After costs'),('Library support','With reinvestment')]):
    x=42+i*134;box(x,593,126,62);text(x+10,605,title,11,INK,'SpaceMedium');paragraph(x+10,628,108,detail,9)
paragraph(42,678,528,'Trust is a working description. The Canadian legal and charitable structure remains to be determined. No charitable status, secured donations, library participation, endorsements or tax receipts are claimed.',9.5,MUTED,13.5)
page()

# Dynamic pages are completed from the selected, validated Rust scenario.
base('The funding model','From interest to library support.','Working approach: money-market-style investing, subject to professional review. Proposed split: 50% to libraries, 50% reinvested, after costs.')
page()
base('Long-term capacity','The fund over time.','Nominal assets and purchasing power tell different stories. These are hypothetical results from the selected inputs, not forecasts.')
page()
base('Community support','Connect funding to everyday access.','A lasting fund could help libraries plan access across years. These uses are illustrative; actual needs and participation must be verified.')
page()
base('Stress testing','See what weaker conditions change.','These comparisons change selected assumptions while retaining the same horizon, costs, inflation, need and gift terms. They are not probabilities or forecasts.')
page()

# 6 / Governance and clear next steps.
base('Oversight and readiness','Stewardship with clear responsibilities.','The governing arrangements below are proposed. Trustees oversee donated assets; they do not personally own them.')
roles=[
 ('Trustees / governing board','Set purpose, gift, investment, spending and reserve policies. Monitor risk, liquidity and costs. Authorize support under governing documents.'),
 ('Professional management','Manage investments within an approved mandate, with suitable custody, transparent costs and regular reporting to the board.'),
 ('Allocation reviewers','Publish eligibility and weigh verified need, access barriers and community context, including rural and northern service. Record reasons and provide a fair review route.'),
 ('Library authorities','Keep existing ownership, governance and service decisions. Set local priorities and report proportionately on the use of grants.'),
 ('Independent oversight','Review accounts, controls and restricted-fund use independently. Publish findings and report action on them.'),
 ('Legal / charitable advisers','Determine the appropriate Canadian structure and applicable requirements before any promise of receipts or future fundraising launch.'),
]
for i,(title,body) in enumerate(roles):
    x=42+(i%2)*270;y=176+(i//2)*131;box(x,y,258,116);text(x+16,y+15,title,12,INK,'SpaceMedium');paragraph(x+16,y+40,225,body,10,leading=14)
eyebrow(42,590,'Evidence before a future launch')
paragraph(42,611,528,'<b>Listen and verify needs</b>  /  Determine the structure  /  Approve stewardship policies  /  Publish fair grant arrangements  /  Consider a separate future launch',11,INK,17)
paragraph(42,672,528,'Donor restrictions remain binding. Declare and manage conflicts; publish financial performance, costs, awards and practical outcomes. No donation collection or grantmaking is activated by this presentation.',10,MUTED,14.5)
page()

base('Selected inputs','Every starting figure is editable.','This report records the scenario selected at download. The proposed CA$250 million target is distinct from secured capital; rates and costs are hypothetical.')
page()

# 8 / Methods and sources. Full ledger follows as a dynamic appendix.
base('Read the illustration correctly','Calculation notes and evidence.','The same Rust engine supplies the website, charts and annual ledger. Selected assumptions appear on the preceding page.')
notes=[
 ('Interest is not total return','Cash interest = (opening fund + annual long-term gifts) x interest yield. Capital value changes use the same base and a separate assumption; unrealized gains are not cash interest.'),
 ('Costs come before sharing','Investment costs use the invested balance. Administration rises with inflation from year 2. Net interest = max(0, gross cash interest - all costs due). Unpaid costs are reported.'),
 ('Policy and actual grants','The proposed budget uses the selected share of positive net interest. Grants are capped by eligible need and assets above reserve and donor-restriction floors. Unused capacity stays in the fund.'),
 ('Principal and reserve floors','The reserve floor is a percentage of opening assets inside the fund, not extra money. With principal protection, the retained floor is the greater of the reserve and cumulative contributed capital. Losses and permitted costs can still erode principal.'),
 ('Separate immediate-use gifts','Annual gifts arrive at the start of each year, including year 1. Immediate-use gifts stay in a separate uninvested ledger, meet eligible need first, and never pay fund investment or administration costs. Unused amounts carry forward separately.'),
 ('Comparison and purchasing power','The capital-spending comparison uses spending rate x opening balance before new gifts and returns. It may draw on earlier gains or permitted capital. Real balance divides closing assets by (1 + inflation)^year. Cumulative support is nominal.'),
]
for i,(title,body) in enumerate(notes):
    x=42+(i%2)*270;y=176+(i//2)*151;eyebrow(x,y,title);paragraph(x,y+21,250,body,10.2,leading=14.5)
line(42,636,570,636);eyebrow(42,650,'Context sources - checked 18 November 2026')
sources=[
 ('Government of Saskatchewan - library system','https://www.saskatchewan.ca/residents/education-and-learning/library-system-in-saskatchewan'),
 ('Canada Revenue Agency - before you give','https://www.canada.ca/en/revenue-agency/services/charities-giving/giving-charity-information-donors/before-you-give.html'),
 ('BC Securities Commission - mutual funds','https://www.investright.org/understand-investing/types-of-investments/common-investments/mutual-funds/'),
]
for j,(title,url) in enumerate(sources):
    y=672+j*16;text(42,y,title,9.5,ICE);c.linkURL(url,(42,H-y-14,560,H-y+2),relative=0)
text(42,724,'Sources provide context; they do not verify or endorse the proposed fund.',8,MUTED)
page();c.save();print(OUT)
