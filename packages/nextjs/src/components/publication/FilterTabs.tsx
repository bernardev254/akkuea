import React from 'react';

interface FilterTab {
  id: string;
  label: string;
  count: number;
}

interface FilterTabsProps {
  tabs: FilterTab[];
  activeTab: string;
  onTabChange: (tabId: string) => void;
}

export const FilterTabs: React.FC<FilterTabsProps> = ({ tabs, activeTab, onTabChange }) => {
  return (
    <div className="flex flex-wrap gap-2">
      {tabs.map((tab) => (
        <button
          key={tab.id}
          className={`px-3 py-1 text-sm rounded-full ${
            activeTab === tab.id
              ? 'bg-gray-800 text-white'
              : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
          }`}
          onClick={() => onTabChange(tab.id)}
        >
          {tab.label} {tab.count > 0 && <span>{tab.count}</span>}
        </button>
      ))}
    </div>
  );
};
