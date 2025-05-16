import { BASE_URL } from '@/services';

export async function post<Req, Resp>(
  path: string,
  request: Req,
  withAuth: boolean = true
): Promise<Resp> {
  if (path[0] !== '/') {
    path = `/${path}`;
  }
  const authHeader: HeadersInit = withAuth
    ? { Authorization: `Bearer ${localStorage.getItem('token')}` }
    : {};
  const resp = await fetch(`${BASE_URL}${path}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...authHeader,
    },
    body: JSON.stringify(request),
  });
  if (resp.status < 200 || resp.status >= 300) {
    return Promise.reject({
      statusCode: resp.status,
      ...(await resp.json()),
    });
  }
  if (resp.headers.get('Content-Type') === 'application/json') {
    return await resp.json();
  }
  return {} as Resp;
}

export async function get<Req extends Record<string, string>, Resp>(
  path: string,
  params?: Req,
  withAuth: boolean = true
): Promise<Resp> {
  if (path[0] !== '/') {
    path = `/${path}`;
  }
  const searchParams = new URLSearchParams();
  Object.entries(params ?? {}).forEach(([key, value]) =>
    searchParams.append(key, value)
  );
  const authHeader: HeadersInit = withAuth
    ? { Authorization: `Bearer ${localStorage.getItem('token')}` }
    : {};
  const resp = await fetch(`${BASE_URL}${path}?${searchParams.toString()}`, {
    method: 'GET',
    headers: authHeader,
  });
  if (resp.status < 200 || resp.status >= 300) {
    return Promise.reject({
      statusCode: resp.status,
      ...(await resp.json()),
    });
  }
  return await resp.json();
}
