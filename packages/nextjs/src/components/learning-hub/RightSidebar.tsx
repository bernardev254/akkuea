'use client';

import { useState, useEffect } from 'react';
import { ChevronLeft, ChevronRight } from 'lucide-react';

export default function RightSidebar() {
  const [isCollapsed, setIsCollapsed] = useState(false);

  useEffect(() => {
    const savedState = localStorage.getItem('rightSidebarState');
    if (savedState) {
      setIsCollapsed(JSON.parse(savedState));
    }
  }, []);

  const toggleSidebar = () => {
    const newState = !isCollapsed;
    setIsCollapsed(newState);
    localStorage.setItem('rightSidebarState', JSON.stringify(newState));
  };

  return (
    <aside 
      className={`fixed right-0 top-0 h-screen bg-white dark:bg-[#111827] shadow-lg transition-all duration-300 ease-in-out 
        ${isCollapsed ? 'w-16' : 'w-64'}
        md:translate-x-0 
        ${isCollapsed ? 'translate-x-16' : 'translate-x-64'} 
        md:translate-x-0`}
    >
      <button 
        onClick={toggleSidebar}
        className="absolute -left-3 top-1/2 -translate-y-1/2 bg-white dark:bg-[#111827] rounded-full p-1.5 shadow-lg z-50 hover:bg-gray-50 dark:hover:bg-gray-800"
        aria-label={isCollapsed ? "Expand sidebar" : "Collapse sidebar"}
      >
        {isCollapsed ? 
          <ChevronLeft className="text-gray-600 dark:text-gray-400" size={16} /> : 
          <ChevronRight className="text-gray-600 dark:text-gray-400" size={16} />
        }
      </button>

      {!isCollapsed && (
        <div className="p-4">
          <h2 className="text-lg font-medium text-gray-900 dark:text-white mb-6">Discover</h2>
          
          <div className="space-y-6">
            <section>
              <h3 className="flex items-center text-sm font-medium text-teal-600 dark:text-teal-500 mb-3">
                <span className="mr-2">🔍</span>
                Recommendations
              </h3>
              
              <div className="space-y-3">
                <div className="bg-gray-100 dark:bg-[#1F2937] rounded-lg p-3">
                  <div className="flex justify-between items-start mb-1">
                    <span className="text-xs bg-teal-100 dark:bg-teal-500/20 text-teal-600 dark:text-teal-500 px-2 py-0.5 rounded">Article</span>
                    <span className="text-xs text-gray-600 dark:text-gray-400">98% relevant</span>
                  </div>
                  <h4 className="text-sm font-medium text-gray-900 dark:text-gray-200">Introduction to Functional Programming</h4>
                  <p className="text-xs text-gray-600 dark:text-gray-400 mt-1">Kim Cascante</p>
                </div>

                <div className="bg-gray-100 dark:bg-[#1F2937] rounded-lg p-3">
                  <div className="flex justify-between items-start mb-1">
                    <span className="text-xs bg-blue-100 dark:bg-blue-500/20 text-blue-600 dark:text-blue-400 px-2 py-0.5 rounded">Tutorial</span>
                    <span className="text-xs text-gray-600 dark:text-gray-400">92% relevant</span>
                  </div>
                  <h4 className="text-sm font-medium text-gray-900 dark:text-gray-200">Data Visualization with D3.js</h4>
                  <p className="text-xs text-gray-600 dark:text-gray-400 mt-1">Carlos Rodriguez</p>
                </div>

                <div className="bg-gray-100 dark:bg-[#1F2937] rounded-lg p-3">
                  <div className="flex justify-between items-start mb-1">
                    <span className="text-xs bg-teal-100 dark:bg-teal-500/20 text-teal-600 dark:text-teal-500 px-2 py-0.5 rounded">Article</span>
                    <span className="text-xs text-gray-600 dark:text-gray-400">89% relevant</span>
                  </div>
                  <h4 className="text-sm font-medium text-gray-900 dark:text-gray-200">Machine Learning Fundamentals</h4>
                  <p className="text-xs text-gray-600 dark:text-gray-400 mt-1">Ana López</p>
                </div>

                <button className="text-xs text-teal-600 hover:text-teal-700 dark:text-teal-500 dark:hover:text-teal-400">
                  See more recommendations →
                </button>
              </div>
            </section>

            <section>
              <h3 className="flex items-center text-sm font-medium text-teal-600 dark:text-teal-500 mb-3">
                <span className="mr-2">🔥</span>
                Trending
              </h3>
              
              <div className="space-y-3">
                <div className="bg-gray-100 dark:bg-[#1F2937] rounded-lg p-3">
                  <h4 className="text-sm font-medium text-gray-900 dark:text-gray-200">How AI is Transforming Education</h4>
                  <div className="flex justify-between mt-1">
                    <span className="text-xs bg-emerald-100 dark:bg-emerald-500/20 text-emerald-600 dark:text-emerald-400 px-2 py-0.5 rounded">Technology</span>
                    <div className="flex items-center gap-3 text-xs text-gray-600 dark:text-gray-400">
                      <span>156</span>
                      <span>89</span>
                    </div>
                  </div>
                </div>

                <div className="bg-gray-100 dark:bg-[#1F2937] rounded-lg p-3">
                  <h4 className="text-sm font-medium text-gray-900 dark:text-gray-200">Complete Guide to React Hooks</h4>
                  <div className="flex justify-between mt-1">
                    <span className="text-xs bg-purple-100 dark:bg-purple-500/20 text-purple-600 dark:text-purple-400 px-2 py-0.5 rounded">Programming</span>
                    <div className="flex items-center gap-3 text-xs text-gray-600 dark:text-gray-400">
                      <span>124</span>
                      <span>76</span>
                    </div>
                  </div>
                </div>

                <div className="bg-gray-100 dark:bg-[#1F2937] rounded-lg p-3">
                  <h4 className="text-sm font-medium text-gray-900 dark:text-gray-200">Neuroscience-based Study Techniques</h4>
                  <div className="flex justify-between mt-1">
                    <span className="text-xs bg-blue-100 dark:bg-blue-500/20 text-blue-600 dark:text-blue-400 px-2 py-0.5 rounded">Education</span>
                    <div className="flex items-center gap-3 text-xs text-gray-600 dark:text-gray-400">
                      <span>98</span>
                      <span>112</span>
                    </div>
                  </div>
                </div>

                <button className="text-xs text-teal-600 hover:text-teal-700 dark:text-teal-500 dark:hover:text-teal-400">
                  See more trending →
                </button>
              </div>
            </section>

            <button className="w-full bg-teal-500 hover:bg-teal-600 text-white rounded-lg py-2 text-sm font-medium">
              Create Post
            </button>
          </div>
        </div>
      )}
    </aside>
  );
} 