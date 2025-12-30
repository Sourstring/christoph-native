import { useState } from 'react';
import { ConnectionForm } from './components/ConnectionForm';
import { FileBrowser } from './components/FileBrowser';
import { NotificationProvider } from './components/NotificationProvider';

function App() {
  const [connectionId, setConnectionId] = useState<string | null>(null);

  return (
    <NotificationProvider>
      <div style={{
        height: '100vh',
        display: 'flex',
        flexDirection: 'column',
        background: '#0a0a0a',
        color: '#f6f6f6',
      }}>
        <header style={{
          padding: '16px 24px',
          borderBottom: '2px solid #333',
          background: '#0f0f0f',
        }}>
          <h1 style={{ margin: 0, fontSize: '24px' }}>SFTP Explorer</h1>
        </header>

        <main style={{ flex: 1, overflow: 'hidden', padding: connectionId ? '0' : '40px 20px' }}>
          {!connectionId ? (
            <ConnectionForm onConnect={setConnectionId} />
          ) : (
            <FileBrowser connectionId={connectionId} />
          )}
        </main>
      </div>
    </NotificationProvider>
  );
}

export default App;
