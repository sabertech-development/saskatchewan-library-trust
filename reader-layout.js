// Progressive enhancement: screen-sized, keyboard-accessible views. No model math.
(()=>{
const main=document.getElementById('main'),globalNav=document.querySelector('header nav');
const el=(tag,cls,html)=>{const e=document.createElement(tag);if(cls)e.className=cls;if(html!==undefined)e.innerHTML=html;return e;};
const section=(id)=>document.getElementById(id);
let sequence=0;
function tabSet(parent,entries,label){
 const list=el('div','tab-list');list.setAttribute('role','tablist');list.setAttribute('aria-label',label);parent.append(list);
 const pages=[],buttons=[];
 function choose(i,focus=false){buttons.forEach((b,j)=>{b.setAttribute('aria-selected',String(i===j));b.tabIndex=i===j?0:-1;pages[j].hidden=i!==j;});if(focus)buttons[i].focus();}
 entries.forEach(([name,content],i)=>{const id='review-tab-'+(++sequence),b=el('button');b.type='button';b.textContent=name;b.id=id;b.setAttribute('role','tab');b.setAttribute('aria-controls',id+'-panel');
 const page=el('div','tab-page');page.id=id+'-panel';page.setAttribute('role','tabpanel');page.setAttribute('aria-labelledby',id);page.tabIndex=0;
 if(typeof content==='string')page.innerHTML=content;else if(Array.isArray(content))page.append(...content);else if(content)page.append(content);
 list.append(b);parent.append(page);buttons.push(b);pages.push(page);b.addEventListener('click',()=>choose(i));
 b.addEventListener('keydown',e=>{let n=i;if(e.key==='ArrowRight')n=(i+1)%entries.length;else if(e.key==='ArrowLeft')n=(i+entries.length-1)%entries.length;else if(e.key==='Home')n=0;else if(e.key==='End')n=entries.length-1;else return;e.preventDefault();choose(n,true);});});choose(0);return {choose,pages,buttons};
}
const originalIntro=[...main.children].filter(e=>e.tagName!=='SECTION');
const lead=main.querySelector('.lead').textContent,qualification=main.querySelector('.note').textContent;
const outcomes=section('fairness').querySelector('.grid');outcomes.classList.add('compact-rows');
const modelQuestions=['purpose','donations','spending','fairness','oversight'].map(id=>[section(id).querySelector('h2').textContent,section(id).querySelector('p').textContent]);
const stages=[...main.querySelectorAll(':scope > section')].find(s=>!s.id);stages.id='readiness';
const source={};for(const s of main.querySelectorAll(':scope >section')){source[s.id]=[...s.children];s.replaceChildren();s.className='screen';s.hidden=true;}
originalIntro.forEach(e=>e.remove());
document.querySelector('header>p').outerHTML='<p class="brand">SASKATCHEWAN<small>LIBRARY SUSTAINABILITY TRUST</small></p>';
const artifact='<svg class="context-art" viewBox="0 0 400 230" aria-hidden="true"><g fill="none" stroke="currentColor" stroke-width="1"><path opacity=".2" d="M20 150 200 55 380 150 200 245ZM40 150 200 65 360 150 200 235ZM70 150 200 80 330 150 200 220"/><path fill="#0c2c40" d="m90 110 110-58 110 58-110 58z"/><path fill="#071c2c" d="m90 110 110 58v55L90 165zM200 168l110-58v55l-110 58z"/><path d="M110 130v36m22-25v36m22-24v36m22-25v36m47-35v36m23-48v36m23-48v36m23-48v36"/><path d="m150 95 50-26 50 26-50 26zM150 95v25l50 26 50-26V95M200 121v25"/><path opacity=".3" d="M30 128 200 38 370 128M30 180l170 90 170-90"/></g></svg>';
function context(){return el('aside','panel context-panel','<p class="eyebrow">A connected library community</p>'+artifact+'<div><p class="target">CA$250 million</p><p class="muted">Proposed corporate donation target · Not secured</p></div><div class="split-stats"><div><strong>50%</strong><span>Net interest to libraries</span></div><div><strong>50%</strong><span>Net interest reinvested</span></div></div><p class="muted">After investment and administration costs.</p><div class="context-links"><a href="#atlas">Library atlas →</a><a href="#spending">Funding model →</a></div><p class="muted">Proposed trust · Structure unresolved · No receipts promised</p>');}
function heading(s,title,sub,label='PROPOSED / SASKATCHEWAN'){s.append(el('div','screen-heading','<p class="eyebrow">'+label+'</p><h1>'+title+'</h1>'+(sub?'<p class="screen-subtitle">'+sub+'</p>':'')));}
function narrative(id,title,sub,entries){const s=section(id);heading(s,title,sub);const work=el('div','workspace'),content=el('div','panel content-panel');work.append(context(),content);s.append(work);return tabSet(content,entries,title+' details');}
const purpose=source.purpose.slice(1);
narrative('purpose','A lasting fund. Stronger library communities.','Corporate donations. Responsible investment. Transparent community support.',[
 ['Purpose','<p class="eyebrow">The purpose</p><h3>Help libraries plan beyond the next budget.</h3><p>'+lead+'</p>'+purpose.map(e=>e.outerHTML).join('')+'<p>Library authorities retain ownership, governance and decisions about services.</p>'],
 ['How it works',modelQuestions.slice(1).map(([q,a])=>'<div><h3 style="font-size:19px">'+q+'</h3><p>'+a+'</p></div>').join('')],
 ['Service outcomes',outcomes.cloneNode(true)],['Open decisions','<h3>A working proposal.</h3><p>'+qualification+'</p><p>Confirm actual library needs, eligible purposes, donor restrictions, professional investment arrangements and independent oversight before any future launch.</p><a href="#evidence">Explore the evidence register →</a>']]);
const gifts=source.donations.find(e=>e.classList.contains('grid'));const giftArticles=[...gifts.children];
narrative('donations','A gift for a shared purpose.','Corporate gifts build capacity while respecting each gift’s terms.',[
 ['Long-term capital',[source.donations[1],...giftArticles[0].children]],['Immediate needs',[...giftArticles[1].children]],['Ownership & receipts',source.donations.at(-1)]]);
narrative('fairness','Local needs. Published decisions.','Eligibility, evidence and donor-compatible purposes.',[
 ['Allocation process',[source.fairness[1],source.fairness[2]]],['Service outcomes',outcomes]]);
function recordPicker(records,label){const box=el('div');const id='record-'+(++sequence);const l=el('label');l.htmlFor=id;l.textContent=label;const select=el('select','record-picker');select.id=id;const body=el('div','record-body');
 records.forEach((r,i)=>{const option=el('option');option.value=i;option.textContent=(r.querySelector('summary,h3')||r).textContent;select.append(option);});
 function update(){const r=records[Number(select.value)].cloneNode(true);if(r.tagName==='DETAILS'){const title=el('h3');title.textContent=r.querySelector('summary').textContent;r.querySelector('summary').remove();body.replaceChildren(title,...r.children);}else{r.classList.remove('panel');body.replaceChildren(r);}}
 select.addEventListener('change',update);box.append(l,select,body);update();return box;}
narrative('oversight','Stewardship with clear responsibilities.','Proposed roles, independent review and public reporting.',[
 ['Responsibilities',recordPicker(source.oversight.filter(e=>e.tagName==='DETAILS'),'Review a proposed role')],
 ['Donor protections','<h3>Keep each gift’s purpose visible.</h3><p>Track long-term and immediate-use funds separately, with additional donor restrictions where required. Check permitted costs and grants before authorization.</p><p>Trustees oversee donated assets; they do not personally own them. Library authorities retain ownership, governance and service decisions.</p>'],
 ['Conflicts & reporting','<h3>Independent judgement. Visible results.</h3><p>Declare interests, recuse conflicted decision-makers, document related-party decisions and provide an independent review or appeal process.</p><p>Publish financial reports, investment performance, costs, allocations and practical outcomes. Arrange independent oversight and report action on findings.</p>']]);
narrative('atlas','Libraries at the centre.','Illustrative relationships. No participation is claimed.',[['Connected roles',recordPicker(source.atlas.filter(e=>e.tagName==='DETAILS'),'Explore a library ecosystem role')],['Connections',source.atlas[1]]]);
narrative('evidence','Make the open decisions visible.','Context is separate from verified needs, commitments and approvals.',[['Evidence register',recordPicker(source.evidence.filter(e=>e.tagName==='ARTICLE'),'Review an evidence record')]]);
const stageEntries=[];for(let i=1;i<source.readiness.length;i+=2)stageEntries.push([['Listen','Structure','Policies','Allocation','Future launch'][(i-1)/2],[source.readiness[i],source.readiness[i+1],el('p','muted','Proposed / not completed. Each stage requires evidence and a separate decision.')]]);
narrative('readiness','Evidence before a future launch.','Proposed decision gates, not completed steps.',stageEntries);

// The funding page mirrors the requested two-panel, three-tab presentation.
const funding=section('spending'),scenario=section('scenario');
function fundingHeading(s){heading(s,'Build the fund. Sustain library access.','','SASKATCHEWAN / LIBRARY FUNDING');s.append(el('div','notice','<strong>Proposed model.</strong> CA$250 million in corporate donations would establish the principal fund. 50% of positive net interest would support libraries; 50% would be reinvested. The target is not secured capital.'));}
function fundingTabs(s,active){const nav=el('div','tab-list');nav.setAttribute('aria-label','Funding views');[['How it works','spending'],['Scenario calculator','scenario'],['Funding & accountability','accountability']].forEach(([t,id],i)=>{const a=el('a');a.href='#'+id;a.textContent='0'+(i+1)+'  '+t;a.style.cssText='padding:10px 0;text-decoration:none;border-bottom:2px solid '+(id===active?'#3dcaff':'transparent');if(id===active)a.setAttribute('aria-current','page');nav.append(a);});s.append(nav);}
fundingHeading(funding);fundingTabs(funding,'spending');
const fundingCols=el('div','funding-columns');fundingCols.append(el('div','panel','<p class="eyebrow">Proposed allocation of net cash interest</p><div class="ring-area"><div class="ring" role="img" aria-label="50 percent of net interest for libraries and 50 percent reinvested"><div class="ring-center"><span>NET INTEREST</span><strong>50 / 50</strong><span>after costs</span></div></div><div class="funding-key"><div>Corporate donation target<strong>CA$250M</strong><small>Principal to raise · Not secured</small></div><div>Library support<strong>50%</strong><small>Of net cash interest</small></div><div>Reinvestment<strong>50%</strong><small>Of net cash interest</small></div></div></div><p class="eyebrow">Donations → Principal fund → Interest → Library support</p><p class="muted">Net interest is cash interest after investment and administration costs. Grants respect reserves, donor restrictions and eligible need. Unused grant capacity stays in the fund.</p>'));
const serviceOutcomes=outcomes.cloneNode(true);const briefs=['Evening and weekend access','Reliable public service teams','Books and accessible formats','Computers and connectivity','Fewer barriers to library use','Community learning and culture'];serviceOutcomes.querySelectorAll('.panel p').forEach((p,i)=>p.textContent=briefs[i]);
const services=el('div','panel');services.append(el('p','eyebrow','What library support could make possible'),el('h3','','The places we learn.<br>The services we share.'),serviceOutcomes,el('p','muted','Illustrative uses, not verified needs or promised outcomes. Rates and purchasing power vary. Neither permanent funding nor capital preservation is guaranteed.'));fundingCols.append(services);funding.append(fundingCols);
const accountability=el('section','screen');accountability.id='accountability';accountability.hidden=true;main.append(accountability);fundingHeading(accountability);fundingTabs(accountability,'accountability');
const accountCols=el('div','funding-columns');accountCols.append(el('div','panel','<p class="eyebrow">Gift purpose & spending</p><h3>Protect the purpose of each gift.</h3>'+source.spending.slice(1).map(e=>e.outerHTML).join('')+'<p class="muted">Immediate-use gifts remain separately accounted for. Unused funds retain their original restrictions. No shares or investment returns go to donors.</p>'),el('div','panel','<p class="eyebrow">Proposed oversight</p><h3>Accountable to the stated purpose.</h3><p>Trustees oversee policy, risk and assets. Professional managers work within an approved mandate. Library authorities keep service decisions.</p><p>Publish eligibility, allocation decisions, conflicts controls, performance, costs and financial reports. Arrange independent scrutiny.</p><p class="muted">'+qualification+'</p><a href="#oversight">Review proposed responsibilities →</a>'));accountability.append(accountCols);

// Preserve the original form and IDs: the existing Rust/WASM adapter stays shared.
fundingHeading(scenario);fundingTabs(scenario,'scenario');const sc=el('div','scenario-content');scenario.append(sc);
const help=source.scenario.find(e=>e.id==='scenario-help');help.className='sr-only';sc.append(help);
const preset=source.scenario.find(e=>e.id==='preset'),presetLabel=source.scenario.find(e=>e.tagName==='LABEL');
const toolbar=el('div','scenario-toolbar');toolbar.append(presetLabel,preset);sc.append(toolbar);
const form=source.scenario.find(e=>e.tagName==='FORM');form.classList.add('panel');const inputFields=[...form.querySelectorAll('fieldset')],inputGrid=form.querySelector('.grid');
// Keep inflation with need and horizon so returns and costs remain a short panel.
for(const n of ['inflation']){const input=form.elements.namedItem(n),label=form.querySelector('label[for="'+n+'"]');inputFields.at(-1).prepend(label,input);}
const inputList=el('div','tab-list');inputList.setAttribute('role','tablist');inputList.setAttribute('aria-label','Scenario input groups');form.prepend(inputList);
const inputNames=['Donations','Returns & costs','Policy','Horizon'],inputPages=[];
function showInput(i,focus=false){inputFields.forEach((f,j)=>{f.hidden=j!==i;inputPages[j].hidden=j!==i;const b=inputList.children[j];b.setAttribute('aria-selected',String(j===i));b.tabIndex=j===i?0:-1;});if(focus)inputList.children[i].focus();}
inputFields.forEach((f,i)=>{const b=el('button');b.type='button';b.textContent=inputNames[i];b.id='input-tab-'+i;const page=el('div','input-page');page.id='input-panel-'+i;page.setAttribute('role','tabpanel');page.setAttribute('aria-labelledby',b.id);f.before(page);page.append(f);inputPages.push(page);b.setAttribute('role','tab');b.setAttribute('aria-controls',page.id);inputList.append(b);b.addEventListener('click',()=>showInput(i));b.addEventListener('keydown',e=>{let n=i;if(e.key==='ArrowRight')n=(i+1)%4;else if(e.key==='ArrowLeft')n=(i+3)%4;else if(e.key==='Home')n=0;else if(e.key==='End')n=3;else return;e.preventDefault();showInput(n,true);});});showInput(0);
form.addEventListener('invalid',e=>{const i=inputFields.indexOf(e.target.closest('fieldset'));if(i>=0)showInput(i);},true);
function comparison(){const visible=form.elements.namedItem('interest_only').value==='false';form.elements.namedItem('spending_rate').hidden=!visible;form.querySelector('label[for="spending_rate"]').hidden=!visible;}form.addEventListener('change',comparison);form.addEventListener('input',comparison);comparison();
const results=source.scenario.find(e=>e.id==='results'),resultPanel=el('div','panel content-panel');
const metric=results.querySelector('#metrics'),table=results.querySelector('.table-wrap'),detail=results.querySelector('details'),dataAction=results.querySelector('.actions'),resultNote=results.querySelector('p:not([id])');
resultNote.textContent='Hypothetical. Cumulative support is nominal. No capital or funding guarantee. Immediate-use gifts remain separate.';
toolbar.append(dataAction);
const rules=source.scenario.filter(e=>e.tagName==='DETAILS');
const resultTabs=tabSet(results,[['At a glance',[metric,resultNote]],['Annual ledger',table],['Policy ledger',detail.querySelector('.table-wrap')],['Calculation rules',recordPicker(rules,'Review a calculation rule')]],'Scenario result views');detail.remove();
resultPanel.append(results);for(const id of ['engine-status','error'])resultPanel.append(source.scenario.find(e=>e.id===id));
const scenarioCols=el('div','scenario-columns');scenarioCols.append(form,resultPanel);sc.append(scenarioCols);
// Submission changes results in place; no navigation or page-height growth.
form.addEventListener('submit',()=>{comparison();resultTabs.choose(0);});
section('reset').addEventListener('click',()=>{queueMicrotask(comparison);});preset.addEventListener('change',()=>queueMicrotask(comparison));

// Main navigation switches one complete screen at a time.
for(const [id,title]of [['atlas','Atlas'],['readiness','Readiness']]){const a=el('a');a.href='#'+id;a.textContent=title;globalNav.insertBefore(a,globalNav.lastElementChild);}
globalNav.querySelector('a[href="#spending"]').textContent='Funding';globalNav.querySelector('a[href="#scenario"]').remove();
const screens=[...main.querySelectorAll(':scope>.screen')];
function show(){let id=location.hash.slice(1)||'purpose';if(!screens.some(s=>s.id===id))id='purpose';screens.forEach(s=>s.hidden=s.id!==id);for(const a of globalNav.querySelectorAll('a')){const selected=a.hash==='#'+id||(a.hash==='#spending'&&['scenario','accountability'].includes(id));if(selected)a.setAttribute('aria-current','page');else a.removeAttribute('aria-current');}document.title=(section(id).querySelector('h1')?.textContent||'Library Trust')+' — Saskatchewan Library Sustainability Trust';main.scrollTop=0;}
window.addEventListener('hashchange',show);document.body.classList.add('review-app');show();
document.querySelector('footer').innerHTML='<span>PROPOSED LIBRARY SUSTAINABILITY TRUST · NO DONATIONS COLLECTED</span><span><a href="review.txt">Download complete review</a> · <a href="index.html#/overview">Visual demo</a></span>';
})();
