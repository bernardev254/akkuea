'use client';

import { BookOpen, Tags } from 'lucide-react';
import Post from './post';
import { Toaster } from 'sonner';
import { useModalStore } from '@/store/useModalStore';

export default function Page() {
  const { onOpen } = useModalStore();

  const component = (
    <div className="max-w-4xl w-full mx-auto px-8">
      <div className="w-full">
        <Post
          id="1"
          author={{
            name: 'Sebastián Salazar',
            username: 'sebastiánsalazar',
            avatar: '/placeholder.svg',
          }}
          content={{
            text: "Check out this interactive particle effect! Click to create more particles that move around the screen. Ideal for visualising systems that multiply, like stars in space.\n\nHere's also an interesting article about particle systems:",
            media: [
              {
                type: 'video',
                url: 'https://www.youtube.com/embed/NWdEOAYm4FA?si=WbvhIlP1GBZkfwUt',
                aspectRatio: 16 / 9,
                downloadUrl: 'https://example.com/video.mp4',
              },
            ],
          }}
          categories={[
            {
              name: 'Computer',
              icon: <BookOpen className="h-4 w-4" />,
            },
            {
              name: 'Interactive Visualization',
              icon: <Tags className="h-4 w-4" />,
            },
          ]}
          modal={() => onOpen(component)}
        />
        <Toaster />
      </div>
    </div>
  );

  return component;
}
