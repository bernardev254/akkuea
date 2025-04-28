'use client';
import React from 'react';
import { Edit, MapPin, MessageCircle, Share2, Flag as LinkIcon, Calendar } from 'lucide-react';
import { Flag } from 'lucide-react';
import UserPost from '@/components/quickPost/post-grid/userprofilepost';
import UserActivityDashboard from '@/components/Activity/UserActivity';
import PublicationMain from '@/components/publication/PublicationMain';

const ProfilePage = () => {
  return (
    <div className="bg-white md:h-screen  shadow-sm p-6 ">
      <div className="flex flex-col md:flex-row items-start gap-6 md:pt-7 md:pl-10 container mx-auto rounded-lg py-8 px-4  border">
        <div className="w-full md:w-auto flex flex-col items-center justify-between md:items-start md:h-[270px]  pt-6">
          <div className="w-32 h-32 md:w-44 md:h-44 rounded-full ring-4 ring-white bg-gradient-to-br from-gray-100 to-gray-200 flex items-center justify-center overflow-hidden shadow-lg">
            <svg
              className="w-16 h-16 md:w-20 md:h-20 text-gray-400 transition-transform hover:scale-110 duration-300"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z" />
            </svg>
          </div>
        </div>

        <div className="flex-1 pt-2 w-full mx-3">
          {/* username */}
          <div className='border-b pb-4'>
            <div className="flex flex-col justify-between sm:flex-row sm:items-center gap-4 mb-6">
              <div>
                <h1 className="text-2xl font-bold text-gray-900 hover:text-teal-600 transition-colors duration-200">
                  Jefferson Calderon
                </h1>
                <div className="text-gray-500 font-normal">@xJeffx23</div>
              </div>
              <button className="px-4 py-2 border border-gray-300 rounded-lg text-sm font-medium hover:bg-gray-50 transition-all duration-200 hover:shadow-sm flex items-center gap-2 text-gray-700">
                <Edit size={16} />
                Edit Profile
              </button>
            </div>

            <p className=" text-gray-700 leading-relaxed mb-4 md:w-1/2">
              I am a crack designer, I like software engineering and also mixing as a DJ. Passionate
              about creating beautiful and functional digital experiences.
            </p>

            <div className="flex gap-2 mt-4">
              <span className="px-4 py-1.5 bg-teal-50 text-teal-600 rounded-full text-sm font-medium hover:bg-teal-100 transition-colors duration-200">
                Student
              </span>
              <span className="px-4 py-1.5 bg-blue-50 text-blue-600 rounded-full text-sm font-medium hover:bg-blue-100 transition-colors duration-200">
                Designer
              </span>
            </div>
          </div>
          {/* following */}

          <div className="flex flex-row gap-8 my-4  border-b py-3 ">
            <div className="flex flex-col items-center">
              <span className="font-bold text-xl text-gray-900">15</span>
              <span className="text-gray-600 text-sm">posts</span>
            </div>
            <div className="flex flex-col items-center">
              <span className="font-bold text-xl text-gray-900">1,234</span>
              <span className="text-gray-600 text-sm">followers</span>
            </div>
            <div className="flex flex-col items-center">
              <span className="font-bold text-xl text-gray-900">567</span>
              <span className="text-gray-600 text-sm">following</span>
            </div>
          </div>

           

          <p className='font-normal text-gray-500  mb-2'>National University of Design</p>
          <span className='text-[#00CED1] font-normal mt-5'>"Creating the future through design and technology"</span>
        </div>
      </div>
      <PublicationMain />
      <UserActivityDashboard />
    </div>
  );
};

export default ProfilePage;
