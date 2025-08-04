import {
  Breadcrumb,
  BreadcrumbEllipsis,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from '../ui/breadcrumb';
import { Link } from 'react-router-dom';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '../ui/dropdown-menu';
import { Fragment } from 'react/jsx-runtime';

interface BreadcrumbProps {
  content: React.ReactNode;
  link?: string;
}

export interface BreadcrumbsProps {
  breadcrumbs: BreadcrumbProps[];
  className?: string;
}

export default function Breadcrumbs({
  breadcrumbs,
  className,
}: BreadcrumbsProps) {
  const getBreadcrubsLink = (breadcrumb: BreadcrumbProps) => {
    return (
      <BreadcrumbLink asChild>
        {breadcrumb.link ? (
          <Link to={breadcrumb.link}>{breadcrumb.content}</Link>
        ) : (
          <div>{breadcrumb.content}</div>
        )}
      </BreadcrumbLink>
    );
  };
  if (breadcrumbs.length <= 1) {
    return undefined;
  }
  if (breadcrumbs.length > 4) {
    return (
      <Breadcrumb className={className}>
        <BreadcrumbList>
          <BreadcrumbItem>
            <BreadcrumbLink asChild>
              {getBreadcrubsLink(breadcrumbs[0])}
            </BreadcrumbLink>
          </BreadcrumbItem>
          <BreadcrumbSeparator />

          <BreadcrumbItem>
            <DropdownMenu>
              <DropdownMenuTrigger className='flex items-center gap-1'>
                <BreadcrumbEllipsis className='h-4 w-4' />
                <span className='sr-only'>Toggle menu</span>
              </DropdownMenuTrigger>
              <DropdownMenuContent align='start'>
                {breadcrumbs.slice(1, -2).map((bc, index) => (
                  <DropdownMenuItem key={index}>
                    {bc.link ? (
                      <Link to={bc.link}>{bc.content}</Link>
                    ) : (
                      <div>{bc.content}</div>
                    )}
                  </DropdownMenuItem>
                ))}
              </DropdownMenuContent>
            </DropdownMenu>
          </BreadcrumbItem>
          <BreadcrumbSeparator />
          <BreadcrumbItem>
            <BreadcrumbLink asChild>
              {getBreadcrubsLink(breadcrumbs[breadcrumbs.length - 2])}
            </BreadcrumbLink>
          </BreadcrumbItem>
          <BreadcrumbSeparator />
          <BreadcrumbItem>
            <BreadcrumbPage>
              {breadcrumbs[breadcrumbs.length - 1].content}
            </BreadcrumbPage>
          </BreadcrumbItem>
        </BreadcrumbList>
      </Breadcrumb>
    );
  }
  return (
    <Breadcrumb>
      <BreadcrumbList>
        {breadcrumbs.slice(0, -1).map((bc, index) => (
          <Fragment key={index}>
            <BreadcrumbItem>
              <BreadcrumbLink asChild>
                {getBreadcrubsLink(bc)}
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
          </Fragment>
        ))}
        <BreadcrumbItem>
          <BreadcrumbPage>
            {breadcrumbs[breadcrumbs.length - 1].content}
          </BreadcrumbPage>
        </BreadcrumbItem>
      </BreadcrumbList>
    </Breadcrumb>
  );
}
