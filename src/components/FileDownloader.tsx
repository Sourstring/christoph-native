import { useState } from 'react';
import { save } from '@tauri-apps/plugin-dialog';
import { Download } from 'lucide-react';
import { useFileTransfer } from '../hooks/useFileTransfer';
import { useNotifications } from './NotificationProvider';

interface FileDownloaderProps {
  connectionId: string;
  remotePath: string;
  fileName: string;
  onDownloadComplete?: () => void;
}

export function FileDownloader({ connectionId, remotePath, fileName, onDownloadComplete }: FileDownloaderProps) {
  const { downloadFile, transfers } = useFileTransfer(connectionId);
  const { showSuccess, showError } = useNotifications();
  const [downloading, setDownloading] = useState(false);

  const handleDownload = async () => {
    try {
      const savePath = await save({
        defaultPath: fileName,
      });

      if (!savePath) return;

      setDownloading(true);

      await downloadFile(remotePath, savePath);

      showSuccess(`Successfully downloaded ${fileName}`);
      setDownloading(false);

      if (onDownloadComplete) {
        onDownloadComplete();
      }
    } catch (err) {
      showError(String(err));
      setDownloading(false);
    }
  };

  const activeTransfers = transfers.filter(t => t.type === 'download' && t.transfer_id === remotePath);
  const hasActiveTransfer = activeTransfers.length > 0;

  return (
    <button
      onClick={handleDownload}
      disabled={downloading || hasActiveTransfer}
      style={{
        display: 'flex',
        alignItems: 'center',
        gap: '6px',
        padding: '4px 8px',
        background: downloading || hasActiveTransfer ? '#333' : 'transparent',
        color: downloading || hasActiveTransfer ? '#888' : '#4a9eff',
        border: '1px solid #4a9eff',
        borderRadius: '3px',
        cursor: downloading || hasActiveTransfer ? 'not-allowed' : 'pointer',
        fontSize: '12px',
        opacity: downloading || hasActiveTransfer ? 0.6 : 1,
      }}
      onMouseEnter={(e) => {
        if (!downloading && !hasActiveTransfer) {
          e.currentTarget.style.background = '#4a9eff20';
        }
      }}
      onMouseLeave={(e) => {
        if (!downloading && !hasActiveTransfer) {
          e.currentTarget.style.background = 'transparent';
        }
      }}
      title={`Download ${fileName}`}
    >
      <Download size={14} />
      {downloading || hasActiveTransfer ? 'Downloading...' : 'Download'}
    </button>
  );
}
