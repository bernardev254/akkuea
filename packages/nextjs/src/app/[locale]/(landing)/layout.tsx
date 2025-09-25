import HeaderLanding from '@/components/landing/HeaderLanding';
import { FooterLanding } from '@/components/landing/FooterLanding';

export default function LandingLayout({ children }: { children: React.ReactNode }) {
  return (
    <section>
      <HeaderLanding />
      <main className="pt-20 bg-[#f5f7f8] dark:bg-background text-black dark:text-foreground">
        {children}
      </main>
<<<<<<< HEAD:packages/nextjs/src/app/(landing)/layout.tsx
=======
      <FooterLanding />
>>>>>>> f9dd7f37b7d5da48247896382bd9508c44299e62:packages/nextjs/src/app/[locale]/(landing)/layout.tsx
    </section>
  );
}
