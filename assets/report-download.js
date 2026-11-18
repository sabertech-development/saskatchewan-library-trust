// All selected inputs and PDF rendering stay on the reader's device.
let resources;
async function loadResources(){
  const fetchBytes=async path=>{const r=await fetch(new URL(path,import.meta.url));if(!r.ok)throw Error('Could not load a report resource. Please reload and try again.');return new Uint8Array(await r.arrayBuffer());};
  const [template,regular,medium]=await Promise.all([fetchBytes('report-template.pdf'),fetchBytes('fonts/SpaceGrotesk.ttf'),fetchBytes('fonts/SpaceGrotesk-Medium.ttf')]);
  return {template,regular,medium};
}
function status(message,error=false){
  let el=document.getElementById('pdf-download-status');
  if(!el){el=document.createElement('p');el.id='pdf-download-status';el.setAttribute('role','status');el.setAttribute('aria-live','polite');Object.assign(el.style,{position:'fixed',bottom:'48px',right:'20px',maxWidth:'min(520px,85vw)',padding:'14px 20px',background:'#091622',color:'#e8f2fa',border:'1px solid #3dcaff',font:'16px sans-serif',zIndex:'1000'});document.body.append(el);}
  el.textContent=message;el.style.borderColor=error?'#f79c8e':'#3dcaff';
  clearTimeout(status.timer);if(!error)status.timer=setTimeout(()=>el.remove(),6500);
}
let active=false;
window.downloadTrustReport=async function(payload){
  if(active)return false;active=true;status('Preparing your review PDF with the selected scenario...');
  try{
    const data=typeof payload==='string'?JSON.parse(payload):payload;
    const {createReviewPdf}=await import('./report-runtime.mjs');
    resources??=loadResources().catch(error=>{resources=null;throw error;});
    const bytes=await createReviewPdf(data,await resources);
    const url=URL.createObjectURL(new Blob([bytes],{type:'application/pdf'}));
    const a=document.createElement('a');a.href=url;a.download='library-trust-review.pdf';document.body.append(a);a.click();a.remove();
    setTimeout(()=>URL.revokeObjectURL(url),30000);
    status('Your review PDF is ready. Check your browser downloads.');return true;
  }catch(error){status('PDF download failed: '+String(error.message||error),true);return false;}
  finally{active=false;}
};
