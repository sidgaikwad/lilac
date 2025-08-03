import { getFormattedTime, getRelativeTime } from "@/lib";
import { Tooltip, TooltipContent, TooltipTrigger } from "../ui/tooltip";

export interface RelativeTimeProps {
  date: Date | number;
}

export function RelativeTime(props: RelativeTimeProps) {
  const { date } = props;
  return (
    <Tooltip>
      <TooltipTrigger>
        <p className='underline decoration-dotted'>{getRelativeTime(date)}</p>
      </TooltipTrigger>
      <TooltipContent side='bottom'>
        <p>{getFormattedTime(date)}</p>
      </TooltipContent>
    </Tooltip>
  );
}
