import { createContext, useContext, useState, useCallback, ReactNode } from 'react';
import { CheckCircle, AlertCircle, X } from 'lucide-react';

interface Notification {
  id: string;
  type: 'success' | 'error';
  message: string;
}

interface NotificationContextType {
  showSuccess: (message: string) => void;
  showError: (message: string) => void;
}

const NotificationContext = createContext<NotificationContextType | undefined>(undefined);

export function useNotifications() {
  const context = useContext(NotificationContext);
  if (!context) {
    throw new Error('useNotifications must be used within NotificationProvider');
  }
  return context;
}

export function NotificationProvider({ children }: { children: ReactNode }) {
  const [notifications, setNotifications] = useState<Notification[]>([]);

  const showSuccess = useCallback((message: string) => {
    const id = Date.now().toString();
    setNotifications(prev => [...prev, { id, type: 'success', message }]);
    setTimeout(() => {
      setNotifications(prev => prev.filter(n => n.id !== id));
    }, 30000);
  }, []);

  const showError = useCallback((message: string) => {
    const id = Date.now().toString();
    setNotifications(prev => [...prev, { id, type: 'error', message }]);
    setTimeout(() => {
      setNotifications(prev => prev.filter(n => n.id !== id));
    }, 60000);
  }, []);

  const removeNotification = (id: string) => {
    setNotifications(prev => prev.filter(n => n.id !== id));
  };

  return (
    <NotificationContext.Provider value={{ showSuccess, showError }}>
      {children}
      <div
        style={{
          position: 'fixed',
          top: '16px',
          right: '16px',
          zIndex: 9999,
          display: 'flex',
          flexDirection: 'column',
          gap: '8px',
          maxWidth: '400px',
        }}
      >
        {notifications.map(notification => (
          <div
            key={notification.id}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '12px',
              padding: '12px 16px',
              background: notification.type === 'success' ? '#00ff0020' : '#ff000020',
              border: `1px solid ${notification.type === 'success' ? '#00ff00' : '#ff0000'}`,
              color: notification.type === 'success' ? '#66ff66' : '#ff6666',
              borderRadius: '4px',
              fontSize: '13px',
              boxShadow: '0 4px 12px rgba(0, 0, 0, 0.3)',
              animation: 'slideIn 0.3s ease-out',
            }}
          >
            {notification.type === 'success' ? (
              <CheckCircle size={18} />
            ) : (
              <AlertCircle size={18} />
            )}
            <span style={{ flex: 1 }}>{notification.message}</span>
            <X
              size={16}
              style={{ cursor: 'pointer', flexShrink: 0 }}
              onClick={() => removeNotification(notification.id)}
            />
          </div>
        ))}
      </div>
      <style>{`
        @keyframes slideIn {
          from {
            transform: translateX(100%);
            opacity: 0;
          }
          to {
            transform: translateX(0);
            opacity: 1;
          }
        }
      `}</style>
    </NotificationContext.Provider>
  );
}
