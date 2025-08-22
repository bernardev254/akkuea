import type { Metadata } from 'next';
import LearningHubSidebar from '@/components/learning-hub/learning-hub-sidebar';
import RightSidebar from '@/components/learning-hub/right-sidebar';
import { ThemeProvider } from '@/components/theme-provider';
import './globals.css';
import Navbar from '@/components/navbar/navbar';
import { SidebarProvider } from '@/components/ui/sidebar';

export const metadata: Metadata = {
  title: 'Learning Hub',
  description: 'A platform for continuous learning and development',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body>
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          disableTransitionOnChange
        >
          <SidebarProvider defaultOpen={true}>
            <Navbar />
            <div className="grid grid-cols-[320px_1fr_256px] min-h-screen bg-background text-foreground">
              <LearningHubSidebar />
              <main className="flex justify-center mt-14 px-4 py-8 pl-[4em]">
                <div className="w-full max-w-5xl">{children}</div>
              </main>
              <RightSidebar />
            </div>
          </SidebarProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}
