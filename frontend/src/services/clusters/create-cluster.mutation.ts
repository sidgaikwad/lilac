import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ServiceError } from '@/types';
import { postHttp } from '@/lib/fetch';
import { QueryKeys } from '../constants';
import type { SnakeCasedPropertiesDeep as Sn } from 'type-fest';
import { camelCaseObject, snakeCaseObject } from '@/lib/utils';

export interface CreateClusterRequest {
  clusterName: string;
  clusterDescription?: string;
}

export interface CreateClusterResponse {
  clusterId: string;
}

async function createCluster(
  payload: CreateClusterRequest
): Promise<CreateClusterResponse> {
  const resp = await postHttp<
    Sn<CreateClusterRequest>,
    Sn<CreateClusterResponse>
  >(`/clusters`, snakeCaseObject(payload));
  return camelCaseObject(resp);
}

export interface UseCreateClusterProps {
  onSuccess?: (data: CreateClusterResponse) => void;
  onError?: (error: ServiceError) => void;
}

export function useCreateCluster(props?: UseCreateClusterProps) {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [QueryKeys.CREATE_CLUSTER],
    mutationFn: createCluster,
    onSuccess: (data) => {
      queryClient.invalidateQueries({
        queryKey: [QueryKeys.LIST_CLUSTERS],
      });
      if (props?.onSuccess !== undefined) {
        props.onSuccess(data);
      }
    },
    onError: props?.onError,
  });
}
