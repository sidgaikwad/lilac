import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { getOrganizationQuery } from '@/services';
import { useParams } from 'react-router-dom';
import {
  Container,
  ContainerContent,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import { useSuspenseQuery } from '@tanstack/react-query';

function OrganizationSettingsPage() {
  const { organizationId } = useParams<'organizationId'>();
  const { data: organization } = useSuspenseQuery(
    getOrganizationQuery(organizationId)
  );

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
    <Container>
      <ContainerHeader>
        <ContainerTitle>Organization Settings</ContainerTitle>
      </ContainerHeader>
      <ContainerContent>
        <Card>
          <CardHeader>
            <CardTitle>General</CardTitle>
            <CardDescription>Update organization details.</CardDescription>
          </CardHeader>
          <CardContent>
            <div key={organization.id} className="max-w-sm space-y-2">
              <Label htmlFor="orgName">Name</Label>
              <Input
                id="orgName"
                defaultValue={organization.name}
                disabled={!organization}
              />
            </div>
          </CardContent>
          <CardFooter>
            <Button onClick={handleRenameOrg} disabled>
              Save Name
            </Button>
          </CardFooter>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>Members</CardTitle>
            <CardDescription>
              Manage organization members and their roles.
            </CardDescription>
            <CardAction>
              <Button onClick={handleInviteMember} disabled>
                Invite Members
              </Button>
            </CardAction>
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
                      className="text-muted-foreground text-center"
                    >
                      No members found.
                    </TableCell>
                  </TableRow>
                ) : (
                  members.map((member) => (
                    <TableRow key={member.id}>
                      <TableCell className="font-medium">
                        {member.name}
                      </TableCell>
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
      </ContainerContent>
    </Container>
  );
}

export default OrganizationSettingsPage;
