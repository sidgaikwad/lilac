import { User } from './user';

export interface Organization {
  organizationId: string;
  organizationName: string;
}

export interface OrganizationMember extends User {
  role: 'admin' | 'member';
  joinedAt?: string;
}

export type OrganizationList = Organization[];

export type OrganizationMemberList = OrganizationMember[];
