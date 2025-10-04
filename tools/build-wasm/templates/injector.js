const originalFetch = window.fetch;

window.fetch = function(...args) {
  const url = args[0];
  
  if (url === 'game.wasm') {
    const binaryData = 'INSERT_WASM_HERE';
    
    const binaryString = atob(binaryData);
    const bytes = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
    }
    
    const response = new Response(bytes.buffer, {
      status: 200,
      statusText: 'OK',
      headers: new Headers({
        'Content-Type': 'application/wasm'
      })
    });
    
    return Promise.resolve(response);
  }
  
  return originalFetch.apply(this, args);
};

load("game.wasm");
