// src/components/Sidebar/Sidebar.tsx
import { Link } from 'react-router-dom';
import { MenuIcon, XIcon } from '@heroicons/react/outline';
import React from 'react';

export default function Sidebar() {
  const [open, setOpen] = React.useState(false);
  return (
    <nav className={`md:fixed md:top-0 md:left-0 md:h-full ${open ? '' : 'hidden md:block'} w-64 bg-surface dark:bg-surface p-4 overflow-y-auto`}>
      <div className="flex items-center justify-between md:hidden mb-4">
        <h2 className="text-xl font-semibold text-primary">RNA‑seq</h2>
        <button onClick={() => setOpen(prev => !prev)}>
          {open ? <XIcon className="h-6 w-6" /> : <MenuIcon className="h-6 w-6" />}
        </button>
      </div>
      <ul className="space-y-2">
        <li>
          <Link to="/bulk" className="block px-3 py-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700">
            Bulk RNA‑seq
          </Link>
        </li>
        <li>
          <Link to="/sc" className="block px-3 py-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700">
            Single‑cell RNA‑seq
          </Link>
        </li>
        <li>
          <Link to="/cellxgene" className="block px-3 py-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700">
            CellxGene
          </Link>
        </li>
      </ul>
    </nav>
  );
}