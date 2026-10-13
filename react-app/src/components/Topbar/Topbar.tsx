// src/components/Topbar/Topbar.tsx
import { useContext } from 'react';
import { SessionContext } from '../../store/SessionContext';
import { ThemeContext } from '../../store/ThemeContext';
import { SunIcon, MoonIcon } from '@heroicons/react/outline';
import React from 'react';

export default function Topbar() {
  const { user, logout } = useContext(SessionContext);
  const { isDark, toggleDark } = useContext(ThemeContext);

  return (
    <header className="flex justify-between items-center p-4 bg-surface dark:bg-surface-dark shadow-sm">
      <h1 className="text-lg font-medium text-primary">RNA‑seq Pipeline</h1>
      <div className="flex items-center space-x-4">
        <button onClick={toggleDark} className="p-1 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700">
          {isDark ? (
            <SunIcon className="h-5 w-5 text-yellow-500" />
          ) : (
            <MoonIcon className="h-5 w-5 text-gray-700" />
          )}
        </button>
        <span className="text-sm">{user?.email ?? 'Guest'}</span>
        {user && (
          <button
            onClick={logout}
            className="px-3 py-1 rounded bg-error text-white hover:bg-error-dark"
          >
            Logout
          </button>
        )}
      </div>
    </header>
  );
}


