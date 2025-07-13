import { useEffect, useRef } from 'react';
import { useLocation, useParams } from 'react-router-dom';
import { useSsoExchange } from '@/services';
import { Spinner } from '@/components/ui/spinner';

function SsoCallbackPage() {
  const location = useLocation();
  const { provider, type } = useParams();
  const { mutate: ssoExchange } = useSsoExchange();
  const exchangeCalled = useRef(false);

  useEffect(() => {
    if (exchangeCalled.current) {
      return;
    }

    const params = new URLSearchParams(location.search);
    const code = params.get('code');
    const state = params.get('state');

    if (code && state && provider && type) {
      exchangeCalled.current = true;
      ssoExchange({ code, state, provider, type });
    }
  }, [location, provider, type, ssoExchange]);

  return (
    <div className='flex h-screen items-center justify-center'>
      <Spinner />
    </div>
  );
}

export default SsoCallbackPage;
