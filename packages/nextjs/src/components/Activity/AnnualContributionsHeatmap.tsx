const AnnualContributionsHeatmap = () => {
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
  
    // Generate sample heatmap data
    const generateHeatmapData = () => {
      const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
      return months.map(month => ({
        month,
        days: Array(7).fill(0).map(() => Math.floor(Math.random() * 5))
      }));
    };
  
    const heatmapData = generateHeatmapData();
    const weekdays = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
  
    return (
      <div className="mb-8">
        <h2 className="text-base font-medium mb-4">Annual Contributions</h2>
        
        <div className="flex">
          {/* Weekday labels column */}
          <div className="flex flex-col mr-2 pt-6">
            {weekdays.map(day => (
              <div key={day} className="h-6 flex items-center justify-end text-xs text-gray-500 mb-1">
                {day}
              </div>
            ))}
          </div>
          
          {/* Main content area */}
          <div className="flex-1">
            {/* Month labels row */}
            <div className="flex justify-between w-[1350px] mb-2 text-xs text-gray-500">
              {heatmapData.map(month => (
                <div key={month.month}>{month.month}</div>
              ))}
            </div>
            
            {/* Activity grid */}
            <div className="grid grid-rows-2 gap-1">
              {weekdays.map((_, dayIndex) => (
                <div key={`day-row-${dayIndex}`} className="grid grid-cols-12 gap-1">
                  {heatmapData.map((month, monthIndex) => {
                    // For demo purposes, generate a consistent pattern similar to the image
                    const pattern = [
                      [4, 0, 2, 2, 2, 0, 2, 2, 0, 2, 0, 2], // Mon
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
                        className={`h-4 w-4 rounded-sm ${getActivityColor(activityLevel)}`}
                      />
                    );
                  })}
                </div>
              ))}
            </div>
            
            {/* Legend */}
            <div className="flex items-center justify-end mt-4">
              <span className="text-xs text-gray-500 mr-2">Less</span>
              {[0, 1, 2, 3, 4].map(level => (
                <div
                  key={`legend-${level}`}
                  className={`h-3 w-3 rounded-sm mr-1 ${getActivityColor(level)}`}
                />
              ))}
              <span className="text-xs text-gray-500 ml-1">More</span>
            </div>
          </div>
        </div>
      </div>
    );
  };
  
  export default AnnualContributionsHeatmap;