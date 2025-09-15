import CommunitySection from '@/components/landing/CommunitySection';
import Benefits from '@/components/landing/Benefits';
import HeroSection from '@/components/landing/HeroSection';
export default function LandingPage() {
  return (
    <div>
      <HeroSection/>
      <Benefits />
      <CommunitySection />
    </div>
  );
}
