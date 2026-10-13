import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { ThemeProvider } from '../../store/ThemeContext';
import { SessionProvider } from '../../store/SessionContext';
import { StatusProvider } from '../../store/StatusContext';
import Sidebar from '../Sidebar/Sidebar';
import Topbar from '../Topbar/Topbar';
import StatusBar from '../Status/StatusBar';
import Login from '../Auth/Login';
import BulkRunner from '../Dashboard/BulkRunner';
import SCRunner from '../Dashboard/SCRunner';
import Cellxgene from '../Dashboard/Cellxgene';
import Home from '../Home';

export default function App() {
  return (
    <ThemeProvider>
      <SessionProvider>
        <StatusProvider>
          <Router>
            <div className="flex h-screen bg-bg-page dark:bg-bg-page">
              <Sidebar />
              <div className="flex-1 flex flex-col overflow-hidden">
                <Topbar />
                <main className="p-4 flex-1 overflow-y-auto">
                  <Routes>
                    <Route path="/" element={<Navigate to="/login" replace />} />
                    <Route path="/login" element={<Login />} />
                    <Route path="/bulk" element={<BulkRunner />} />
                    <Route path="/sc" element={<SCRunner />} />
                    <Route path="/cellxgene" element={<Cellxgene />} />
                    <Route path="*" element={<Home />} />
                  </Routes>
                </main>
                <StatusBar />
              </div>
            </div>
          </Router>
        </StatusProvider>
      </SessionProvider>
    </ThemeProvider>
  );
}
