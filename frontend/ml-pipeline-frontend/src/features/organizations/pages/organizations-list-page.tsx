import { useEffect, useState } from 'react';
import useOrganizationStore from '@/store/use-organization-store';
import { toast } from 'sonner';
import { Spinner } from '@/components/ui/spinner';
import CreateOrganizationModal from '../components/create-organization-modal';
import EmptyCardSection from '@/components/common/empty-card-section';
import { Link } from 'react-router-dom';
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
} from '@/components/ui/card';
import { BuildingIcon } from 'lucide-react';
import { useListOrganizations } from '@/services';
import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
} from '@/components/ui/container';
import DeleteOrganizationModal from '../components/delete-organization-modal';

function OrganizationsListPage() {
  const [isCreateOrgModalOpen, setCreateOrgModalOpen] = useState(false);
  const { setSelectedOrganizationId } = useOrganizationStore();

  const { data: organizations = [], isLoading: isLoadingOrganizations } =
    useListOrganizations({
      onError: (error) =>
        toast.error('Error listing organizations', {
          description: `${error.statusCode} ${error.error}`,
        }),
    });

  useEffect(() => {
    setSelectedOrganizationId(undefined);
  }, [setSelectedOrganizationId]);

  return (
    <Container>
      <ContainerHeader>
        <ContainerTitle>
          Organizations
          <ContainerDescription>Select your organization</ContainerDescription>
        </ContainerTitle>
        <ContainerAction>
          <CreateOrganizationModal
            isOpen={isCreateOrgModalOpen}
            setOpen={setCreateOrgModalOpen}
          />
        </ContainerAction>
      </ContainerHeader>

      <ContainerContent>
        {isLoadingOrganizations && <Spinner show={isLoadingOrganizations} />}

        {!isLoadingOrganizations && organizations.length > 0 ? (
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3">
            {organizations.map((org) => (
              <Card className="h-full transition-shadow duration-200 ease-in-out hover:shadow-lg">
                <CardHeader>
                  <Link
                    key={org.id}
                    to={`/organizations/${org.id}/projects`}
                    className="hover:text-primary"
                    onClick={() => setSelectedOrganizationId(org.id)}
                  >
                    <CardTitle className="flex items-center">
                      <BuildingIcon className="text-primary mr-2 h-5 w-5" />
                      {org.name}
                    </CardTitle>
                  </Link>
                  <CardDescription className="pt-1">
                    View projects in this organization.
                  </CardDescription>
                  <CardAction>
                    <DeleteOrganizationModal organization={org} />
                  </CardAction>
                </CardHeader>
              </Card>
            ))}
          </div>
        ) : (
          !isLoadingOrganizations && (
            <EmptyCardSection
              title="No Organizations"
              description="Get started by creating a new organization."
              buttonText="Create Organization"
              onClick={() => setCreateOrgModalOpen(true)}
            />
          )
        )}
      </ContainerContent>
    </Container>
  );
}
export default OrganizationsListPage;
