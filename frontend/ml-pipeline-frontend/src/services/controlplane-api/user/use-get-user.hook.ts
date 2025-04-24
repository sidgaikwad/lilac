import { useQuery } from "@tanstack/react-query";
import { QueryKeys } from "../constants";
import { User } from "@/types/api/user";

export interface UseGetUserProps {}

export function useGetUser(_props: UseGetUserProps) {
    return useQuery({
        queryKey: [QueryKeys.GET_USER],
        queryFn: async (): Promise<User> => {
            const resp = await fetch('http://localhost:3000/account/details', {
                method: 'GET',
                credentials: 'include',
                headers: {
                    'Authorization': `Bearer ${localStorage.getItem('token') ?? ''}`,
                }
            });
            return await resp.json();
        },
        enabled: true,
    });
}