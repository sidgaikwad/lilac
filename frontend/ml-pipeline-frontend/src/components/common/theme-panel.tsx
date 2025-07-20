import {
  CheckCircle2Icon,
  PopcornIcon,
  AlertCircleIcon,
  BadgeCheckIcon,
  GitBranch,
  Loader2,
  Calculator,
  CreditCard,
  Settings,
  Smile,
  User,
  Settings2,
  CircleHelp,
  CircleDollarSign,
} from 'lucide-react';
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '../ui/accordion';
import { Alert, AlertTitle, AlertDescription } from '../ui/alert';
import { Button } from '../ui/button';
import { ThemeToggle } from './theme-toggle';
import {
  AlertDialog,
  AlertDialogTrigger,
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogCancel,
  AlertDialogAction,
} from '../ui/alert-dialog';
import { AspectRatio } from '../ui/aspect-ratio';
import { Avatar, AvatarImage, AvatarFallback } from '../ui/avatar';
import { Badge } from '../ui/badge';
import {
  Breadcrumb,
  BreadcrumbList,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbSeparator,
  BreadcrumbEllipsis,
  BreadcrumbPage,
} from '../ui/breadcrumb';
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
} from '../ui/dropdown-menu';
import { Calendar } from '../ui/calendar';
import React from 'react';
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '../ui/card';
import { Input } from '../ui/input';
import { Label } from '../ui/label';
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselPrevious,
  CarouselNext,
} from '../ui/carousel';
import { Bar, BarChart } from 'recharts';
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from '../ui/chart';
import { Checkbox } from '../ui/checkbox';
import {
  Collapsible,
  CollapsibleTrigger,
  CollapsibleContent,
} from '../ui/collapsible';
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
  CommandShortcut,
} from '../ui/command';
import {
  ContextMenu,
  ContextMenuTrigger,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuShortcut,
  ContextMenuSub,
  ContextMenuSubTrigger,
  ContextMenuSubContent,
  ContextMenuSeparator,
  ContextMenuCheckboxItem,
  ContextMenuRadioGroup,
  ContextMenuLabel,
  ContextMenuRadioItem,
} from '../ui/context-menu';
import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
  DialogClose,
} from '../ui/dialog';
import {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from '../ui/drawer';
import {
  HoverCard,
  HoverCardTrigger,
  HoverCardContent,
} from '../ui/hover-card';
import {
  InputOTP,
  InputOTPGroup,
  InputOTPSlot,
  InputOTPSeparator,
} from '../ui/input-otp';
import {
  Menubar,
  MenubarMenu,
  MenubarTrigger,
  MenubarContent,
  MenubarItem,
  MenubarShortcut,
  MenubarSeparator,
} from '../ui/menubar';
import {
  NavigationMenu,
  NavigationMenuList,
  NavigationMenuItem,
  NavigationMenuTrigger,
  NavigationMenuContent,
  NavigationMenuLink,
} from '../ui/navigation-menu';
import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationPrevious,
  PaginationLink,
  PaginationEllipsis,
  PaginationNext,
} from '../ui/pagination';
import { Popover, PopoverTrigger, PopoverContent } from '../ui/popover';
import { Progress } from '../ui/progress';
import { RadioGroup, RadioGroupItem } from '../ui/radio-group';
import {
  ResizablePanelGroup,
  ResizablePanel,
  ResizableHandle,
} from '../ui/resizable';
import { ScrollArea } from '../ui/scroll-area';
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from '../ui/select';
import { Separator } from '../ui/separator';
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from '../ui/sheet';
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarTrigger,
} from '../ui/sidebar';
import { Skeleton } from '../ui/skeleton';
import { Slider } from '../ui/slider';
import { Toaster } from '../ui/sonner';
import { Switch } from '../ui/switch';
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableFooter,
  TableHead,
  TableHeader,
  TableRow,
} from '../ui/table';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '../ui/tabs';
import { Textarea } from '../ui/textarea';
import { Toggle } from '../ui/toggle';
import { ToggleGroup, ToggleGroupItem } from '../ui/toggle-group';
import { Tooltip, TooltipContent, TooltipTrigger } from '../ui/tooltip';
import { DateRange } from 'react-day-picker';
import {
  Container,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '../ui/container';
import { toast } from '../toast';
import { random } from 'lodash';

export default function ThemePanel() {
  const [date, setDate] = React.useState<DateRange | undefined>();
  const data = [
    { month: 'January', desktop: 186, mobile: 80 },
    { month: 'February', desktop: 305, mobile: 200 },
    { month: 'March', desktop: 237, mobile: 120 },
    { month: 'April', desktop: 73, mobile: 190 },
    { month: 'May', desktop: 209, mobile: 130 },
    { month: 'June', desktop: 214, mobile: 140 },
  ];

  const chartConfig = {
    desktop: {
      label: 'Desktop',
      color: '#2563eb',
    },
    mobile: {
      label: 'Mobile',
      color: '#60a5fa',
    },
  } satisfies ChartConfig;
  return (
    <div className='flex h-screen w-screen flex-col items-center space-y-8 overflow-scroll scroll-auto'>
      <span className='border-accent bg-background absolute z-9999 rounded-lg border p-2'>
        <ThemeToggle /> Toggle Theme
      </span>
      <div className='space-y-2'>
        <p className='text-center'>Accordion</p>
        <div className='flex flex-row space-x-4'>
          <Accordion type='single' collapsible>
            <AccordionItem value='item-1'>
              <AccordionTrigger>Lorem ipsum dolor sit amet?</AccordionTrigger>
              <AccordionContent>
                Aenean luctus rutrum auctor. Cras consequat vehicula felis, nec
                gravida urna eleifend vel.
              </AccordionContent>
            </AccordionItem>
          </Accordion>
          <Accordion type='multiple'>
            <AccordionItem value='item-1'>
              <AccordionTrigger>Lorem ipsum dolor sit amet?</AccordionTrigger>
              <AccordionContent>
                Aenean luctus rutrum auctor. Cras consequat vehicula felis, nec
                gravida urna eleifend vel.
              </AccordionContent>
            </AccordionItem>
            <AccordionItem value='item-2'>
              <AccordionTrigger>Lorem ipsum dolor sit amet?</AccordionTrigger>
              <AccordionContent>
                Aenean luctus rutrum auctor. Cras consequat vehicula felis, nec
                gravida urna eleifend vel.
              </AccordionContent>
            </AccordionItem>
          </Accordion>
        </div>
      </div>

      <div className='space-y-2'>
        <p className='text-center'>Alert</p>
        <div className='flex flex-row space-x-4'>
          <Alert variant='success'>
            <CheckCircle2Icon />
            <AlertTitle>Success! Your changes have been saved</AlertTitle>
            <AlertDescription>
              This is an alert with icon, title and description.
            </AlertDescription>
          </Alert>
          <Alert variant='warn'>
            <CheckCircle2Icon />
            <AlertTitle>Warning! Pay attention</AlertTitle>
            <AlertDescription>
              This is an alert warning the user of something.
            </AlertDescription>
          </Alert>
          <Alert variant='info'>
            <CheckCircle2Icon />
            <AlertTitle>Here's some info!</AlertTitle>
            <AlertDescription>
              This is an alert with information.
            </AlertDescription>
          </Alert>
        </div>
        <div className='flex flex-row space-x-4'>
          <Alert>
            <PopcornIcon />
            <AlertTitle>
              This Alert has a title and an icon. No description.
            </AlertTitle>
          </Alert>
          <Alert variant='destructive'>
            <AlertCircleIcon />
            <AlertTitle>Unable to process your payment.</AlertTitle>
            <AlertDescription>
              <p>Please verify your billing information and try again.</p>
              <ul className='list-inside list-disc text-sm'>
                <li>Check your card details</li>
                <li>Ensure sufficient funds</li>
                <li>Verify billing address</li>
              </ul>
            </AlertDescription>
          </Alert>
          <Alert variant='loading'>
            <Loader2 className='animate-spin' />
            <AlertTitle>Loading</AlertTitle>
          </Alert>
        </div>
        <div className='flex flex-row space-x-4'>
          <Alert variant='help'>
            <CircleHelp />
            <AlertTitle>Here's some help info</AlertTitle>
          </Alert>
        </div>
      </div>

      <div className='space-y-2'>
        <p className='text-center'>Alert Dialog</p>
        <div className='flex flex-row space-x-4'>
          <AlertDialog>
            <AlertDialogTrigger>Open</AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
                <AlertDialogDescription>
                  This action cannot be undone. This will permanently delete
                  your account and remove your data from our servers.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction>Continue</AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </div>
      </div>

      <div className='space-y-2'>
        <p className='text-center'>Aspect Ratio</p>
        <div className='flex flex-row space-x-4'>
          <AspectRatio ratio={16 / 9} className='bg-muted rounded-lg'>
            <img
              src='https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=800&dpr=2&q=80'
              alt='Photo by Drew Beamer'
              className='h-full w-full rounded-lg object-cover dark:brightness-[0.2] dark:grayscale'
            />
          </AspectRatio>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Avatar</p>
        <div className='flex flex-row space-x-4'>
          <Avatar>
            <AvatarImage src='https://github.com/shadcn.png' alt='@shadcn' />
            <AvatarFallback>CN</AvatarFallback>
          </Avatar>
          <Avatar className='rounded-lg'>
            <AvatarImage
              src='https://github.com/evilrabbit.png'
              alt='@evilrabbit'
            />
            <AvatarFallback>ER</AvatarFallback>
          </Avatar>
          <div className='*:data-[slot=avatar]:ring-background flex -space-x-2 *:data-[slot=avatar]:ring-2 *:data-[slot=avatar]:grayscale'>
            <Avatar>
              <AvatarImage src='https://github.com/shadcn.png' alt='@shadcn' />
              <AvatarFallback>CN</AvatarFallback>
            </Avatar>
            <Avatar>
              <AvatarImage src='https://github.com/leerob.png' alt='@leerob' />
              <AvatarFallback>LR</AvatarFallback>
            </Avatar>
            <Avatar>
              <AvatarImage
                src='https://github.com/evilrabbit.png'
                alt='@evilrabbit'
              />
              <AvatarFallback>ER</AvatarFallback>
            </Avatar>
          </div>
          <div>
            <Avatar>
              <AvatarImage src='' alt='' />
              <AvatarFallback>FB</AvatarFallback>
            </Avatar>
          </div>
        </div>
      </div>

      <div className='space-y-2'>
        <p className='text-center'>Badge</p>
        <div className='flex flex-row space-x-4'>
          <div className='flex flex-col items-center gap-2'>
            <div className='flex w-full flex-wrap gap-2'>
              <Badge asChild>
                <a>Badge</a>
              </Badge>
              <Badge asChild variant='secondary'>
                <a>Secondary</a>
              </Badge>
              <Badge asChild variant='destructive'>
                <a>Destructive</a>
              </Badge>
              <Badge asChild variant='outline'>
                <a>Outline</a>
              </Badge>
            </div>
            <div className='flex w-full flex-wrap gap-2'>
              <Badge
                variant='secondary'
                className='bg-blue-500 text-white dark:bg-blue-600'
              >
                <BadgeCheckIcon />
                Verified
              </Badge>
              <Badge className='h-5 min-w-5 rounded-full px-1 font-mono tabular-nums'>
                8
              </Badge>
              <Badge
                className='h-5 min-w-5 rounded-full px-1 font-mono tabular-nums'
                variant='destructive'
              >
                99
              </Badge>
              <Badge
                className='h-5 min-w-5 rounded-full px-1 font-mono tabular-nums'
                variant='outline'
              >
                20+
              </Badge>
            </div>
          </div>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Breadcrumb</p>
        <div className='flex flex-row space-x-4'>
          <Breadcrumb>
            <BreadcrumbList>
              <BreadcrumbItem>
                <BreadcrumbLink asChild>
                  <a href='/'>Home</a>
                </BreadcrumbLink>
              </BreadcrumbItem>
              <BreadcrumbSeparator />
              <BreadcrumbItem>
                <DropdownMenu>
                  <DropdownMenuTrigger className='flex items-center gap-1'>
                    <BreadcrumbEllipsis className='size-4' />
                    <span className='sr-only'>Toggle menu</span>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent align='start'>
                    <DropdownMenuItem>Documentation</DropdownMenuItem>
                    <DropdownMenuItem>Themes</DropdownMenuItem>
                    <DropdownMenuItem>GitHub</DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </BreadcrumbItem>
              <BreadcrumbSeparator />
              <BreadcrumbItem>
                <BreadcrumbLink asChild>
                  <a href='/docs/components'>Components</a>
                </BreadcrumbLink>
              </BreadcrumbItem>
              <BreadcrumbSeparator />
              <BreadcrumbItem>
                <BreadcrumbPage>Breadcrumb</BreadcrumbPage>
              </BreadcrumbItem>
            </BreadcrumbList>
          </Breadcrumb>
        </div>
      </div>

      <div className='space-y-2'>
        <p className='text-center'>Button</p>
        <div className='flex flex-row space-x-4'>
          <Button>Default</Button>
          <Button variant='secondary'>Secondary</Button>
          <Button variant='outline'>Outline</Button>
          <Button variant='ghost'>Ghost</Button>
          <Button variant='destructive'>Destructive</Button>
          <Button variant='link'>Link</Button>
          <Button variant='outline' size='sm'>
            <GitBranch /> New Branch
          </Button>
          <Button size='sm' disabled>
            <Loader2 className='animate-spin' />
            Please wait
          </Button>
        </div>
      </div>

      <div className='space-y-2'>
        <p className='text-center'>Calendar</p>
        <div className='flex flex-row space-x-4'>
          <Calendar
            mode='range'
            selected={date}
            onSelect={setDate}
            captionLayout='dropdown'
          />
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Card</p>
        <div className='flex flex-row space-x-4'>
          <Card className='w-full max-w-sm'>
            <CardHeader>
              <CardTitle>Login to your account</CardTitle>
              <CardDescription>
                Enter your email below to login to your account
              </CardDescription>
              <CardAction>
                <Button variant='link'>Sign Up</Button>
              </CardAction>
            </CardHeader>
            <CardContent>
              <form>
                <div className='flex flex-col gap-6'>
                  <div className='grid gap-2'>
                    <Label htmlFor='email'>Email</Label>
                    <Input
                      id='email'
                      type='email'
                      placeholder='m@example.com'
                      required
                    />
                  </div>
                  <div className='grid gap-2'>
                    <div className='flex items-center'>
                      <Label htmlFor='password'>Password</Label>
                      <a
                        href='#'
                        className='ml-auto inline-block text-sm underline-offset-4 hover:underline'
                      >
                        Forgot your password?
                      </a>
                    </div>
                    <Input id='password' type='password' required />
                  </div>
                </div>
              </form>
            </CardContent>
            <CardFooter className='flex-col gap-2'>
              <Button type='submit' className='w-full'>
                Login
              </Button>
              <Button variant='outline' className='w-full'>
                Login with Google
              </Button>
            </CardFooter>
          </Card>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Carousel</p>
        <div className='flex flex-row space-x-4'>
          <Carousel className='w-full max-w-xs'>
            <CarouselContent>
              {Array.from({ length: 5 }).map((_, index) => (
                <CarouselItem key={index}>
                  <div className='p-1'>
                    <Card>
                      <CardContent className='flex aspect-square items-center justify-center p-6'>
                        <span className='text-4xl font-semibold'>
                          {index + 1}
                        </span>
                      </CardContent>
                    </Card>
                  </div>
                </CarouselItem>
              ))}
            </CarouselContent>
            <CarouselPrevious />
            <CarouselNext />
          </Carousel>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Chart</p>
        <div className='flex flex-row space-x-4'>
          <ChartContainer config={chartConfig} className='min-h-[200px] w-full'>
            <BarChart data={data}>
              <ChartTooltip content={<ChartTooltipContent />} />
              <Bar dataKey='desktop' fill='var(--color-chart-1)' radius={4} />
              <Bar dataKey='mobile' fill='var(--color-chart-2)' radius={4} />
            </BarChart>
          </ChartContainer>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Checkbox</p>
        <div className='flex flex-row space-x-4'>
          <Checkbox> Checkbox </Checkbox>
          <Checkbox className='size-6'> Checkbox </Checkbox>
          <Checkbox className='size-8'> Checkbox </Checkbox>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Collapsible</p>
        <div className='flex flex-row space-x-4'>
          <Collapsible>
            <CollapsibleTrigger>
              Can I use this in my project?
            </CollapsibleTrigger>
            <CollapsibleContent>
              Yes. Free to use for personal and commercial projects. No
              attribution required.
            </CollapsibleContent>
          </Collapsible>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Command</p>
        <div className='flex flex-row space-x-4'>
          <Command className='rounded-lg border shadow-md md:min-w-[450px]'>
            <CommandInput placeholder='Type a command or search...' />
            <CommandList>
              <CommandEmpty>No results found.</CommandEmpty>
              <CommandGroup heading='Suggestions'>
                <CommandItem>
                  <Calendar />
                  <span>Calendar</span>
                </CommandItem>
                <CommandItem>
                  <Smile />
                  <span>Search Emoji</span>
                </CommandItem>
                <CommandItem disabled>
                  <Calculator />
                  <span>Calculator</span>
                </CommandItem>
              </CommandGroup>
              <CommandSeparator />
              <CommandGroup heading='Settings'>
                <CommandItem>
                  <User />
                  <span>Profile</span>
                  <CommandShortcut>⌘P</CommandShortcut>
                </CommandItem>
                <CommandItem>
                  <CreditCard />
                  <span>Billing</span>
                  <CommandShortcut>⌘B</CommandShortcut>
                </CommandItem>
                <CommandItem>
                  <Settings />
                  <span>Settings</span>
                  <CommandShortcut>⌘S</CommandShortcut>
                </CommandItem>
              </CommandGroup>
            </CommandList>
          </Command>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>ContextMenu</p>
        <div className='flex flex-row space-x-4'>
          <Container>
            <ContainerHeader>
              <ContainerTitle>Container Title</ContainerTitle>
              <ContainerDescription>Container description</ContainerDescription>
            </ContainerHeader>
            <ContainerContent>Container content</ContainerContent>
          </Container>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>ContextMenu</p>
        <div className='flex flex-row space-x-4'>
          <ContextMenu>
            <ContextMenuTrigger className='flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm'>
              Right click here
            </ContextMenuTrigger>
            <ContextMenuContent className='w-52'>
              <ContextMenuItem inset>
                Back
                <ContextMenuShortcut>⌘[</ContextMenuShortcut>
              </ContextMenuItem>
              <ContextMenuItem inset disabled>
                Forward
                <ContextMenuShortcut>⌘]</ContextMenuShortcut>
              </ContextMenuItem>
              <ContextMenuItem inset>
                Reload
                <ContextMenuShortcut>⌘R</ContextMenuShortcut>
              </ContextMenuItem>
              <ContextMenuSub>
                <ContextMenuSubTrigger inset>More Tools</ContextMenuSubTrigger>
                <ContextMenuSubContent className='w-44'>
                  <ContextMenuItem>Save Page...</ContextMenuItem>
                  <ContextMenuItem>Create Shortcut...</ContextMenuItem>
                  <ContextMenuItem>Name Window...</ContextMenuItem>
                  <ContextMenuSeparator />
                  <ContextMenuItem>Developer Tools</ContextMenuItem>
                  <ContextMenuSeparator />
                  <ContextMenuItem variant='destructive'>
                    Delete
                  </ContextMenuItem>
                </ContextMenuSubContent>
              </ContextMenuSub>
              <ContextMenuSeparator />
              <ContextMenuCheckboxItem checked>
                Show Bookmarks
              </ContextMenuCheckboxItem>
              <ContextMenuCheckboxItem>Show Full URLs</ContextMenuCheckboxItem>
              <ContextMenuSeparator />
              <ContextMenuRadioGroup value='pedro'>
                <ContextMenuLabel inset>People</ContextMenuLabel>
                <ContextMenuRadioItem value='pedro'>
                  Pedro Duarte
                </ContextMenuRadioItem>
                <ContextMenuRadioItem value='colm'>
                  Colm Tuite
                </ContextMenuRadioItem>
              </ContextMenuRadioGroup>
            </ContextMenuContent>
          </ContextMenu>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Dialog</p>
        <div className='flex flex-row space-x-4'>
          <Dialog>
            <DialogTrigger asChild>
              <Button variant='outline'>Share</Button>
            </DialogTrigger>
            <DialogContent className='sm:max-w-md'>
              <DialogHeader>
                <DialogTitle>Share link</DialogTitle>
                <DialogDescription>
                  Anyone who has this link will be able to view this.
                </DialogDescription>
              </DialogHeader>
              <div className='flex items-center gap-2'>
                <div className='grid flex-1 gap-2'>
                  <Label htmlFor='link' className='sr-only'>
                    Link
                  </Label>
                  <Input
                    id='link'
                    defaultValue='https://ui.shadcn.com/docs/installation'
                    readOnly
                  />
                </div>
              </div>
              <DialogFooter className='sm:justify-start'>
                <DialogClose asChild>
                  <Button type='button' variant='secondary'>
                    Close
                  </Button>
                </DialogClose>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Drawer</p>
        <div className='flex flex-row space-x-4'>
          <Drawer>
            <DrawerTrigger>Open</DrawerTrigger>
            <DrawerContent>
              <DrawerHeader>
                <DrawerTitle>Are you absolutely sure?</DrawerTitle>
                <DrawerDescription>
                  This action cannot be undone.
                </DrawerDescription>
              </DrawerHeader>
              <DrawerFooter>
                <Button>Submit</Button>
                <DrawerClose>
                  <Button variant='outline'>Cancel</Button>
                </DrawerClose>
              </DrawerFooter>
            </DrawerContent>
          </Drawer>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Dropdown Menu</p>
        <div className='flex flex-row space-x-4'>
          <DropdownMenu>
            <DropdownMenuTrigger>Open</DropdownMenuTrigger>
            <DropdownMenuContent>
              <DropdownMenuLabel>My Account</DropdownMenuLabel>
              <DropdownMenuSeparator />
              <DropdownMenuItem>Profile</DropdownMenuItem>
              <DropdownMenuItem>Billing</DropdownMenuItem>
              <DropdownMenuItem>Team</DropdownMenuItem>
              <DropdownMenuItem>Subscription</DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Hover Card</p>
        <div className='flex flex-row space-x-4'>
          <HoverCard>
            <HoverCardTrigger>Hover</HoverCardTrigger>
            <HoverCardContent>
              The React Framework – created and maintained by @vercel.
            </HoverCardContent>
          </HoverCard>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Input</p>
        <div className='flex flex-row space-x-4'>
          <Input />
          <Input disabled placeholder='disabled' />
          <Input placeholder='placeholder' />
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Input OTP</p>
        <div className='flex flex-row space-x-4'>
          <InputOTP maxLength={6}>
            <InputOTPGroup>
              <InputOTPSlot index={0} />
              <InputOTPSlot index={1} />
              <InputOTPSlot index={2} />
            </InputOTPGroup>
            <InputOTPSeparator />
            <InputOTPGroup>
              <InputOTPSlot index={3} />
              <InputOTPSlot index={4} />
              <InputOTPSlot index={5} />
            </InputOTPGroup>
          </InputOTP>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Label</p>
        <div className='flex flex-row space-x-4'>
          <div>
            <Label htmlFor='email'>Your email address</Label>
            <Input placeholder='email' />
          </div>
          <div>
            <Label htmlFor='password'>Password</Label>
          </div>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Menubar</p>
        <div className='flex flex-row space-x-4'>
          <Menubar>
            <MenubarMenu>
              <MenubarTrigger>File</MenubarTrigger>
              <MenubarContent>
                <MenubarItem>
                  New Tab <MenubarShortcut>⌘T</MenubarShortcut>
                </MenubarItem>
                <MenubarItem>New Window</MenubarItem>
                <MenubarSeparator />
                <MenubarItem>Share</MenubarItem>
                <MenubarSeparator />
                <MenubarItem>Print</MenubarItem>
              </MenubarContent>
            </MenubarMenu>
          </Menubar>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>NavigationMenu</p>
        <div className='flex flex-row space-x-4'>
          <NavigationMenu>
            <NavigationMenuList>
              <NavigationMenuItem>
                <NavigationMenuTrigger>Item One</NavigationMenuTrigger>
                <NavigationMenuContent>
                  <NavigationMenuLink>Link</NavigationMenuLink>
                </NavigationMenuContent>
              </NavigationMenuItem>
            </NavigationMenuList>
          </NavigationMenu>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Pagination</p>
        <div className='flex flex-row space-x-4'>
          <Pagination>
            <PaginationContent>
              <PaginationItem>
                <PaginationPrevious href='#' />
              </PaginationItem>
              <PaginationItem>
                <PaginationLink isActive href='#'>
                  1
                </PaginationLink>
              </PaginationItem>
              <PaginationItem>
                <PaginationLink href='#'>2</PaginationLink>
              </PaginationItem>
              <PaginationItem>
                <PaginationEllipsis />
              </PaginationItem>
              <PaginationItem>
                <PaginationNext href='#' />
              </PaginationItem>
            </PaginationContent>
          </Pagination>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Popover</p>
        <div className='flex flex-row space-x-4'>
          <Popover>
            <PopoverTrigger>Open</PopoverTrigger>
            <PopoverContent>Place content for the popover here.</PopoverContent>
          </Popover>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Progress</p>
        <div className='flex flex-row space-x-4'>
          <Progress value={33} />
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Radio Group</p>
        <div className='flex flex-row space-x-4'>
          <RadioGroup defaultValue='option-one'>
            <div className='flex items-center space-x-2'>
              <RadioGroupItem value='option-one' id='option-one' />
              <Label htmlFor='option-one'>Option One</Label>
            </div>
            <div className='flex items-center space-x-2'>
              <RadioGroupItem value='option-two' id='option-two' />
              <Label htmlFor='option-two'>Option Two</Label>
            </div>
          </RadioGroup>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Resizable</p>
        <div className='flex flex-row space-x-4'>
          <ResizablePanelGroup direction='horizontal'>
            <ResizablePanel>One</ResizablePanel>
            <ResizableHandle />
            <ResizablePanel>Two</ResizablePanel>
          </ResizablePanelGroup>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Scroll Area</p>
        <div className='flex flex-row space-x-4'>
          <ScrollArea className='border-gray-border-subtle h-[200px] w-[350px] rounded-md border p-4'>
            Jokester began sneaking into the castle in the middle of the night
            and leaving jokes all over the place: under the king's pillow, in
            his soup, even in the royal toilet. The king was furious, but he
            couldn't seem to stop Jokester. And then, one day, the people of the
            kingdom discovered that the jokes left by Jokester were so funny
            that they couldn't help but laugh. And once they started laughing,
            they couldn't stop.
          </ScrollArea>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Select</p>
        <div className='flex flex-row space-x-4'>
          <Select>
            <SelectTrigger className='w-[180px]'>
              <SelectValue placeholder='Theme' />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value='light'>Light</SelectItem>
              <SelectItem value='dark'>Dark</SelectItem>
              <SelectItem value='system'>System</SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Separator</p>
        <div className='flex flex-row space-x-4'>
          <Separator />
          <Separator orientation='vertical' />
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Sheet</p>
        <div className='flex flex-row space-x-4'>
          <Sheet>
            <SheetTrigger>Open</SheetTrigger>
            <SheetContent>
              <SheetHeader>
                <SheetTitle>Are you absolutely sure?</SheetTitle>
                <SheetDescription>
                  This action cannot be undone. This will permanently delete
                  your account and remove your data from our servers.
                </SheetDescription>
              </SheetHeader>
            </SheetContent>
          </Sheet>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Sidebar</p>
        <div className='flex flex-row space-x-4'>
          <SidebarProvider>
            <Sidebar collapsible='icon'>
              <SidebarHeader>
                <SidebarTrigger />
              </SidebarHeader>
              <SidebarContent>
                <SidebarGroup>
                  <SidebarGroupLabel>Application</SidebarGroupLabel>
                  <SidebarGroupContent>
                    <SidebarMenu>
                      <SidebarMenuItem>
                        <SidebarMenuButton>Submenu</SidebarMenuButton>
                      </SidebarMenuItem>
                    </SidebarMenu>
                  </SidebarGroupContent>
                </SidebarGroup>
                <SidebarGroup>
                  <SidebarGroupLabel>Account</SidebarGroupLabel>
                  <SidebarGroupContent>
                    <SidebarMenu>
                      <SidebarMenuItem>
                        <SidebarMenuButton isActive asChild>
                          <a href='/'>
                            <Settings2 />
                            <span>Settings</span>
                          </a>
                        </SidebarMenuButton>
                      </SidebarMenuItem>
                    </SidebarMenu>
                  </SidebarGroupContent>
                </SidebarGroup>
              </SidebarContent>
              <SidebarFooter />
            </Sidebar>
          </SidebarProvider>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Skeleton</p>
        <div className='flex flex-row space-x-4'>
          <div className='flex flex-col space-y-3'>
            <Skeleton className='h-[125px] w-[250px] rounded-xl' />
            <div className='space-y-2'>
              <Skeleton className='h-4 w-[250px]' />
              <Skeleton className='h-4 w-[200px]' />
            </div>
          </div>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Slider</p>
        <div className='flex flex-row space-x-4'>
          <Slider defaultValue={[33]} max={100} step={1} />
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Buttons</p>
        <div className='flex flex-row space-x-4'>
          <Button
            variant='outline'
            onClick={() =>
              toast('Event has been created', {
                icon: <CircleDollarSign />,
                description: 'Sunday, December 03, 2023 at 9:00 AM',
                action: {
                  label: 'Undo',
                  onClick: () => console.log('Undo'),
                },
              })
            }
          >
            Show Toast
          </Button>
          <Button
            variant='outline'
            onClick={() =>
              toast.success('Event has been created', {
                description: 'Sunday, December 03, 2023 at 9:00 AM',
                action: {
                  label: 'Undo',
                  onClick: () => console.log('Undo'),
                },
              })
            }
          >
            Show Success
          </Button>
          <Button
            variant='outline'
            onClick={() =>
              toast.error('Event has been created', {
                description: 'Sunday, December 03, 2023 at 9:00 AM',
                action: {
                  label: 'Undo',
                  onClick: () => console.log('Undo'),
                },
              })
            }
          >
            Show Error
          </Button>
          <Button
            variant='outline'
            onClick={() =>
              toast.info('Event has been created', {
                description: 'Sunday, December 03, 2023 at 9:00 AM',
                action: {
                  label: 'Undo',
                  onClick: () => console.log('Undo'),
                },
              })
            }
          >
            Show Info
          </Button>
          <Button
            variant='outline'
            onClick={() =>
              toast.warning('Event has been created', {
                description: 'Sunday, December 03, 2023 at 9:00 AM',
                action: {
                  label: 'Undo',
                  onClick: () => console.log('Undo'),
                },
              })
            }
          >
            Show Warning
          </Button>
          <Button
            variant='outline'
            onClick={() =>
              toast.loading('Event has been created', {
                description: 'Sunday, December 03, 2023 at 9:00 AM',
                action: {
                  label: 'Undo',
                  onClick: () => console.log('Undo'),
                },
              })
            }
          >
            Show Loading
          </Button>
          <Button
            variant='outline'
            onClick={() => {
              const myPromise = new Promise<{ name: string }>(
                (resolve, reject) => {
                  setTimeout(() => {
                    const val = random(10);
                    if (val < 5) {
                      resolve({ name: 'My toast' });
                    } else {
                      reject(new Error('test'));
                    }
                  }, 2000);
                }
              );

              toast.promise(myPromise, {
                loading: 'Loading...',
                success: (data: { name: string }) => {
                  return {
                    title: `${data.name} toast has been added`,
                  };
                },
                error: (err) => ({ title: 'Error', description: err.message }),
              });
            }}
          >
            Show Promise
          </Button>
          <Button
            variant='outline'
            onClick={() =>
              toast.message('Event has been created', {
                description: 'Sunday, December 03, 2023 at 9:00 AM',
                action: {
                  label: 'Undo',
                  onClick: () => console.log('Undo'),
                },
              })
            }
          >
            Show Message
          </Button>
          <Toaster />
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Switch</p>
        <div className='flex flex-row space-x-4'>
          <Switch />
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Table</p>
        <div className='flex flex-row space-x-4'>
          <Table>
            <TableCaption>A list of your recent invoices.</TableCaption>
            <TableHeader>
              <TableRow>
                <TableHead className='w-[100px]'>Invoice</TableHead>
                <TableHead>Status</TableHead>
                <TableHead>Method</TableHead>
                <TableHead className='text-right'>Amount</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow>
                <TableCell className='font-medium'>INV001</TableCell>
                <TableCell>Paid</TableCell>
                <TableCell>Credit Card</TableCell>
                <TableCell className='text-right'>$250.00</TableCell>
              </TableRow>
              <TableRow>
                <TableCell className='font-medium'>INV002</TableCell>
                <TableCell>Unpaid</TableCell>
                <TableCell>Credit Card</TableCell>
                <TableCell className='text-right'>$150.00</TableCell>
              </TableRow>
            </TableBody>
            <TableFooter>
              <TableRow>
                <TableCell colSpan={3}>Total</TableCell>
                <TableCell className='text-right'>$2,500.00</TableCell>
              </TableRow>
            </TableFooter>
          </Table>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Tabs</p>
        <div className='flex flex-row space-x-4'>
          <Tabs defaultValue='account' className='w-[400px]'>
            <TabsList>
              <TabsTrigger value='account'>Account</TabsTrigger>
              <TabsTrigger value='password'>Password</TabsTrigger>
            </TabsList>
            <TabsContent value='account'>
              Make changes to your account here.
            </TabsContent>
            <TabsContent value='password'>
              Change your password here.
            </TabsContent>
          </Tabs>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Textarea</p>
        <div className='flex flex-row space-x-4'>
          <Textarea placeholder='Type your message here.' />
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Toggle</p>
        <div className='flex flex-row space-x-4'>
          <Toggle>Toggle</Toggle>
          <Toggle variant='outline'>Outline</Toggle>
          <Toggle disabled>Disabled</Toggle>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>ToggleGroup</p>
        <div className='flex flex-row space-x-4'>
          <ToggleGroup type='single'>
            <ToggleGroupItem value='a'>A</ToggleGroupItem>
            <ToggleGroupItem value='b'>B</ToggleGroupItem>
            <ToggleGroupItem value='c'>C</ToggleGroupItem>
          </ToggleGroup>
          <ToggleGroup type='single' variant='outline'>
            <ToggleGroupItem value='a'>A</ToggleGroupItem>
            <ToggleGroupItem value='b'>B</ToggleGroupItem>
            <ToggleGroupItem value='c'>C</ToggleGroupItem>
          </ToggleGroup>
        </div>
      </div>
      <div className='space-y-2'>
        <p className='text-center'>Tooltip</p>
        <div className='flex flex-row space-x-4'>
          <Tooltip>
            <TooltipTrigger>Hover</TooltipTrigger>
            <TooltipContent>
              <p>Add to library</p>
            </TooltipContent>
          </Tooltip>
        </div>
      </div>
    </div>
  );
}
