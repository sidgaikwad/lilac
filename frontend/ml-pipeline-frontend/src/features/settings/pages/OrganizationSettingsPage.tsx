import React from 'react';
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
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
// TODO: Import state management for org data (e.g., Zustand store or TanStack Query)

const OrganizationSettingsPage: React.FC = () => {
  // TODO: Fetch current organization details (e.g., GET /organization/{current_org_id})
  const currentOrg = { id: 'org-123', name: 'Default Org' }; // Placeholder

  // TODO: Fetch members list (e.g., GET /organization/{current_org_id}/members)
  const members = [
    {
      id: 'user-admin',
      name: 'Admin User',
      email: 'admin@example.com',
      role: 'Admin',
    },
    {
      id: 'user-456',
      name: 'Jane Doe',
      email: 'jane@example.com',
      role: 'Member',
    },
  ]; // Placeholder

  // TODO: Implement form handling and API calls for rename, invite, manage roles, remove member

  const handleRenameOrg = () => {
    console.log('Renaming org...');
    // TODO: API Call - PUT/PATCH /organization/{currentOrg.id}
  };

  const handleInviteMember = () => {
    console.log('Inviting member...');
    // TODO: API Call - POST /organization/{currentOrg.id}/invitations (or similar)
  };

  const handleManageMember = (memberId: string) => {
    console.log('Managing member:', memberId);
    // TODO: Open modal or navigate to manage role/remove member
  };

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Organization Name</CardTitle>
          <CardDescription>Manage your organization's name.</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-1 max-w-sm">
            <Label htmlFor="orgName">Name</Label>
            <Input id="orgName" defaultValue={currentOrg.name} />
          </div>
        </CardContent>
        <CardFooter>
          <Button onClick={handleRenameOrg} disabled>
            Save Name
          </Button>
        </CardFooter>
      </Card>

      <Separator />

      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle>Members</CardTitle>
            <CardDescription>
              Manage organization members and their roles.
            </CardDescription>
          </div>
          <Button onClick={handleInviteMember} disabled>
            Invite Member
          </Button>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>Email</TableHead>
                <TableHead>Role</TableHead>
                <TableHead className="text-right">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {members.length === 0 ? (
                <TableRow>
                  <TableCell
                    colSpan={4}
                    className="text-center text-muted-foreground"
                  >
                    No members found.
                  </TableCell>
                </TableRow>
              ) : (
                members.map((member) => (
                  <TableRow key={member.id}>
                    <TableCell className="font-medium">{member.name}</TableCell>
                    <TableCell>{member.email}</TableCell>
                    <TableCell>{member.role}</TableCell>
                    <TableCell className="text-right">
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleManageMember(member.id)}
                        disabled
                      >
                        Manage
                      </Button>
                    </TableCell>
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      {/* TODO: Add Danger Zone Card for deleting organization */}
    </div>
  );
};

export default OrganizationSettingsPage;
