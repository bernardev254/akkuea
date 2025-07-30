import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { Achievement } from '@/lib/types';

const categoryStyles: { [key: string]: string } = {
  Contribution: 'bg-green-500/20 text-green-400 border-green-500/30',
  Community: 'bg-green-500/20 text-green-400 border-green-500/30',
  Learning: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
  Impact: 'bg-purple-500/20 text-purple-400 border-purple-500/30',
};

const iconBgStyles: { [key: string]: string } = {
  completed: 'bg-primary',
  'in-progress': 'bg-primary/30',
  locked: 'bg-primary/30',
};

export const AchievementCard = ({ achievement }: { achievement: Achievement }) => {
  const badgeStyle = categoryStyles[achievement.category] || 'bg-gray-500/20 text-gray-400 border-gray-500/30';
  const iconBgStyle = iconBgStyles[achievement.status];

  return (
    <div className="bg-gray-700 rounded-lg p-4 flex flex-col sm:flex-row gap-4 items-center sm:items-start">
      <div>
        <div className={`${iconBgStyle} p-3 rounded-lg`}>
          <achievement.icon className="w-6 h-6 text-card" />
        </div>
      </div>
      <div className="flex flex-col w-full text-center sm:text-left">
        <div className="flex flex-col sm:flex-row items-center justify-between mb-2">
          <h3 className="text-card font-semibold text-base">{achievement.title}</h3>
          <Badge variant="outline" className={`${badgeStyle} mt-2 sm:mt-0`}>
            {achievement.category}
          </Badge>
        </div>

        <p className="text-muted text-sm">{achievement.description}</p>
        <div className="flex flex-col gap-1 mt-4">
          <div className="flex justify-between">
            {achievement.status === 'completed' && (
              <p className="text-slate-500 text-sm">Completed on {achievement.completedDate}</p>
            )}
            {achievement.status === 'in-progress' && (
              <p className="text-slate-500 text-sm">In progress - {achievement.progressText}</p>
            )}
            {achievement.status !== 'locked' && (
              <span className="text-card font-semibold text-sm">{achievement.progress}%</span>
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
};