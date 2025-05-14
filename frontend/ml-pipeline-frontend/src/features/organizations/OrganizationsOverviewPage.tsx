import React, { useEffect, useState } from 'react';
import { useListOrganizations } from '@/services/controlplane-api/useListOrganizations.hook';
import useOrganizationStore from '@/store/useOrganizationStore';
import { toast } from 'sonner';
import { Spinner } from '@/components/ui';
import CreateOrganizationModal from './components/CreateOrganizationModal';
import EmptyCardSection from '@/components/common/EmptyCardSection';
import { Link } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { BuildingIcon } from 'lucide-react';

const OrganizationsOverviewPage: React.FC = () => {
  const [isCreateOrgModalOpen, setCreateOrgModalOpen] = useState(false);
  const { setSelectedOrganizationId } = useOrganizationStore();

  const { data: organizations = [], isLoading: isLoadingOrganizations } =
    useListOrganizations({
      onError: (error) =>
        toast.error(
          `Error listing organizations: ${error.statusCode} ${error.error}`,
          {
            dismissible: true,
            duration: Infinity,
          }
        ),
    });

  useEffect(() => {
    
    setSelectedOrganizationId(undefined);
  }, [setSelectedOrganizationId]);

  return (
    <div className="container mx-auto p-4 md:p-6 lg:p-8 space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Organizations</h1>
        <CreateOrganizationModal
          isOpen={isCreateOrgModalOpen}
          setOpen={setCreateOrgModalOpen}
        />
      </div>

      {isLoadingOrganizations && <Spinner show={isLoadingOrganizations} />}

      {!isLoadingOrganizations && organizations.length > 0 ? (
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4">
          {organizations.map((org) => (
            <Link
              key={org.id}
              to={`/organizations/${org.id}/projects`}
              className="block hover:no-underline"
              onClick={() => setSelectedOrganizationId(org.id)}
            >
              <Card className="h-full hover:shadow-lg transition-shadow duration-200 ease-in-out">
                <CardHeader>
                  <CardTitle className="flex items-center">
                    <BuildingIcon className="mr-2 h-5 w-5 text-primary" />
                    {org.name}
                  </CardTitle>
                  <CardDescription className="pt-1">
                    View projects in this organization.
                  </CardDescription>
                </CardHeader>
              </Card>
            </Link>
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
    </div>
  );
};
export default OrganizationsOverviewPage;
