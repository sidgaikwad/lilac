import { queryOptions, useQuery } from '@tanstack/react-query';
import { QueryKeys } from '../constants';
import { ServiceError, Service } from '@/types';
import { getHttp } from '@/lib/fetch';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';

export interface ListServicesResponse {
  services: Service[];
}

export async function listServices(
  projectId: string
): Promise<ListServicesResponse> {
  const resp = await getHttp<Sn<ListServicesResponse>>(
    `/projects/${projectId}/services`
  );
  return {
    services: resp.services.map((service) => ({
      serviceId: service.service_id,
      serviceName: service.service_name,
      projectId: service.project_id,
      serviceType: service.service_type,
      url: service.url,
    })),
  };
}

export function listServicesQuery(projectId: string, enabled: boolean = true) {
  return queryOptions({
    queryKey: [QueryKeys.LIST_SERVICES, projectId],
    queryFn: () => listServices(projectId),
    enabled,
    staleTime: 1000 * 60 * 5,
    select: (data) => data.services,
  });
}

interface UseListServicesProps {
  enabled?: boolean;
  onSuccess?: (services: Service[]) => void;
  onError?: (error: ServiceError) => void;
}

export function useListServices(
  projectId: string,
  props?: UseListServicesProps
) {
  const query = useQuery({
    ...listServicesQuery(projectId, props?.enabled),
    ...props,
  });

  return query;
}
