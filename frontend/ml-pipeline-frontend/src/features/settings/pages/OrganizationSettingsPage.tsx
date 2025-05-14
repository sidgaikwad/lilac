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
import useOrganizationStore from '@/store/useOrganizationStore';
import { useGetOrganization } from '@/services/controlplane-api/useGetOrganization.hook';
import { Spinner } from '@/components/ui';

const OrganizationSettingsPage: React.FC = () => {
  const selectedOrganizationId = useOrganizationStore(
    (state) => state.selectedOrganizationId
  );

  const {
    data: currentOrg,
    isLoading: isLoadingOrg,
    error: orgError,
  } = useGetOrganization({
    organizationId: selectedOrganizationId,
    enabled: !!selectedOrganizationId,
  });

  
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
  ]; 

  

  const handleRenameOrg = () => {
    console.log('Renaming org...');
    
  };

  const handleInviteMember = () => {
    console.log('Inviting member...');
    
  };

  const handleManageMember = (memberId: string) => {
    console.log('Managing member:', memberId);
    
  };

  return (
    <div className="space-y-6">
      <Card>
        <CardHeader>
          <CardTitle>Organization Name</CardTitle>
          <CardDescription>Manage your organization's name.</CardDescription>
        </CardHeader>
        <CardContent>
          {isLoadingOrg && <Spinner show={isLoadingOrg} />}
          {!isLoadingOrg && orgError && (
            <p className="text-destructive">Error loading organization details.</p>
          )}
          {!isLoadingOrg && !orgError && !currentOrg && selectedOrganizationId && (
            <p className="text-muted-foreground">Organization not found.</p>
          )}
          {!isLoadingOrg && !orgError && !selectedOrganizationId && (
            <p className="text-muted-foreground">
              No organization selected. Please select an organization from the
              header or dashboard.
            </p>
          )}
          {currentOrg && (
            <div className="space-y-1 max-w-sm">
              <Label htmlFor="orgName">Name</Label>
              <Input
                id="orgName"
                defaultValue={currentOrg.name}
                disabled={isLoadingOrg || !currentOrg}
              />
            </div>
          )}
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
