'use client';

import React, { ReactNode } from 'react';
import { useTab, TabType } from '@/contexts/TabContext';

interface TabNavProps {
  children: ReactNode;
}

export const TabNav: React.FC<TabNavProps> = ({ children }) => {
  return (
    <div className="flex flex-col sm:flex-row items-start sm:items-center justify-start sm:justify-center gap-2 sm:space-x-2 mb-6 overflow-x-auto p-2 bg-[#F4F4F5] dark:bg-gray-800/50 rounded-lg w-full">
      {children}
    </div>
  );
};

interface TabItemProps {
  icon: ReactNode;
  label: string;
  value: TabType;
}

export const TabItem: React.FC<TabItemProps> = ({ icon, label, value }) => {
  const { activeTab, setActiveTab } = useTab();
  const isActive = activeTab === value;

  return (
    <button
      className={`flex items-center gap-2 px-4 sm:px-6 md:px-8 py-2 rounded-lg font-medium w-full sm:w-auto
        ${
          isActive
            ? 'bg-white dark:bg-gray-700 text-black dark:text-white shadow-sm transition-all duration-200 ease-in-out'
            : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800'
        }`}
      onClick={() => setActiveTab(value)}
    >
      {icon}
      <span>{label}</span>
    </button>
  );
};

interface TabContentProps {
  value: TabType;
  children: ReactNode;
}

export const TabContent: React.FC<TabContentProps> = ({ value, children }) => {
  const { activeTab } = useTab();

  if (activeTab !== value) return null;

  return <>{children}</>;
};
