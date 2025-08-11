import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';

function App() {
  const [mode, setMode] = useState('google-meet');

  const connect = async () => {
    await invoke('connect', { mode });
  };

  return (
    <div className="p-8">
      <h1>Phantom Tunnel v5</h1>
      <select value={mode} onChange={e => setMode(e.target.value)}>
        <option value="google-meet">Google Meet</option>
        <option value="silent">Тихий режим</option>
        <option value="quic">QUIC+ECH</option>
      </select>
      <button onClick={connect} className="ml-4 px-4 py-2 bg-green-500 text-white">
        Connect
      </button>
    </div>
  );
}