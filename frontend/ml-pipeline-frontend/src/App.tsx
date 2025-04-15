    // src/App.tsx (Using Sonner Toaster)
    import React from 'react';
    import PipelineEditorPage from './pages/PipelineEditorPage';
    import { Toaster } from "sonner"; // Import Sonner Toaster
    import 'reactflow/dist/style.css';

    function App() {
      return (
        <React.Fragment>
          <PipelineEditorPage />
          <Toaster position="bottom-left" richColors closeButton />
        </React.Fragment>
      );
    }

    export default App;
    