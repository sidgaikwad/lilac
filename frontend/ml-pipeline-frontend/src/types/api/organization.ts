import { User } from './user';

export interface Organization {
  id: string;
  name: string;
}

export interface OrganizationMember extends User {
  role: 'admin' | 'member';
  joinedAt?: string;
}

export type OrganizationList = Organization[];

export type OrganizationMemberList = OrganizationMember[];
