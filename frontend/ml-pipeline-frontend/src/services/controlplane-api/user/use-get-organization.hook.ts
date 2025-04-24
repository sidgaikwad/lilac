import { useQuery } from "@tanstack/react-query";
import { QueryKeys } from "../constants";
import { User } from "@/types/api/user";

export interface UseGetOrganizationProps {
    organizationId: string,
}

export function useGetOrganization(props: UseGetOrganizationProps) {
    return useQuery({
        queryKey: [QueryKeys.GET_ORGANIZATION, props.organizationId],
        queryFn: async (): Promise<User> => {
            const resp = await fetch(`/users/${props.organizationId}`, {
                method: 'GET',
                credentials: 'include',
                headers: {
                    'Authorization': `Bearer ${localStorage.getItem('token') ?? ''}`,
                }
            });
            return await resp.json();
        },
        enabled: !!props.organizationId,
    });
}