import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';

function SeedRecovery() {
  const [words, setWords] = useState(Array(24).fill(''));
  const [status, setStatus] = useState('');

  const handleChange = (idx: number, value: string) => {
    const newWords = [...words];
    newWords[idx] = value;
    setWords(newWords);
  };

  const recover = async () => {
    const seed = words.join(' ');
    try {
      await invoke('recover_from_seed', { seed });
      setStatus('✅ Восстановлено');
    } catch (e) {
      setStatus('❌ Ошибка');
    }
  };

  return (
    <div className="p-6">
      <h2>Восстановление по seed-фразе</h2>
      <p>Введите 24 слова:</p>
      <div className="grid grid-cols-3 gap-2 mt-4">
        {words.map((word, i) => (
          <input
            key={i}
            type="text"
            value={word}
            onChange={(e) => handleChange(i, e.target.value)}
            className="border p-1"
            placeholder={`Слово ${i+1}`}
          />
        ))}
      </div>
      <button onClick={recover} className="mt-4 px-4 py-2 bg-blue-500 text-white">
        Восстановить
      </button>
      <p>{status}</p>
    </div>
  );
}