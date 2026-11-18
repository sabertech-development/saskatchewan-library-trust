// DOM integration checks, not a browser or assistive-technology conformance test.
// Install test-only dependencies with: npm install --prefix target/semantic-tests jsdom
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {JSDOM} from '../target/semantic-tests/node_modules/jsdom/lib/api.js';
import init, {project_scenario, scenario_inputs} from '../dist/pkg/saskatchewan_library_funding.js';
const bytes=await readFile(new URL('../dist/pkg/saskatchewan_library_funding_bg.wasm',import.meta.url));
await init({module_or_path:bytes});
const html=await readFile(new URL('../dist/reader.html',import.meta.url),'utf8');
const script=(await readFile(new URL('../dist/reader.js',import.meta.url),'utf8')).replace(/^\s*import .*?;\s*/,'');
const dom=new JSDOM(html,{url:'http://127.0.0.1:4387/reader.html',runScripts:'outside-only'});
const {document,FormData,Event}=dom.window;
const layout=await readFile(new URL('../dist/reader-layout.js',import.meta.url),'utf8');
dom.window.eval(layout);
const style=document.createElement('style');style.textContent=await readFile(new URL('../dist/reader-layout.css',import.meta.url),'utf8');document.head.append(style);assert.ok(style.sheet.cssRules.length>30);
assert.ok(document.body.classList.contains('review-app'));
assert.equal(document.querySelectorAll('.screen:not([hidden])').length,1);
const purposeTabs=document.querySelector('#purpose [role="tablist"]');
purposeTabs.children[1].click();
assert.equal(purposeTabs.children[1].getAttribute('aria-selected'),'true');
purposeTabs.children[1].dispatchEvent(new dom.window.KeyboardEvent('keydown',{key:'ArrowRight',bubbles:true}));
assert.equal(purposeTabs.children[2].getAttribute('aria-selected'),'true');
dom.window.location.hash='#scenario';dom.window.dispatchEvent(new dom.window.HashChangeEvent('hashchange'));
assert.equal(document.getElementById('scenario').hidden,false);
assert.equal(document.querySelectorAll('#scenario-form fieldset:not([hidden])').length,1);
const inputTabs=document.querySelector('#scenario-form [role="tablist"]');
inputTabs.children[2].click();
assert.equal(document.getElementById('input-panel-2').hidden,false);
inputTabs.children[0].click();
for(const id of ['purpose','donations','spending','accountability','fairness','oversight','atlas','evidence','readiness','scenario']){
  dom.window.location.hash='#'+id;dom.window.dispatchEvent(new dom.window.HashChangeEvent('hashchange'));
  assert.equal(document.querySelectorAll('.screen:not([hidden])').length,1);
  assert.equal(document.querySelector('.screen:not([hidden])').id,id);
  assert.ok(document.querySelector('.screen:not([hidden]) h1'));
}
const AsyncFunction=Object.getPrototypeOf(async function(){}).constructor;
await new AsyncFunction('document','FormData','init','project_scenario','scenario_inputs',script)(document,FormData,async()=>{},project_scenario,scenario_inputs);
assert.equal(document.getElementById('results').hidden,false);
assert.equal(document.querySelectorAll('#table-body tr').length,20);
assert.match(document.getElementById('result-status').textContent,/Proposed interest-sharing/);
assert.equal(document.getElementById('calculate').disabled,false);
for(const control of document.querySelectorAll('input,select')){assert.ok(document.querySelector('label[for="'+control.id+'"]'),'Unlabelled '+control.id);}
const form=document.getElementById('scenario-form');
function set(name,value){form.elements.namedItem(name).value=String(value);}
function calculate(){form.dispatchEvent(new Event('submit',{bubbles:true,cancelable:true}));assert.equal(document.getElementById('error').textContent,'');}
function firstCells(){return [...document.querySelector('#table-body tr').children].map(el=>el.textContent);}
set('capital',1000);set('annual_capital_donations',0);set('annual_immediate_donations',0);set('interest_rate',10);set('return_rate',0);set('first_year_return','');set('investment_cost_rate',1);set('administration',10);set('inflation',0);set('library_need',500);set('years',1);set('library_interest_share',50);calculate();
let cells=firstCells();assert.ok(cells.some(v=>v.includes('40')),'Expected net-interest share of 40');assert.ok(cells.some(v=>v.includes('1,040')),'Expected closing balance 1040');
set('annual_immediate_donations',600);calculate();cells=firstCells();assert.ok(cells.some(v=>v.includes('1,080')),'Immediate gifts must not be capitalized');assert.ok(cells.at(-1).includes('100'),'Unused immediate gifts stay separate');
document.getElementById('preset').value='3';document.getElementById('preset').dispatchEvent(new Event('change'));assert.match(document.getElementById('result-status').textContent,/Purchasing power/);assert.equal(document.querySelectorAll('#table-body tr').length,20);
document.getElementById('reset').click();assert.equal(form.elements.namedItem('capital').value,'250000000');assert.equal(form.elements.namedItem('library_interest_share').value,'50');
set('interest_only','false');set('protect_principal','false');calculate();assert.match(document.getElementById('result-status').textContent,/Capital-spending comparison only/);
console.log('Semantic form and real WebAssembly engine: initial calculation, labels, custom values, separate pools, stress preset, reset and comparison passed.');
