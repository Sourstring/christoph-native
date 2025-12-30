// src/hooks/useFileTransfer.ts
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface TransferProgress {
  transfer_id: string;
  transferred: number;
  total: number;
  type: 'upload' | 'download';
}

export function useFileTransfer(connectionId: string) {
  const [transfers, setTransfers] = useState<Map<string, TransferProgress>>(new Map());

  useEffect(() => {
    const unlisten = Promise.all([
      listen('upload_progress', (event: any) => {
        setTransfers(prev => new Map(prev).set(event.payload.transfer_id, event.payload));
      }),
      listen('download_progress', (event: any) => {
        setTransfers(prev => new Map(prev).set(event.payload.transfer_id, event.payload));
      }),
      listen('process_finished', (event: any) => {
        setTransfers(prev => {
          const next = new Map(prev);
          next.delete(event.payload.transfer_id);
          return next;
        });
      }),
    ]);

    return () => {
      unlisten.then(listeners => listeners.forEach(fn => fn()));
    };
  }, []);

  const uploadFile = async (localPath: string, remotePath: string) => {
    return await invoke<string>('upload_file', { connectionId, localPath, remotePath });
  };

  const downloadFile = async (remotePath: string, localPath: string) => {
    return await invoke<string>('download_file', { connectionId, remotePath, localPath });
  };

  return { transfers: Array.from(transfers.values()), uploadFile, downloadFile };
}
