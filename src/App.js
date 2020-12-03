import React, { useState, useEffect } from 'react';
import WebWorker from './workers/web.worker.js';
import style from './App.module.css';

function App() {
  const worker = React.useRef(new WebWorker()).current;
  const [input, setInput] = useState('');
  const [output, setOutput] = useState('');

  const onChangeTextarea = e => setInput(e.target.value);

  useEffect(() => {
    worker.postMessage(input);
  }, [worker, input]);
  useEffect(() => {
    const onMessage = (e) => setOutput(e.data);
    worker.addEventListener('message', onMessage);
    return () => worker.removeEventListener('message', onMessage);
  }, [worker]);

  return (
      <div className={style.page}>
        <h2>XML formatter</h2>
        <p>This is an XML formatter written in Rust, compiled to WebAssembly and posted via a WebWorker</p>
        <div className={style.formatter}>
          <textarea onChange={onChangeTextarea} placeholder="Unformatted XML" />
          <textarea disabled value={output} placeholder="Formatted XML" />
        </div>
      </div>
  );
}

export default App;