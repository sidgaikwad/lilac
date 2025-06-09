import { BASE_URL } from '@/services';

export async function postHttp<Req, Resp>(
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
  if (!resp.ok) {
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

export async function getHttp<Resp>(
  path: string,
  params?: Record<string, string>,
  withAuth?: boolean
): Promise<Resp>;
export async function getHttp<Req extends Record<string, string>, Resp>(
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
  if (!resp.ok) {
    return Promise.reject({
      statusCode: resp.status,
      ...(await resp.json()),
    });
  }
  return await resp.json();
}

export async function deleteHttp<Resp = void>(
  path: string,
  withAuth: boolean = true
): Promise<Resp> {
  if (path[0] !== '/') {
    path = `/${path}`;
  }
  const authHeader: HeadersInit = withAuth
    ? { Authorization: `Bearer ${localStorage.getItem('token')}` }
    : {};
  const resp = await fetch(`${BASE_URL}${path}`, {
    method: 'DELETE',
    headers: {
      ...authHeader,
    },
  });

  if (!resp.ok) {
    const errorBody = await resp.json().catch(() => ({
      error: resp.statusText || 'Delete operation failed',
    }));
    return Promise.reject({
      statusCode: resp.status,
      error: errorBody.error,
    });
  }

  if (resp.status === 204 || resp.headers.get('Content-Length') === '0') {
    return Promise.resolve({} as Resp);
  }

  if (resp.headers.get('Content-Type')?.includes('application/json')) {
    return await resp.json();
  }

  return Promise.resolve({} as Resp);
}
