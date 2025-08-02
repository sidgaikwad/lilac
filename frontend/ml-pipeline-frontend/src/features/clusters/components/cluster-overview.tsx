import { Card } from '@/components/common/card';
import { KeyValueDisplay } from '@/components/common/key-value';
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from '@/components/ui/chart';
import { Label } from '@/components/ui/label';
import { Progress } from '@/components/ui/progress';
import { Separator } from '@/components/ui/separator';
import { ClusterInfo } from '@/types';
import { Label as ChartLabel, PieChart, Pie } from 'recharts';

const NODE_CHART_CONFIG = {
  busy: {
    label: 'Busy nodes',
    color: 'var(--color-red-400)',
  },
  idle: {
    label: 'Idle nodes',
    color: 'var(--color-green-400)',
  },
  empty: {
    label: 'No nodes',
    color: 'var(--color-gray-400)',
  },
} satisfies ChartConfig;

export interface ClusterOverviewProps {
  cluster: ClusterInfo;
}

export function ClusterOverview(props: ClusterOverviewProps) {
  const NODE_DATA = props.cluster.totalNodes != 0 ? [
    { nodeStatus: 'busy', nodeCount: props.cluster.busyNodes, fill: "var(--color-busy)" },
    {
      nodeStatus: 'idle',
      nodeCount: props.cluster.totalNodes - props.cluster.busyNodes,
      fill: "var(--color-idle)" 
    },
  ] : [
    {
      nodeStatus: 'empty',
      nodeCount: 1,
      fill: "var(--color-empty)" 
    },
  ];
  return (
    <Card
      className='w-full'
      title='Overview'
      description={
        <div>
          An overview of <i>{props.cluster.clusterName}</i>
        </div>
      }
      content={
        <div className='flex h-fit space-y-2 space-x-4'>
          <div className='w-fit min-w-xs space-y-2'>
            <Label className='font-semibold'>Node Info</Label>
            <KeyValueDisplay
              items={[
                {
                  key: 'Total Nodes',
                  value: <div>{props.cluster.totalNodes}</div>,
                },
                {
                  key: 'Busy Nodes',
                  value: <div>{props.cluster.busyNodes}</div>,
                },
                {
                  key: 'Idle Nodes',
                  value: (
                    <div>
                      {props.cluster.totalNodes - props.cluster.busyNodes}
                    </div>
                  ),
                },
              ]}
            />
            <ChartContainer
              config={NODE_CHART_CONFIG}
              className='mx-auto aspect-square max-h-[250px]'
            >
              <PieChart>
                {props.cluster.totalNodes != 0 &&
                <ChartTooltip
                  cursor={false}
                  content={<ChartTooltipContent hideLabel />}
                /> }
                <Pie
                  data={NODE_DATA}
                  dataKey='nodeCount'
                  nameKey='nodeStatus'
                  startAngle={90}
                  endAngle={450}
                  innerRadius={60}
                  outerRadius={80}
                  strokeWidth={5}
                >
                  <ChartLabel
                    content={({ viewBox }) => {
                      if (viewBox && 'cx' in viewBox && 'cy' in viewBox) {
                        return (
                          <text
                            x={viewBox.cx}
                            y={viewBox.cy}
                            textAnchor='middle'
                            dominantBaseline='middle'
                          >
                            <tspan
                              x={viewBox.cx}
                              y={viewBox.cy}
                              className='fill-foreground text-3xl font-bold'
                            >
                              {props.cluster.totalNodes.toLocaleString()}
                            </tspan>
                            <tspan
                              x={viewBox.cx}
                              y={(viewBox.cy || 0) + 24}
                              className='fill-gray-text-muted'
                            >
                              Registered
                            </tspan>
                            <tspan
                              x={viewBox.cx}
                              y={(viewBox.cy || 0) + 38}
                              className='fill-gray-text-muted'
                            >
                              Nodes
                            </tspan>
                          </text>
                        );
                      }
                    }}
                  />
                </Pie>
              </PieChart>
            </ChartContainer>
          </div>
          <Separator
            orientation='vertical'
            className='data-[orientation=vertical]:h-80'
          />
          <div className='space-y-4'>
            <div className='space-y-2'>
              <Label className='font-semibold'>CPU Usage</Label>
              <KeyValueDisplay
                items={[
                  {
                    key: 'Total CPU',
                    value: <div>{props.cluster.cpuInfo.totalMillicores}m</div>,
                  },
                  {
                    key: 'Used CPU',
                    value: <div>{props.cluster.cpuInfo.usedMillicores}m</div>,
                  },
                ]}
              />
              <Progress
                color='default'
                value={
                  (props.cluster.cpuInfo.usedMillicores /
                    props.cluster.cpuInfo.totalMillicores) *
                  100
                }
                className='max-w-[200px]'
              />
            </div>
            <div className='space-y-2'>
              <Label className='font-semibold'>Memory Usage</Label>
              <KeyValueDisplay
                items={[
                  {
                    key: 'Total Memory',
                    value: (
                      <div>{props.cluster.memoryInfo.totalMemoryMb} MB</div>
                    ),
                  },
                  {
                    key: 'Used Memory',
                    value: (
                      <div>{props.cluster.memoryInfo.usedMemoryMb} MB</div>
                    ),
                  },
                ]}
              />
              <Progress
                color='default'
                value={
                  (props.cluster.memoryInfo.usedMemoryMb /
                    props.cluster.memoryInfo.totalMemoryMb) *
                  100
                }
                className='max-w-[200px]'
              />
            </div>
            <div className='space-y-2'>
              <Label className='font-semibold'>GPU Usage</Label>
              <KeyValueDisplay
                items={[
                  {
                    key: 'Total GPUs',
                    value: <div>{props.cluster.gpuInfo.totalGpus}</div>,
                  },
                  {
                    key: 'Used GPUs',
                    value: <div>{props.cluster.gpuInfo.usedGpus}</div>,
                  },
                ]}
              />
              <Progress
                color='default'
                value={
                  (props.cluster.gpuInfo.usedGpus /
                    props.cluster.gpuInfo.totalGpus) *
                  100
                }
                className='max-w-[200px]'
              />
            </div>
          </div>
        </div>
      }
    />
  );
}
