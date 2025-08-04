import { useLocation, useNavigate } from 'react-router-dom';
import {
  Tabs as TabsComponent,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '../ui/tabs';
import { useEffect } from 'react';

export interface TabItem {
  id: string;
  label?: React.ReactNode;
  content: React.ReactNode;
}

export interface TabsProps {
  defaultTab?: string;
  items: TabItem[];
}

export function Tabs(props: TabsProps) {
  const location = useLocation();
  const navigate = useNavigate();

  const defaultTab =
    (props.defaultTab ?? props.items.length > 0) ? props.items[0].id : '';

  useEffect(() => {
    if (!location.hash && defaultTab) {
      navigate(`#${defaultTab}`, {
        replace: true,
      });
    }
  }, [location.hash, navigate, defaultTab]);

  return (
    <TabsComponent
      onValueChange={(value) =>
        navigate(`#${value}`, {
          replace: true,
        })
      }
      value={location.hash.slice(1)}
      defaultValue={defaultTab}
      className='w-full'
    >
      <TabsList className='max-w-[400px]'>
        {props.items.map((item) => (
          <TabsTrigger key={item.id} value={item.id}>
            {item.label ?? <span className='capitalize'>{item.id}</span>}
          </TabsTrigger>
        ))}
      </TabsList>
      {props.items.map((item) => (
        <TabsContent key={item.id} value={item.id}>
          {item.content}
        </TabsContent>
      ))}
    </TabsComponent>
  );
}
