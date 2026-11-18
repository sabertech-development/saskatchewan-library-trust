import {PDFDocument,rgb} from 'pdf-lib';
import fontkit from '@pdf-lib/fontkit';
const W=612,H=792,M=42,CW=528;
const hex=s=>rgb(parseInt(s.slice(0,2),16)/255,parseInt(s.slice(2,4),16)/255,parseInt(s.slice(4,6),16)/255);
const C={bg:hex('030509'),panel:hex('091622'),line:hex('254456'),ink:hex('E8F2FA'),muted:hex('A8BECE'),cyan:hex('3DCAFF'),ice:hex('B4EDFF'),coral:hex('F79C8E'),blue:hex('397A9F')};
const money=v=>'CA$'+Math.round(v).toLocaleString('en-CA');
const compact=v=>{const a=Math.abs(v);for(const [u,n]of [[1e15,'Q'],[1e12,'T'],[1e9,'B'],[1e6,'M'],[1e3,'K']])if(a>=u)return (v<0?'-':'')+'CA$'+(a/u).toFixed(2)+n;return money(v);};
const pct=v=>(v*100).toLocaleString('en-CA',{maximumFractionDigits:3})+'%';
const clean=s=>String(s).replace(/[‐‑–—−]/g,'-').replace(/[‘’]/g,"'").replace(/[“”]/g,'"').replace(/→/g,'>');

