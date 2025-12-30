import { useState } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { Upload, File } from 'lucide-react';
import { useFileTransfer } from '../hooks/useFileTransfer';
import { useNotifications } from './NotificationProvider';

interface FileUploaderProps {
  connectionId: string;
  currentPath: string;
  onUploadComplete?: () => void;
}

export function FileUploader({ connectionId, currentPath, onUploadComplete }: FileUploaderProps) {
  const { uploadFile, transfers } = useFileTransfer(connectionId);
  const { showSuccess, showError } = useNotifications();
  const [uploading, setUploading] = useState(false);

  const handleFileSelect = async () => {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
      });

      if (!selected) return;

      setUploading(true);

      const localPath = selected as string;
      const fileName = localPath.split(/[/\\]/).pop() || 'file';
      const remotePath = currentPath.endsWith('/')
        ? `${currentPath}${fileName}`
        : `${currentPath}/${fileName}`;

      await uploadFile(localPath, remotePath);

      showSuccess(`Successfully uploaded ${fileName}`);
      setUploading(false);

      if (onUploadComplete) {
        onUploadComplete();
      }
    } catch (err) {
      showError(String(err));
      setUploading(false);
    }
  };

  const activeTransfers = transfers.filter(t => t.type === 'upload');

  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
      <button
        onClick={handleFileSelect}
        disabled={uploading}
        style={{
          display: 'flex',
          alignItems: 'center',
          gap: '8px',
          padding: '10px 16px',
          background: uploading ? '#333' : '#4a9eff',
          color: '#fff',
          border: 'none',
          borderRadius: '4px',
          cursor: uploading ? 'not-allowed' : 'pointer',
          fontSize: '14px',
          fontWeight: 'bold',
          opacity: uploading ? 0.6 : 1,
        }}
        onMouseEnter={(e) => {
          if (!uploading) {
            e.currentTarget.style.background = '#3a8eef';
          }
        }}
        onMouseLeave={(e) => {
          if (!uploading) {
            e.currentTarget.style.background = '#4a9eff';
          }
        }}
      >
        <Upload size={18} />
        {uploading ? 'Uploading...' : 'Upload File'}
      </button>

      {activeTransfers.length > 0 && (
        <div style={{ display: 'flex', gap: '8px' }}>
          {activeTransfers.map((transfer) => {
            const progress = (transfer.transferred / transfer.total) * 100;

            return (
              <div
                key={transfer.transfer_id}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '8px',
                  padding: '8px 12px',
                  background: '#1a1a1a',
                  border: '1px solid #333',
                  borderRadius: '4px',
                  minWidth: '200px',
                }}
              >
                <File size={16} color="#4a9eff" />
                <div style={{ flex: 1 }}>
                  <div style={{ position: 'relative', height: '4px', background: '#333', borderRadius: '2px', overflow: 'hidden', marginBottom: '4px' }}>
                    <div
                      style={{
                        position: 'absolute',
                        top: 0,
                        left: 0,
                        height: '100%',
                        width: `${progress}%`,
                        background: '#4a9eff',
                        transition: 'width 0.3s ease',
                      }}
                    />
                  </div>
                  <div style={{ fontSize: '10px', color: '#888' }}>
                    {progress.toFixed(0)}%
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
