import { getHttp } from '@/lib/fetch';
import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ApiError, Project } from '@/types';
import { useEffect } from 'react';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface GetProjectResponse {
  id: string;
  name: string;
  organizationId: string;
  awsIntegration?: {
    roleArn: string;
    externalId: string;
  }
}

export async function getProject(
  projectId: string
): Promise<GetProjectResponse> {
  const resp = await getHttp<Sn<GetProjectResponse>>(`/projects/${projectId}`);
  return {
    id: resp.id,
    name: resp.name,
    organizationId: resp.organization_id,
    awsIntegration: resp.aws_integration && {
      roleArn: resp.aws_integration.role_arn,
      externalId: resp.aws_integration.external_id,
    },
  };
}

export function getProjectQuery(projectId?: string, enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.GET_PROJECT, projectId],
    queryFn: () => getProject(projectId!),
    enabled: !!projectId && enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data as Project,
  });
}

interface UseGetProjectProps {
  projectId: string | undefined;
  enabled?: boolean;
  onSuccess?: (project: Project) => void;
  onError?: (error: ApiError) => void;
}

export function useGetProject(props: UseGetProjectProps) {
  const query = useQuery(getProjectQuery(props.projectId, props.enabled));

  useEffect(() => {
    if (props?.onSuccess && query.data !== undefined) {
      props.onSuccess(query.data);
    }
  }, [props, query.data]);

  useEffect(() => {
    if (props?.onError && query.error !== null) {
      props.onError(query.error);
    }
  }, [props, query.error]);

  return query;
}
