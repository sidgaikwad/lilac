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
  link: string;
}

export interface BreadcrumbsProps {
  breadcrumbs: BreadcrumbProps[];
  className?: string;
}

export default function Breadcrumbs({
  breadcrumbs,
  className,
}: BreadcrumbsProps) {
  if (breadcrumbs.length <= 1) {
    return undefined;
  }
  if (breadcrumbs.length > 4) {
    return (
      <Breadcrumb className={className}>
        <BreadcrumbList>
          <BreadcrumbItem>
            <BreadcrumbLink asChild>
              <Link to={breadcrumbs[0].link}>{breadcrumbs[0].content}</Link>
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
                    <Link to={bc.link}>{bc.content}</Link>
                  </DropdownMenuItem>
                ))}
              </DropdownMenuContent>
            </DropdownMenu>
          </BreadcrumbItem>
          <BreadcrumbSeparator />
          <BreadcrumbItem>
            <BreadcrumbLink asChild>
              <Link to={breadcrumbs[breadcrumbs.length - 2].link}>
                {breadcrumbs[breadcrumbs.length - 2].content}
              </Link>
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
                <Link to={bc.link}>{bc.content}</Link>
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
