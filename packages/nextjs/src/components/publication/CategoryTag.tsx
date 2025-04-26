import React from 'react';

interface CategoryTagProps {
  category: {
    name: string;
    type: 'pedagogy' | 'technology' | 'socioemotional' | 'methodologies' | 'neuroscience' | 'inclusive';
  };
}

export const CategoryTag: React.FC<CategoryTagProps> = ({ category }) => {
  const categoryConfig = {
    pedagogy: {
      bgColor: 'bg-emerald-100',
      textColor: 'text-emerald-700',
      icon: '📚',
      label: 'Pedagogy'
    },
    technology: {
      bgColor: 'bg-blue-100',
      textColor: 'text-blue-700',
      icon: '💻',
      label: 'Educational Technology'
    },
    socioemotional: {
      bgColor: 'bg-purple-100',
      textColor: 'text-purple-700',
      icon: '💜',
      label: 'Socioemotional Development'
    },
    methodologies: {
      bgColor: 'bg-yellow-100',
      textColor: 'text-yellow-700',
      icon: '📋',
      label: 'Active Methodologies'
    },
    neuroscience: {
      bgColor: 'bg-red-100',
      textColor: 'text-red-700',
      icon: '🧠',
      label: 'Neuroscience'
    },
    inclusive: {
      bgColor: 'bg-indigo-100',
      textColor: 'text-indigo-700',
      icon: '📊',
      label: 'Inclusive Education'
    }
  };
  
  const config = categoryConfig[category.type];
  
  return (
    <div className={`${config.bgColor} ${config.textColor} text-base px-2 py-1 rounded-md inline-flex items-center`}>
      <span className="mr-1">{config.icon}</span>
      {config.label}
    </div>
  );
};