export async function createReviewPdf(data,resources){
 if(data.version!==1||data.hypothetical!==true||!data.projection?.years?.length)throw Error('A validated scenario is required for this PDF.');
 const doc=await PDFDocument.load(resources.template);doc.registerFontkit(fontkit);
 const font=await doc.embedFont(resources.regular,{subset:true}),bold=await doc.embedFont(resources.medium,{subset:true});
 const pages=doc.getPages(),i=data.inputs,ys=data.projection.years,first=ys[0],last=ys.at(-1);
 const created=resources.createdAt?new Date(resources.createdAt):new Date();
 const date=new Intl.DateTimeFormat('en-CA',{timeZone:'America/New_York',year:'numeric',month:'long',day:'numeric'}).format(created);
 doc.setTitle('Saskatchewan Library Sustainability Trust - Review');doc.setAuthor('Saber Tech Inc.');doc.setSubject('Proposed corporate-donation fund and selected hypothetical scenario');doc.setCreator('Saber Tech Inc.');doc.setCreationDate(created);doc.setModificationDate(created);
 function text(p,x,y,s,size=11,color=C.ink,weight=false,maxWidth){s=clean(s);let f=weight?bold:font;if(maxWidth)while(size>7&&f.widthOfTextAtSize(s,size)>maxWidth)size-=.25;p.drawText(s,{x,y:H-y-size,size,font:f,color});}
 function para(p,x,y,w,s,size=11,color=C.muted,leading=size*1.45){
   const words=clean(s).split(/\s+/);let line='';for(const word of words){const candidate=line?line+' '+word:word;if(font.widthOfTextAtSize(candidate,size)>w&&line){text(p,x,y,line,size,color);y+=leading;line=word;}else line=candidate;}
   if(line){text(p,x,y,line,size,color);y+=leading;}return y;
 }
 function line(p,x,y,x2,y2,color=C.line,thickness=.7,dash){p.drawLine({start:{x,y:H-y},end:{x:x2,y:H-y2},color,thickness,...(dash?{dashArray:dash}:{})});}
 function box(p,x,y,w,h){p.drawRectangle({x,y:H-y-h,width:w,height:h,color:C.panel,borderColor:C.line,borderWidth:.7});line(p,x,y,x+22,y,C.cyan,1);}
 function tag(p,x,y,s){text(p,x,y,s.toUpperCase(),8,C.cyan);}
 function metric(p,x,y,w,label,value,sub=''){box(p,x,y,w,83);text(p,x+13,y+11,label,9,C.muted,false,w-26);text(p,x+13,y+30,value,24,C.cyan,true,w-26);if(sub)text(p,x+13,y+64,sub,8,C.muted,false,w-26);}
 function chart(p,rect,series,unit='CAD',startAtZero=true){
   const {x,y,w,h}=rect,px=x+56,py=y+24,pw=w-68,ph=h-60;
   const all=series.flatMap(s=>s.values);if(all.every(v=>v===0)){text(p,x+22,y+60,'All plotted amounts are CA$0.',13,C.ice,true);text(p,x+22,y+84,'No nonzero amounts in this selected scenario.',10,C.muted);return;}let max=Math.max(1,...all),min=startAtZero?Math.min(0,...all):Math.min(...all);max*=1.08;
   const scale=v=>py+ph-(v-min)/(max-min)*ph;
   text(p,x,y,unit,8,C.muted);
   for(let t=0;t<=4;t++){const v=min+(max-min)*t/4,yy=scale(v);line(p,px,yy,px+pw,yy);text(p,x,yy-5,compact(v),8,C.muted,false,50);}
   const count=series[0].values.length-1;
   for(const n of [...new Set([0,Math.round(count/4),Math.round(count/2),Math.round(count*.75),count])]){const xx=px+n/Math.max(1,count)*pw;text(p,xx-9,py+ph+12,String(n+(rect.startYear||0)),8,C.muted);}
   text(p,px+pw-24,py+ph+25,'YEAR',7,C.muted);
   for(const s of series){if(s.values.length===1)p.drawCircle({x:px,y:H-scale(s.values[0]),size:2.5,color:s.color});for(let n=1;n<s.values.length;n++)line(p,px+(n-1)/Math.max(1,count)*pw,scale(s.values[n-1]),px+n/Math.max(1,count)*pw,scale(s.values[n]),s.color,1.8,s.dashed?[4,3]:undefined);}
 }
 function ledgerMoney(v){return Math.abs(v)>=1e10?compact(v):money(v);}
 function legend(p,x,y,items){let at=x;for(const [label,color,dashed]of items){line(p,at,y+5,at+19,y+5,color,2,dashed?[4,3]:undefined);text(p,at+26,y,label,9,C.muted);at+=font.widthOfTextAtSize(label,9)+49;}}
 function pageBase(p,label,title,sub){p.drawRectangle({x:0,y:0,width:W,height:H,color:C.bg});text(p,42,24,'Prepared by Saber Tech Inc.',10,C.ice,true);text(p,401,27,'PROPOSAL FOR DISCUSSION',7,C.muted);line(p,42,49,570,49);tag(p,42,70,label);text(p,42,89,title,27,C.ink,true);para(p,42,127,528,sub,10.5);line(p,42,743,570,743);text(p,42,755,'SASKATCHEWAN LIBRARY SUSTAINABILITY TRUST',7,C.muted);}

 // Cover: identify the exported selection without changing the proposed target.
 text(pages[0],42,724,'Prepared '+date+'  |  Selected scenario: '+i.years+' years',8,C.muted);

 // 2. Selected-year cash flow, separated from valuation and immediate gifts.
 const p2=pages[1];tag(p2,42,172,i.interest_only?'SELECTED HYPOTHETICAL INTEREST-SHARING SCENARIO':'CAPITAL-SPENDING COMPARISON - NOT THE PROPOSED POLICY');
 metric(p2,42,194,170,'Year 1 gross cash interest',compact(first.interest_income));metric(p2,221,194,170,'Year 1 costs due',compact(first.costs_due));metric(p2,400,194,170,'Positive net interest',compact(first.net_interest));
 text(p2,42,302,'The selected annual allocation',18,C.ink,true);
 const flow=[['Library budget',first.spending_target,C.ice],['Actual fund grant',first.endowment_distribution,C.cyan],['Net interest retained',first.retained_interest,C.blue]];
 const mx=Math.max(1,...flow.map(v=>v[1]));for(let n=0;n<flow.length;n++){const [label,value,color]=flow[n],y=343+n*57;text(p2,42,y,label,11,C.ink,true);p2.drawRectangle({x:190,y:H-y-21,width:Math.max(1,260*value/mx),height:22,color});text(p2,460,y+1,compact(value),13,color,true,110);}
 para(p2,42,493,528,'Corporations give without shares, repayment rights or returns. The proposed fund holds donated assets for its stated purpose.',9.5);
 para(p2,42,526,528,i.interest_only?'Selected library share: '+pct(i.library_interest_share)+'. The proposed policy is 50% / 50%; edited shares are hypothetical comparisons. Actual grants can be lower than the budget because of need, reserves or donor restrictions.':'This selected comparison sets a budget of '+pct(i.spending_rate)+' of opening annual assets. It may use accumulated gains or permitted capital. It does not replace the proposed 50% / 50% net-interest policy.',10.5);
 box(p2,42,596,258,109);tag(p2,56,610,'Capital value is separate');text(p2,56,631,money(first.valuation_change),20,C.ice,true,230);para(p2,56,663,230,'Year 1 valuation change is not cash interest. Losses can outweigh reinvestment.',9.5);
 box(p2,312,596,258,109);tag(p2,326,610,'Immediate-use gifts stay separate');text(p2,326,631,money(first.immediate_distribution),20,C.ice,true,230);para(p2,326,663,230,'Year 1 immediate library support. Unused carryover: '+money(first.immediate_balance)+'.',9.5);
 para(p2,42,717,528,'Retained interest is already included in closing assets; it is never added twice. Unpaid year 1 costs: '+money(first.unpaid_costs)+'.',8.5);

 // 3. Time series: full selected horizon, with nominal and real values.
 const p3=pages[2];metric(p3,42,174,258,'Opening donated principal',compact(i.capital),'Illustration, not secured funds');metric(p3,312,174,258,'Final nominal fund / year '+i.years,compact(last.closing));
 box(p3,42,282,528,292);chart(p3,{x:55,y:298,w:496,h:242},[{values:[i.capital,...ys.map(y=>y.closing)],color:C.cyan},{values:[i.capital,...ys.map(y=>y.real_closing)],color:C.ice,dashed:true}]);legend(p3,99,546,[['Nominal fund',C.cyan],['Purchasing power',C.ice,true]]);
 metric(p3,42,596,258,'Final purchasing power',compact(last.real_closing),'In starting-year dollars');metric(p3,312,596,258,'Real value of contributions',compact(last.real_contributed_principal),'Valued at each gift date');
 para(p3,42,701,528,last.real_closing<last.real_contributed_principal?'In this scenario, final purchasing power is below the real value of contributions. Reinvestment does not guarantee preservation of capital or purchasing power.':'This scenario finishes at or above the real value of contributions. That result depends on its assumptions and does not guarantee capital preservation or permanent funding.',10);

 // 4. Annual support and needs, then concrete service outcomes.
 const p4=pages[3];metric(p4,42,174,258,'Cumulative library support',compact(last.cumulative_support),'Nominal; includes both funding sources');metric(p4,312,174,258,'Final-year unmet funding need',compact(last.gap));
 box(p4,42,278,528,230);chart(p4,{x:55,y:289,w:496,h:184,startYear:1},[{values:ys.map(y=>y.libraries),color:C.cyan},{values:ys.map(y=>y.library_need),color:C.coral,dashed:true}]);legend(p4,97,480,[['Total library support',C.cyan],['Illustrative need',C.coral,true]]);
 const uses=[['Opening hours','Evening and weekend access'],['Staffing','Reliable public service teams'],['Collections','Books and accessible formats'],['Digital access','Computers and connectivity'],['Accessibility','Fewer barriers to library use'],['Programming','Community learning and culture']];
 for(let n=0;n<uses.length;n++){const x=42+n%2*270,y=531+Math.floor(n/2)*59;box(p4,x,y,258,49);text(p4,x+12,y+9,uses[n][0],11,C.ice,true);text(p4,x+12,y+28,uses[n][1],9,C.muted,false,232);}
 para(p4,42,720,528,'The annual ledger separates fund grants and immediate-use support. No library participation or service result is promised.',8.5);

 // 5. Comparisons are calculated by Rust, not by the PDF layer.
 const p5=pages[4],cs=data.comparisons,maxReal=Math.max(1,...cs.filter(c=>c.projection).map(c=>c.projection.years.at(-1).real_closing));
 tag(p5,42,179,'Final purchasing power / starting-year CAD');
 for(let n=0;n<cs.length;n++){const c=cs[n],y=208+n*56,end=c.projection?.years.at(-1),color=n===0?C.cyan:n>=3?C.coral:C.blue;text(p5,42,y,c.name,10.5,C.ink,true);if(!end){text(p5,190,y,'Outside supported numeric range',10,C.muted);continue;}p5.drawRectangle({x:190,y:H-y-22,width:Math.max(1,265*end.real_closing/maxReal),height:23,color});text(p5,467,y+3,compact(end.real_closing),11,color,true,103);}
 line(p5,42,502,570,502);text(p5,42,516,'Comparison',9,C.muted);text(p5,240,516,'Cumulative support',9,C.muted);text(p5,415,516,'Final-year gap',9,C.muted);
 for(let n=0;n<cs.length;n++){const c=cs[n],end=c.projection?.years.at(-1),y=541+n*27;text(p5,42,y,c.name,10,C.ink);if(!end){text(p5,240,y,'Not calculated: numeric limit',10,C.muted);continue;}text(p5,240,y,money(end.cumulative_support),10,C.ice,false,160);text(p5,415,y,money(end.gap),10,C.muted,false,155);line(p5,42,y+20,570,y+20);}
 para(p5,42,686,528,'Weak interest: 0.5% cash yield and no value change. Continuing losses: 0.5% cash yield and -8% annual value change. Early loss: -20% value change in year 1, then selected assumptions. No additional gifts sets both annual donation sources to zero.',9.1,C.muted,12.6);

 // 7. Auditable selected assumptions. No hidden defaults in the export.
 const p7=pages[6],fields=[
 ['Opening donated principal',money(i.capital)],['Annual long-term donations',money(i.annual_capital_donations)],['Annual immediate-use donations',money(i.annual_immediate_donations)],['Annual cash interest yield',pct(i.interest_rate)],['Annual capital value change',pct(i.return_rate)],['First-year value change override',i.first_year_return===null?'None':pct(i.first_year_return)],['Annual investment cost rate',pct(i.investment_cost_rate)],['First-year administration cost',money(i.administration)],['Annual inflation',pct(i.inflation)],['Library share of positive net interest',pct(i.library_interest_share)],['Selected spending approach',i.interest_only?'Share positive net interest':'Capital-spending comparison'],['Comparison spending rate',pct(i.spending_rate)+(i.interest_only?' / not applied':' / opening balance')],['Reserve floor / opening balance',pct(i.reserve_rate)],['Gift terms on contributed principal',i.protect_principal?'Restricted from grants':'Capital grants permitted'],['First-year annual library need',money(i.library_need)],['Projection horizon',i.years+' years']
 ];
 for(let n=0;n<fields.length;n++){const y=179+n*28;if(n%2===0)p7.drawRectangle({x:42,y:H-y-26,width:528,height:26,color:C.panel});text(p7,53,y+6,fields[n][0],10,C.muted);text(p7,340,y+6,fields[n][1],10,C.ice,false,218);}
 para(p7,42,654,528,'Equal annual donations arrive at the start of each year. Need and administration rise with inflation from year 2; donation amounts stay nominally constant. Immediate-use carryover remains uninvested and separate.',10);
 para(p7,42,709,528,'All starting figures are editable illustrations. Changing a hypothetical gift term does not release an actual donor restriction. No tax treatment is assumed.',9.5);

 // Annual figures: paginate to keep every row readable, even at a 50-year horizon.
 const widths=[24,72,68,72,76,76,70,70],headers=['Year','Fund grant','Immediate','Reinvested','Fund balance','Real balance','Need gap','Cumulative'];
 for(let start=0;start<ys.length;start+=24){const p=doc.addPage([W,H]);pageBase(p,'Annual scenario ledger','The numbers behind the charts.','Nominal CAD except real balance (starting-year dollars). Large values use B = billion, T = trillion and Q = quadrillion. Displayed values are rounded.');
 let x=42;for(let n=0;n<headers.length;n++){text(p,x+3,181,headers[n],7.5,C.ice,true,widths[n]-6);x+=widths[n];}line(p,42,201,570,201);
 for(let row=0;row<Math.min(24,ys.length-start);row++){const y=ys[start+row],top=208+row*20;if(row%2===0)p.drawRectangle({x:42,y:H-top-20,width:528,height:20,color:C.panel});const values=[String(y.year),ledgerMoney(y.endowment_distribution),ledgerMoney(y.immediate_distribution),ledgerMoney(y.retained_interest),ledgerMoney(y.closing),ledgerMoney(y.real_closing),ledgerMoney(y.gap),ledgerMoney(y.cumulative_support)];let xx=42;for(let col=0;col<values.length;col++){text(p,xx+3,top+5,values[col],8,col===0?C.ink:C.muted,false,widths[col]-6);xx+=widths[col];}}
 para(p,42,708,528,'Fund grant and immediate-use support are separate. Reinvested net interest is already inside the fund balance. Cumulative support is nominal; investment performance and available funding vary.',9.5);
 }
 for(const [n,p]of doc.getPages().entries())text(p,526,755,String(n+1).padStart(2,'0')+' / '+String(doc.getPageCount()).padStart(2,'0'),8,C.ice);
 return doc.save();
}
