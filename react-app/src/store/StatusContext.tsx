// src/store/StatusContext.tsx
import { createContext, useState, PropsWithChildren } from 'react';

export const StatusContext = createContext({
  wsStatus: '',
  pipelineStatus: '',
  cellxgeneStatus: '',
  setRunStatus: (_: string) => {}
});

export function StatusProvider({ children }: PropsWithChildren<{}>) {
  const [wsStatus, setWsStatus] = useState('');
  const [pipelineStatus, setPipelineStatus] = useState('');
  const [cellxgeneStatus, setCellxgeneStatus] = useState('');

  const setRunStatus = (status: string) => setPipelineStatus(status);

  return (
    <StatusContext.Provider
      value={{ wsStatus, pipelineStatus, cellxgeneStatus, setRunStatus }}
    >
      {children}
    </StatusContext.Provider>
  );
}