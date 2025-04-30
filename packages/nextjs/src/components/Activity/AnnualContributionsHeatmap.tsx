import React from 'react';

const weekdays = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];

const heatmapData = [
  { month: 'Jan' },
  { month: 'Feb' },
  { month: 'Mar' },
  { month: 'Apr' },
  { month: 'May' },
  { month: 'Jun' },
  { month: 'Jul' },
  { month: 'Aug' },
  { month: 'Sep' },
  { month: 'Oct' },
  { month: 'Nov' },
  { month: 'Dec' },
];

// Function to determine the color based on activity level (0 to 4)
const getActivityColor = (level: number): string => {
  switch (level) {
    case 0:
      return 'bg-gray-200 dark:bg-gray-700'; // Lightest (no activity)
    case 1:
      return 'bg-teal-100 dark:bg-teal-900';
    case 2:
      return 'bg-teal-300 dark:bg-teal-700';
    case 3:
      return 'bg-teal-500 dark:bg-teal-500';
    case 4:
      return 'bg-teal-700 dark:bg-teal-300'; // Darkest (highest activity)
    default:
      return 'bg-gray-200 dark:bg-gray-700';
  }
};

const AnnualContributions: React.FC = () => {
  return (
    <div className="mb-8 w-full border rounded-xl md:pl-8 p-3 md:w-[1400px] shadow-lg dark:border-gray-700 dark:bg-gray-800 transition-colors duration-300">
      <h2 className="text-base font-medium mb-4 text-gray-800 dark:text-gray-200">Annual Contributions</h2>

      <div className="flex">
        {/* Weekday labels column */}
        <div className="flex flex-col mr-2 pt-5">
          {weekdays.map((day) => (
            <div key={day} className="h-5 flex items-center justify-end text-xs text-gray-500 dark:text-gray-400 mt-1">
              {day}
            </div>
          ))}
        </div>

        {/* Main content area */}
        <div className="flex-1">
          {/* Month labels row */}
          <div className="flex md:w-[1300px] justify-between mb-4 text-xs text-gray-500 dark:text-gray-400">
            {heatmapData.map((month) => (
              <div key={month.month} className="flex-1 text-center">
                {month.month}
              </div>
            ))}
          </div>

          {/* Activity grid */}
          <div className="grid grid-rows-5 md:w-[1300px] md:ml-10 gap-[2px]">
            {weekdays.map((_, dayIndex) => (
              <div key={`day-row-${dayIndex}`} className="grid grid-cols-12 grid-rows-2 gap-[2px]">
                {heatmapData.map((month, monthIndex) => {
                  // Pattern to match the image
                  const pattern = [
                    [2, 0, 2, 2, 2, 0, 2, 2, 0, 2, 0, 2], // Mon
                    [2, 0, 2, 2, 0, 2, 0, 0, 0, 2, 2, 2], // Tue
                    [0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 4, 0], // Wed
                    [0, 4, 0, 0, 4, 0, 4, 4, 4, 0, 0, 0], // Thu
                    [2, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0], // Fri
                    [0, 2, 0, 0, 2, 0, 2, 0, 0, 2, 2, 0], // Sat
                    [2, 0, 2, 0, 0, 2, 0, 2, 2, 0, 0, 2], // Sun
                  ];

                  const activityLevel = pattern[dayIndex][monthIndex];
                  return (
                    <div
                      key={`${month.month}-${dayIndex}`}
                      className={`h-[10px] w-[10px] rounded-sm ${getActivityColor(activityLevel)}`}
                    />
                  );
                })}
              </div>
            ))}
          </div>

          {/* Legend */}
          <div className="flex items-center justify-end mt-4">
            <span className="text-xs text-gray-500 dark:text-gray-400 mr-2">Less</span>
            {[0, 1, 2, 3, 4].map((level) => (
              <div
                key={`legend-${level}`}
                className={`h-[10px] w-[10px] rounded-sm mr-1 ${getActivityColor(level)}`}
              />
            ))}
            <span className="text-xs text-gray-500 dark:text-gray-400 ml-1">More</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default AnnualContributions;