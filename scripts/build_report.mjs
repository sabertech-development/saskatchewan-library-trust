import {createRequire} from 'node:module';
import {fileURLToPath} from 'node:url';
import {readFile,writeFile} from 'node:fs/promises';
const require=createRequire(import.meta.url);
let esbuild;try{esbuild=require('esbuild');}catch{esbuild=require('../target/report-tools/node_modules/esbuild');}
await esbuild.build({entryPoints:['scripts/report_document.mjs'],bundle:true,format:'esm',platform:'browser',target:'es2020',minify:true,legalComments:'linked',outfile:'assets/report-runtime.mjs',nodePaths:['target/report-tools/node_modules']});
let license='Report runtime dependencies, bundled locally.\n\n';
for(const pkg of ['pdf-lib','@pdf-lib/fontkit']){
 let p;try{p=require.resolve(pkg+'/package.json');}catch{p=fileURLToPath(new URL('../target/report-tools/node_modules/'+pkg+'/package.json',import.meta.url));}
 const base=p.slice(0,p.lastIndexOf('package.json'));
 for(const filename of ['LICENSE.md','LICENSE']){try{license+=pkg+'\n'+await readFile(base+filename,'utf8')+'\n\n';break;}catch{}}
}
await writeFile('assets/report-licenses.txt',license);
