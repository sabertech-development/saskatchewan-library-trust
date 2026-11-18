// Keep useful content visible until both the engine and renderer are ready.
const canvas=document.getElementById('proposal'),loading=document.getElementById('loading');
const status=document.getElementById('loading-status');
let complete=false;
function fail(error){
  if(complete)return;
  console.error('Presentation startup:',error);
  status.textContent='Opening the accessible proposal...';
  // The HTML view renders its narrative without WebAssembly or WebGL.
  location.replace(new URL('reader.html',location.href));
}
const slow=setTimeout(()=>{status.textContent='Still loading. You can open the accessible proposal below.';},8000);
const timeout=setTimeout(()=>fail(Error('Visual startup timed out')),25000);
try{
  const {default:init,start_visual}=await import('./pkg/saskatchewan_library_funding.js');
  status.textContent='Loading the presentation...';
  await init();
  status.textContent='Preparing the library atlas...';
  await start_visual();
  await new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve)));
  complete=true;canvas.style.opacity='1';loading.remove();
}catch(error){fail(error);}
finally{clearTimeout(slow);clearTimeout(timeout);}
