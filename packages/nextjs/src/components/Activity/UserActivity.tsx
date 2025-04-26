"use client"


import { useState } from 'react';
import AnnualContributionsHeatmap from './AnnualContributionsHeatmap';
import { 
  Calendar, 
  Clock, 
  MessageSquare, 
  ThumbsUp, 
  FileText, 
  Award, 
  Users 
} from 'lucide-react';

interface HeatmapDay {
  month: string;
  days: number[][];
}

interface Activity {
  id: number;
  type: 'comment' | 'like' | 'post' | 'achievement' | 'join';
  content: string;
  timeAgo: string;
}

const UserActivityDashboard = () => {
  const [activeTab, setActiveTab] = useState('Activity');
  const tabs = ['Activity', 'Achievements', 'Statistics'];

  // Generate sample heatmap data
  const generateHeatmapData = () => {
    const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
    return months.map(month => ({
      month,
      days: Array(7).fill(0).map(() => Array(4).fill(0).map(() => Math.floor(Math.random() * 5)))
    }));
  };

  const heatmapData = generateHeatmapData();

  // Sample recent activity data
  const recentActivity = [
    { 
      id: 1, 
      type: 'comment', 
      content: 'Commented on \'Introduction to Functional Programming\'', 
      timeAgo: 'Today' 
    },
    { 
      id: 2, 
      type: 'like', 
      content: 'Liked \'Data Visualization with D3.js\'', 
      timeAgo: 'Yesterday' 
    },
    { 
      id: 3, 
      type: 'post', 
      content: 'Posted \'Advanced Machine Learning Techniques\'', 
      timeAgo: '2 days ago' 
    },
    { 
      id: 4, 
      type: 'achievement', 
      content: 'Earned the \'Frequent Contributor\' achievement', 
      timeAgo: '3 days ago' 
    },
    { 
      id: 5, 
      type: 'join', 
      content: 'Joined the \'Web Development\' community', 
      timeAgo: '5 days ago' 
    }
  ];

  // Helper function to get activity level color
  const getActivityColor = (level: number): string => {
    switch(level) {
      case 0: return "bg-gray-900";
      case 1: return "bg-teal-900";
      case 2: return "bg-teal-700";
      case 3: return "bg-teal-500";
      case 4: return "bg-teal-300";
      default: return "bg-gray-900";
    }
  };

  // Helper function to get icon for activity type
  const getActivityIcon = (type: Activity['type'] | string): JSX.Element => {
    switch(type) {
      case 'comment': return <MessageSquare size={16} className="text-teal-400" />;
      case 'like': return <ThumbsUp size={16} className="text-teal-400" />;
      case 'post': return <FileText size={16} className="text-teal-400" />;
      case 'achievement': return <Award size={16} className="text-teal-400" />;
      case 'join': return <Users size={16} className="text-teal-400" />;
      default: return <Clock size={16} className="text-teal-400" />;
    }
  };

  return (
    <div className="container mx-auto p-6 bg-white rounded-lg shadow mb-10">
      {/* Header with title */}
      <div className="flex items-center mb-4 text-teal-600">
        <Calendar size={18} className="mr-2" />
        <h1 className="text-lg font-bold">User Activity</h1>
      </div>
      
      {/* Tab navigation */}
      <div className="flex bg-gray-100 rounded-lg p-1 mb-4">
        {tabs.map(tab => (
          <button
            key={tab}
            className={`flex-1 py-2 px-4 text-sm font-medium rounded-md ${
              activeTab === tab 
                ? 'bg-white text-teal-600 shadow' 
                : 'text-gray-600 hover:text-teal-500'
            }`}
            onClick={() => setActiveTab(tab)}
          >
            {tab}
          </button>
        ))}
      </div>

      {/* Content based on active tab */}
      {activeTab === 'Activity' && (
        <>
          {/* Annual Contributions Heatmap */}
          <AnnualContributionsHeatmap/>

          {/* Recent Activity List */}
          <div className="mt-6">
            <h2 className="text-base font-medium mb-3">Recent Activity</h2>
            <div className="flex flex-col space-y-4">
              {recentActivity.map(activity => (
                <div key={activity.id} className="flex items-start">
                  <div className="mt-1 p-1 bg-teal-100 bg-opacity-20 rounded">
                    {getActivityIcon(activity.type)}
                  </div>
                  <div className="ml-3">
                    <p className="text-sm">{activity.content}</p>
                    <p className="text-xs text-gray-500">{activity.timeAgo}</p>
                  </div>
                </div>
              ))}
            </div>
            <div className="mt-4 text-right">
              <button className="text-sm text-teal-400 hover:text-teal-500">
                View all activity history
              </button>
            </div>
          </div>
        </>
      )}

      {activeTab === 'Achievements' && (
        <div className="h-52 flex items-center justify-center text-gray-500">
          Achievements content would go here
        </div>
      )}

      {activeTab === 'Statistics' && (
        <div className="h-52 flex items-center justify-center text-gray-500">
          Statistics content would go here
        </div>
      )}
    </div>
  );
};

export default UserActivityDashboard;