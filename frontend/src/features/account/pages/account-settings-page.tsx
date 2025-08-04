import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { useGetAccountDetails } from '@/services';

function AccountSettingsPage() {
  const { data: user } = useGetAccountDetails({});

  return (
    <div className='space-y-6'>
      <Card>
        <CardHeader>
          <CardTitle>Profile Information</CardTitle>
          <CardDescription>
            Update your account's profile information.
          </CardDescription>
        </CardHeader>
        <CardContent className='space-y-4'>
          <div className='space-y-1'>
            <Label htmlFor='username'>Username</Label>
            <Input
              id='username'
              type='text'
              defaultValue={user?.username ?? ''}
              disabled
            />
            <p className='text-gray-text-muted text-xs'>
              Username cannot be changed.
            </p>
          </div>
        </CardContent>
      </Card>

      <Separator />

      <Card>
        <CardHeader>
          <CardTitle>Change Password</CardTitle>
          <CardDescription>Update your password.</CardDescription>
        </CardHeader>
        <CardContent className='space-y-4'>
          <div className='space-y-1'>
            <Label htmlFor='currentPassword'>Current Password</Label>
            <Input id='currentPassword' type='password' />
          </div>
          <div className='space-y-1'>
            <Label htmlFor='newPassword'>New Password</Label>
            <Input id='newPassword' type='password' />
          </div>
          <div className='space-y-1'>
            <Label htmlFor='confirmPassword'>Confirm New Password</Label>
            <Input id='confirmPassword' type='password' />
          </div>
        </CardContent>
        <CardFooter>
          <Button disabled>Update Password</Button>
        </CardFooter>
      </Card>
    </div>
  );
}

export default AccountSettingsPage;
