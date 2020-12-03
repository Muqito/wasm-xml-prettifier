onmessage = (e) => {
    (async () => {
        const wasm = await import('prettifier');
        try {
            console.time('prettier');
            const result = wasm.prettify_xml(e.data);
            console.timeEnd('prettier');
            postMessage(result);
        } catch (e) {
            console.error('caught error', e);
            postMessage(e.data);
        }
    })();
};