import React from "react";

const TabNavigation = ({ activeTab, setActiveTab, tabs }) => {
  return (
    <div className="flex bg-white dark:bg-gray-800 rounded-lg p-1 border border-gray-200 dark:border-gray-700  transition-colors duration-300">
      {tabs.map((tab) => (
        <button
          key={tab.id}
          onClick={() => setActiveTab(tab.id)}
          className={`px-6 py-2 rounded-md font-medium transition-colors w-full justify-center flex items-center gap-2 ${
            activeTab === tab.id
              ? "bg-[#59C9D0] hover:bg-[#4ab5bc] text-white dark:bg-teal-400"
              : "text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-gray-100"
          }`}
        >
          <span>{tab.icon}</span>
          {tab.label}
        </button>
      ))}
    </div>
  );
};

export default TabNavigation;