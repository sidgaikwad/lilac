import { User } from './user';

// Placeholder for Organization related types

export interface Organization {
  id: string;
  name: string;
  created_at?: string;
  // Add other fields returned by API (e.g., GET /organization/{organization_id})
}

export interface OrganizationMember extends User {
  role: string; // e.g., 'admin', 'member'
  joined_at?: string;
}

// Type for listing organizations (e.g., GET /organization)
export type OrganizationList = Organization[];

// Type for listing members (e.g., GET /organization/{org_id}/members - hypothetical)
export type OrganizationMemberList = OrganizationMember[];