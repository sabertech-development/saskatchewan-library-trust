import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {JSDOM} from '../target/semantic-tests/node_modules/jsdom/lib/api.js';
const source=(await readFile('assets/site-loader.js','utf8')).replace("await import('./pkg/saskatchewan_library_funding.js')",'await loadModule()');
const AsyncFunction=Object.getPrototypeOf(async function(){}).constructor;
for(const mode of ['success','module-error','renderer-error','slow']){
 const dom=new JSDOM(await readFile('dist/index.html','utf8'));const document=dom.window.document;
 assert.ok(document.getElementById('loading').textContent.includes('Saskatchewan Library Sustainability Trust'));
 const timers=new Map();let redirected=null,rendered=false;
 const location={href:'https://sabertech-development.github.io/saskatchewan-library-trust/',replace:url=>redirected=String(url)};
 const loadModule=async()=>{if(mode==='module-error')throw Error('Module unavailable');return {default:async()=>{},start_visual:async()=>{if(mode==='renderer-error')throw Error('Renderer unavailable');if(mode==='slow'){timers.get(8000)();assert.match(document.getElementById('loading-status').textContent,/Still loading/);timers.get(25000)();}rendered=true;}};};
 await new AsyncFunction('document','location','requestAnimationFrame','setTimeout','clearTimeout','console','loadModule',source)(document,location,cb=>cb(),(cb,ms)=>{timers.set(ms,cb);return ms;},ms=>timers.delete(ms),{error:()=>{}},loadModule);
 if(mode==='success'){assert.equal(redirected,null);assert.equal(document.getElementById('loading'),null);assert.equal(document.getElementById('proposal').style.opacity,'1');assert.ok(rendered);}
 else assert.equal(redirected,'https://sabertech-development.github.io/saskatchewan-library-trust/reader.html');
}
console.log('Startup loader: visible initial content, awaited renderer, module failure, graphics failure and timeout fallback passed.');
