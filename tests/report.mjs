// Generate actual PDFs from the same WebAssembly payload used by download buttons.
import assert from 'node:assert/strict';
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {PDFDocument} from 'pdf-lib';
import {createReviewPdf} from '../assets/report-runtime.mjs';
import init,{scenario_inputs,report_scenario,project_scenario} from '../dist/pkg/saskatchewan_library_funding.js';
await init({module_or_path:await readFile('dist/pkg/saskatchewan_library_funding_bg.wasm')});
const resources={template:await readFile('assets/report-template.pdf'),regular:await readFile('assets/fonts/SpaceGrotesk.ttf'),medium:await readFile('assets/fonts/SpaceGrotesk-Medium.ttf'),createdAt:'2026-11-18T17:00:00Z'};
const defaults=JSON.parse(scenario_inputs(0));
await mkdir('tmp/pdfs/tests',{recursive:true});
for(const [name,changes]of [
 ['selected',{capital:12345678,annual_capital_donations:50000,annual_immediate_donations:200000,years:50,library_interest_share:0.4,first_year_return:-0.2}],
 ['empty',{capital:0,annual_capital_donations:0,annual_immediate_donations:0,administration:0,library_need:0,years:1}],
 ['capital-comparison',{interest_only:false,protect_principal:false,interest_rate:0,return_rate:-0.08,years:20}],
 ['comparison-limit',{capital:1e12,interest_rate:0,return_rate:1,first_year_return:-1,administration:0,years:50}],
 ['large',{capital:1e12,interest_rate:1,return_rate:1,years:12,annual_immediate_donations:1e12,library_need:1e12}]
]){
 const i={...defaults,...changes},data=JSON.parse(report_scenario(JSON.stringify(i)));
 assert.deepEqual(data.projection,JSON.parse(project_scenario(JSON.stringify(i))));
 const bytes=await createReviewPdf(data,resources),doc=await PDFDocument.load(bytes);
 assert.equal(doc.getPageCount(),8+Math.ceil(i.years/24));assert.equal(doc.getAuthor(),'Saber Tech Inc.');
 assert.match(doc.getTitle(),/Saskatchewan Library Sustainability Trust/);
 await writeFile('tmp/pdfs/tests/'+name+'.pdf',bytes);
 await writeFile('tmp/pdfs/tests/'+name+'.json',JSON.stringify(data));
}
assert.throws(()=>report_scenario(JSON.stringify({...defaults,capital:1e12,interest_rate:1,return_rate:1,years:50})));
console.log('PDF generation passed: custom 50-year selection, zero balances, capital-spending comparison, maximum input/horizon, author and pagination.');

// Exercise the browser download adapter with a DOM and local resource loader.
const {JSDOM}=await import('../target/semantic-tests/node_modules/jsdom/lib/api.js');
const dom=new JSDOM('<html><body></body></html>');
globalThis.window=dom.window;globalThis.document=dom.window.document;
const originalFetch=globalThis.fetch;let downloaded=null;
globalThis.fetch=async url=>new Response(await readFile(url));
dom.window.HTMLAnchorElement.prototype.click=function(){downloaded=originalFetch(this.href).then(r=>r.arrayBuffer());};
await import('../assets/report-download.js');
assert.equal(await window.downloadTrustReport(report_scenario(JSON.stringify(defaults))),true);
const downloadedPdf=await PDFDocument.load(await downloaded);assert.equal(downloadedPdf.getPageCount(),9);
assert.match(document.getElementById('pdf-download-status').textContent,/ready/);
assert.equal(await window.downloadTrustReport('{}'),false);
assert.match(document.getElementById('pdf-download-status').textContent,/failed/);
console.log('Browser download adapter: actual PDF Blob, local resources, status and error handling passed.');
process.exit(0);
