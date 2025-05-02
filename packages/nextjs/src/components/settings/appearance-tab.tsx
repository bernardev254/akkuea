'use client'
import { Palette } from 'lucide-react';
import { ThemeSwitcher } from './theme-switcher';
import { SectionContainer, SectionDescription } from './section-components';

export interface AppearanceTabProps {
  isDarkMode: boolean;
}

export function AppearanceTab({ isDarkMode }: AppearanceTabProps) {
  return (
    <div className="p-6">
      <div className="flex items-center mb-2">
        <Palette className={isDarkMode ? "w-6 h-6 mr-2 text-teal-400" : "w-6 h-6 mr-2 text-teal-600"} />
        <h1 className={isDarkMode ? "text-2xl font-bold text-teal-400" : "text-2xl font-bold text-teal-700"}>
          Appearance Settings
        </h1>
      </div>
      <SectionDescription 
        text="Customize the look and feel of the interface" 
        isDarkMode={isDarkMode} 
      />
      <div className="mb-8"></div>
      
      {/* Theme Switcher */}
      <SectionContainer>
        <ThemeSwitcher />
      </SectionContainer>
      
      {/* Additional appearance settings could go here */}
    </div>
  );
}