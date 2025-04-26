 "use client"
import React, { useState, useEffect } from 'react';
import { FilterTabs } from './FilterTabs';
import { SearchBar } from './SearchBar';
import { PublicationsGrid } from './PublicationsGrid';
import { Publication } from '../types/index';

interface PublicationsSectionProps {
  publications: Publication[];
}

export const PublicationsSection: React.FC<PublicationsSectionProps> = ({ publications }) => {
  const [activeTab, setActiveTab] = useState('all');
  const [searchQuery, setSearchQuery] = useState('');
  const [filteredPublications, setFilteredPublications] = useState<Publication[]>(publications);
  
  const tabs = [
    { id: 'all', label: 'All', count: publications.length },
    { id: 'pedagogy', label: 'Pedagogy', count: publications.filter(p => p.category.type === 'pedagogy').length },
    { id: 'technology', label: 'Technology', count: publications.filter(p => p.category.type === 'technology').length },
    { id: 'socioemotional', label: 'Socioemotional', count: publications.filter(p => p.category.type === 'socioemotional').length },
    { id: 'methodologies', label: 'Methodologies', count: publications.filter(p => p.category.type === 'methodologies').length },
  ];
  
  useEffect(() => {
    let filtered = publications;
    
    // Filter by category
    if (activeTab !== 'all') {
      filtered = filtered.filter(pub => pub.category.type === activeTab);
    }
    
    // Filter by search query
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(
        pub => 
          pub.title.toLowerCase().includes(query) || 
          pub.description.toLowerCase().includes(query) ||
          pub.author.name.toLowerCase().includes(query)
      );
    }
    
    setFilteredPublications(filtered);
  }, [activeTab, searchQuery, publications]);
  
  return (
    <div className="bg-gray-50 p-6 rounded-lg">
      <div className="flex flex-col md:flex-row md:items-center justify-between mb-6 gap-4">
        <h2 className="text-2xl font-bold">Publications</h2>
        <div className="md:w-64">
          <SearchBar onSearch={setSearchQuery} />
        </div>
      </div>
      
      <div className="mb-6">
        <FilterTabs tabs={tabs} activeTab={activeTab} onTabChange={setActiveTab} />
      </div>
      
      <PublicationsGrid publications={filteredPublications} />
    </div>
  );
};