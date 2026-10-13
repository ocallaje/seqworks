// src/components/Status/StatusBar.tsx
import { useContext } from 'react';
import { StatusContext } from '../../store/StatusContext';
import {
  CheckCircleIcon,
  ExclamationCircleIcon,
  RefreshIcon,
} from '@heroicons/react/solid';

export default function StatusBar() {
  const { wsStatus, pipelineStatus, cellxgeneStatus } = useContext(StatusContext);

  return (
    <footer className="p-2 bg-surface-dark">
      <div className="flex items-center space-x-6">
        {/* WebSocket */}
        <div className="flex items-center">
          <RefreshIcon className="h-4 w-4 animate-spin text-primary" />
          <span className="ml-1 text-sm">{wsStatus ?? 'Connecting …'}</span>
        </div>

        {/* Pipeline */}
        <div className="flex items-center">
          {pipelineStatus === 'finished' ? (
            <CheckCircleIcon className="h-4 w-4 text-success" />
          ) : pipelineStatus === 'error' ? (
            <ExclamationCircleIcon className="h-4 w-4 text-error" />
          ) : (
            <RefreshIcon className="h-4 w-4 animate-spin text-warning" />
          )}
          <span className="ml-1 text-sm">{pipelineStatus ?? 'Idle'}</span>
        </div>

        {/* CellxGene */}
        <div className="flex items-center">
          {cellxgeneStatus === 'running' ? (
            <RefreshIcon className="h-4 w-4 animate-spin text-success" />
          ) : (
            <ExclamationCircleIcon className="h-4 w-4 text-error" />
          )}
          <span className="ml-1 text-sm">{cellxgeneStatus ?? 'Closed'}</span>
        </div>
      </div>
    </footer>
  );
}