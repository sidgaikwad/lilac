import { Card } from '@/components/common/card';
import {
  Table,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from '@/components/ui/table';
import { AwsLogo, GcpLogo } from '@/icons';
import { useListCredentials } from '@/services/credentials';
import { CredentialSummary } from '@/types';
import CreateCredentialModal from './create-credential-modal';
import { useState } from 'react';

function getIcon(credentialType: string) {
  switch (credentialType) {
    case 'aws':
      return <AwsLogo className='w-10' />;
    case 'gcp':
      return <GcpLogo className='w-10' />;
    default:
      return undefined;
  }
}

function getTableRow(credential: CredentialSummary) {
  return (
    <TableRow>
      <TableCell>
        {getIcon(credential.credentialType) ?? credential.credentialType}
      </TableCell>
      <TableCell className='font-medium'>{credential.credentialName}</TableCell>
      <TableCell>{credential.credentialDescription}</TableCell>
    </TableRow>
  );
}

export function CredentialsSettingsCard() {
  const { data: credentials } = useListCredentials();
  const [isOpen, setOpen] = useState(false);

  return (
    <Card
      className='w-full max-w-3xl'
      title='Credentials'
      description='Configure your cloud credentials.'
      action={<CreateCredentialModal isOpen={isOpen} setOpen={setOpen} />}
      content={
        <div>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Type</TableHead>
                <TableHead>Name</TableHead>
                <TableHead>Description</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {credentials?.map((cred) => getTableRow(cred))}
            </TableBody>
          </Table>
        </div>
      }
    />
  );
}
