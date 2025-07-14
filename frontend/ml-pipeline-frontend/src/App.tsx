import { RouterProvider } from 'react-router-dom';
import { router } from './routes';

// import ThemePanel from './components/common/theme-panel';

function App() {
  return <RouterProvider router={router} />;
  // return (
  //   <div className='flex h-screen w-screen items-center justify-center'>
  //     <div className='space-x-4'>
  //       <ThemePanel />
  //     </div>
  //   </div>
  // );
}

export default App;
