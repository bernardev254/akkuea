import React from "react";

const PopularTopics = ({ topics }) => {
  return (
    <div className="mb-8">
      <h2 className="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4 mt-8">
        Popular Topics
      </h2>
      <div className="flex flex-wrap gap-2">
        {topics.map((topic) => (
          <button
            key={topic}
            className="px-4 py-2 bg-[#59C9D0] dark:bg-teal-900/30 hover:bg-cyan-200 dark:hover:bg-teal-800 text-cyan-700 dark:text-teal-300 rounded-full text-sm font-medium transition-colors duration-300"
          >
            {topic}
          </button>
        ))}
      </div>
    </div>
  );
};

export default PopularTopics;