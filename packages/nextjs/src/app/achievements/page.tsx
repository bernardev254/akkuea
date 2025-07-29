import { Trophy, Star, Users, Code, BookOpen, MessageCircle, Share2, Award } from 'lucide-react';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';

const achievements = [
  {
    id: 1,
    title: 'Star Contributor',
    description: 'Publish more than 50 quality content pieces',
    icon: Star,
    category: 'Contribution',
    categoryColor: 'bg-green-500/20 text-green-400 border-green-500/30',
    progress: 100,
    status: 'completed',
    completedDate: 'March 15, 2023',
    iconBg: 'bg-primary',
  },
  {
    id: 2,
    title: 'Outstanding Mentor',
    description: 'Help more than 20 students with their questions',
    icon: Users,
    category: 'Community',
    categoryColor: 'bg-green-500/20 text-green-400 border-green-500/30',
    progress: 100,
    status: 'completed',
    completedDate: 'April 2, 2023',
    iconBg: 'bg-primary',
  },
  {
    id: 3,
    title: 'Code Expert',
    description: 'Share 30 code examples',
    icon: Code,
    category: 'Contribution',
    categoryColor: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
    progress: 75,
    status: 'in-progress',
    progressText: '23/30',
    iconBg: 'bg-primary/30',
  },
  {
    id: 4,
    title: 'Avid Reader',
    description: 'Read more than 100 articles',
    icon: BookOpen,
    category: 'Learning',
    categoryColor: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
    progress: 60,
    status: 'in-progress',
    progressText: '60/100',
    iconBg: 'bg-primary/30',
  },
  {
    id: 5,
    title: 'Effective Communicator',
    description: 'Receive 50 positive comments on your posts',
    icon: MessageCircle,
    category: 'Community',
    categoryColor: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
    progress: 0,
    status: 'locked',
    iconBg: 'bg-primary/30',
  },
  {
    id: 6,
    title: 'Educational Influencer',
    description: 'Get 30 people to share your content',
    icon: Share2,
    category: 'Impact',
    categoryColor: 'bg-purple-500/20 text-purple-400 border-purple-500/30',
    progress: 0,
    status: 'locked',
    iconBg: 'bg-primary/30',
  },
];

export default function Component() {
  const completedAchievements = achievements.filter((a) => a.status === 'completed');
  const inProgressAchievements = achievements.filter((a) => a.status === 'in-progress');
  const lockedAchievements = achievements.filter((a) => a.status === 'locked');

  const AchievementCard = ({ achievement }: { achievement: (typeof achievements)[0] }) => (
    <div className="bg-gray-700 rounded-lg p-4 flex flex-col sm:flex-row gap-4 items-center sm:items-start">
      <div>
        <div className={`${achievement.iconBg} p-3 rounded-lg`}>
          <achievement.icon className="w-6 h-6 text-white" />
        </div>
      </div>
      <div className="flex flex-col w-full text-center sm:text-left">
        <div className="flex flex-col sm:flex-row items-center justify-between mb-2">
          <h3 className="text-white font-semibold text-base">{achievement.title}</h3>
          <Badge variant="outline" className={`${achievement.categoryColor} mt-2 sm:mt-0`}>
            {achievement.category}
          </Badge>
        </div>

        <p className="text-slate-400 text-sm">{achievement.description}</p>
        <div className="flex flex-col gap-1 mt-4">
          <div className='flex justify-between'>
            {achievement.status === 'completed' && (
              <p className="text-slate-500 text-sm">Completed on {achievement.completedDate}</p>
            )}
            {achievement.status === 'in-progress' && (
              <p className="text-slate-500 text-sm">In progress - {achievement.progressText}</p>
            )}
             {achievement.status !== 'locked' && (
               <span className="text-white font-semibold text-sm">{achievement.progress}%</span>
             )}
          </div>

          {achievement.status !== 'locked' && (
            <div className="flex items-center justify-between">
              <Progress value={achievement.progress} className="flex-1 " />
            </div>
          )}
        </div>
      </div>
    </div>
  );

  return (
    <>
      <div className="min-h-screen bg-black text-white p-6">
        <div className="max-w-7xl mx-auto space-y-8">
          {/* Header */}
          <div className="flex items-center space-x-2 sm:space-x-3">
            <Award className="w-6 h-6 sm:w-8 sm:h-8 text-primary" />
            <h1 className="text-2xl sm:text-3xl font-bold text-primary">Achievements</h1>
          </div>

          <article className="bg-gray-700 w-full p-5 rounded-md space-y-5">
            <div className="flex flex-col lg:flex-row justify-between lg:items-center gap-6">
              <div className="text-center lg:text-left">
                <h2 className="text-lg sm:text-xl font-semibold text-white">Your progress</h2>
                <div className="flex items-center justify-center lg:justify-start space-x-2 text-slate-300 text-sm">
                  <Trophy className="w-5 h-5 text-primary" />
                  <span>You have completed 12 of 30 achievements</span>
                </div>
              </div>
              <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 w-full lg:w-auto">
                <div className="text-center">
                  <div className="text-xl sm:text-2xl font-bold text-primary">12</div>
                  <div className="text-slate-400 text-[11px] sm:text-xs">Completed achievements</div>
                </div>
                <div className="text-center">
                  <div className="text-xl sm:text-2xl font-bold text-primary">Intermediate</div>
                  <div className="text-slate-400 text-[11px] sm:text-xs">Current level</div>
                </div>
                <div className="text-center">
                  <div className="text-xl sm:text-2xl font-bold text-primary">1,245</div>
                  <div className="text-slate-400 text-[11px] sm:text-xs">Experience points</div>
                </div>
                <div className="text-center">
                  <div className="text-xl sm:text-2xl font-bold text-primary">#342</div>
                  <div className="text-slate-400 text-[11px] sm:text-xs">Ranking position</div>
                </div>
              </div>
            </div>
            <div>
              <Progress value={40} className="w-full" />
            </div>
          </article>

          {/* Achievement Tabs */}
          <Tabs defaultValue="all" className="w-full">
            <TabsList className="grid w-full grid-cols-2 sm:grid-cols-4 bg-black">
              <TabsTrigger value="all" className="data-[state=active]:bg-primary data-[state=active]:text-white">
                All
              </TabsTrigger>
              <TabsTrigger value="completed" className="data-[state=active]:bg-primary data-[state=active]:text-white">
                Completed
              </TabsTrigger>
              <TabsTrigger value="in-progress" className="data-[state=active]:bg-primary data-[state=active]:text-white">
                In progress
              </TabsTrigger>
              <TabsTrigger value="locked" className="data-[state=active]:bg-primary data-[state=active]:text-white">
                Locked
              </TabsTrigger>
            </TabsList>

            <TabsContent value="all" className="mt-6">
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {achievements.map((achievement) => (
                  <AchievementCard key={achievement.id} achievement={achievement} />
                ))}
              </div>
            </TabsContent>

            <TabsContent value="completed" className="mt-6">
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {completedAchievements.map((achievement) => (
                  <AchievementCard key={achievement.id} achievement={achievement} />
                ))}
              </div>
            </TabsContent>

            <TabsContent value="in-progress" className="mt-6">
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {inProgressAchievements.map((achievement) => (
                  <AchievementCard key={achievement.id} achievement={achievement} />
                ))}
              </div>
            </TabsContent>

            <TabsContent value="locked" className="mt-6">
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {lockedAchievements.map((achievement) => (
                  <AchievementCard key={achievement.id} achievement={achievement} />
                ))}
              </div>
            </TabsContent>
          </Tabs>
        </div>
      </div>
    </>
  );
}
