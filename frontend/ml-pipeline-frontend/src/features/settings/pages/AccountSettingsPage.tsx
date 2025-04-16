import React from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
// TODO: Import useAuthStore to get user data
// TODO: Import react-hook-form for form handling

const AccountSettingsPage: React.FC = () => {
  // TODO: Fetch current user data from authStore or API (e.g., GET /users/me)
  const currentUser = { name: 'Admin User', email: 'admin@example.com' }; // Placeholder

  // TODO: Implement form handling for profile update
  const handleProfileSave = () => {
    console.log("Saving profile...");
    // TODO: API Call - PUT/PATCH /users/me (or similar) with updated name
  };

  // TODO: Implement form handling for password update
  const handlePasswordUpdate = () => {
    console.log("Updating password...");
    // TODO: API Call - PUT/POST /users/me/password (or similar) with current/new passwords
  };

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Profile Information</CardTitle>
          <CardDescription>Update your account's profile information and email address.</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-1">
            <Label htmlFor="name">Name</Label>
            <Input id="name" defaultValue={currentUser.name} />
          </div>
          <div className="space-y-1">
            <Label htmlFor="email">Email</Label>
            <Input id="email" type="email" defaultValue={currentUser.email} disabled />
            <p className="text-xs text-muted-foreground">Email address cannot be changed.</p>
          </div>
        </CardContent>
        <CardFooter>
          <Button onClick={handleProfileSave} disabled>Save Profile</Button>
        </CardFooter>
      </Card>

      <Separator />

      <Card>
        <CardHeader>
          <CardTitle>Change Password</CardTitle>
          <CardDescription>Update your password. Choose a strong one!</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-1">
            <Label htmlFor="currentPassword">Current Password</Label>
            <Input id="currentPassword" type="password" />
          </div>
          <div className="space-y-1">
            <Label htmlFor="newPassword">New Password</Label>
            <Input id="newPassword" type="password" />
          </div>
           <div className="space-y-1">
            <Label htmlFor="confirmPassword">Confirm New Password</Label>
            <Input id="confirmPassword" type="password" />
          </div>
        </CardContent>
        <CardFooter>
          <Button onClick={handlePasswordUpdate} disabled>Update Password</Button>
        </CardFooter>
      </Card>
    </div>
  );
};

export default AccountSettingsPage;