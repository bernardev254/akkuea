import React from 'react';
import Image from 'next/image';
import { CategoryTag } from './CategoryTag';
import { Publication } from '../types/index';

interface PublicationCardProps {
  publication: Publication;
}

export const PublicationCard: React.FC<PublicationCardProps> = ({ publication }) => {
  console.log(publication);
  return (
    <div className="bg-white rounded-lg shadow overflow-hidden flex flex-col h-full">
      {/* Card Image and Date */}
      <div className="relative">
        <div className="aspect-w-16 aspect-h-9">
          <Image
            src={publication.image}
            alt={publication.title}
            className="w-full h-full object-cover"
            width={300}
            height={170}
          />
        </div>
        {/* Date Badge */}
        <div className="absolute top-2  text-white w-32 h-7 right-2 bg-black/30 text-base font-medium px-2 py-1 rounded">
          {publication.date}
        </div>
        {/* Video Play Button if applicable */}
        {publication.hasVideo && (
          <div className="absolute inset-0 flex items-center justify-center">
            <div className="bg-red-600 text-white rounded-full p-2">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
                />
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
            </div>
          </div>
        )}
      </div>

      {/* Card Content */}
      <div className=" flex-grow flex flex-col">
        <div className="p-4">
          {/* Category Tag */}
          <div className="mb-3">
            <CategoryTag category={publication.category} />
          </div>

          {/* Title */}
          <h3 className="text-lg lead font-semibold mb-2">{publication.title}</h3>

          {/* Description */}
          <p className="text-sm text-gray-600 mb-4 flex-grow">{publication.description}</p>
        </div>
        {/* Author and View Button */}
        <div className="flex items-center justify-between mt-auto h-16 p-4 border-t">
          <div className="flex items-center">
            <div className="w-8 h-8 rounded-full overflow-hidden mr-2">
              <Image
                src={publication.author.avatar}
                alt={publication.author.name}
                className="w-full h-full object-cover"
                width={32}
                height={32}
              />
            </div>
            <span className="text-sm text-gray-700">{publication.author.name}</span>
          </div>
          <button className="text-[#00CED1] text-base  font-medium flex items-center">
            View
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-4 w-4 ml-1"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  );
};
