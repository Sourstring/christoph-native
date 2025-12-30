import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Server, User, Lock, Hash } from 'lucide-react';

export function ConnectionForm({ onConnect }: { onConnect: (id: string) => void }) {
  const [host, setHost] = useState('');
  const [port, setPort] = useState('22');
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const handleConnect = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      const connectionId = await invoke<string>('connect_sftp', {
        host,
        port: parseInt(port),
        username,
        password: password || null,
        passphrase: null,
        privateKeyPath: null,
      });
      onConnect(connectionId);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ maxWidth: '400px', margin: '0 auto' }}>
      <form onSubmit={handleConnect} style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '8px', border: '1px solid #333', borderRadius: '4px' }}>
          <Server size={18} />
          <input
            value={host}
            onChange={(e) => setHost(e.target.value)}
            placeholder="Host (e.g., example.com)"
            required
            style={{ flex: 1, background: 'transparent', border: 'none', outline: 'none', color: 'inherit' }}
          />
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '8px', border: '1px solid #333', borderRadius: '4px' }}>
          <Hash size={18} />
          <input
            value={port}
            onChange={(e) => setPort(e.target.value)}
            placeholder="Port"
            style={{ flex: 1, background: 'transparent', border: 'none', outline: 'none', color: 'inherit' }}
          />
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '8px', border: '1px solid #333', borderRadius: '4px' }}>
          <User size={18} />
          <input
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            placeholder="Username"
            required
            style={{ flex: 1, background: 'transparent', border: 'none', outline: 'none', color: 'inherit' }}
          />
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '8px', border: '1px solid #333', borderRadius: '4px' }}>
          <Lock size={18} />
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="Password"
            style={{ flex: 1, background: 'transparent', border: 'none', outline: 'none', color: 'inherit' }}
          />
        </div>

        <button
          type="submit"
          disabled={loading}
          style={{
            padding: '12px',
            background: loading ? '#555' : '#0066cc',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            cursor: loading ? 'not-allowed' : 'pointer',
            fontWeight: 'bold',
          }}
        >
          {loading ? 'Connecting...' : 'Connect'}
        </button>

        {error && (
          <div style={{
            padding: '12px',
            background: '#ff000020',
            border: '1px solid #ff0000',
            borderRadius: '4px',
            color: '#ff6666'
          }}>
            {error}
          </div>
        )}
      </form>
    </div>
  );
}
