// src/App.tsx
import PipelineEditor from './components/PipelineEditor';
import 'reactflow/dist/style.css'; // Ensure React Flow styles are imported

function App() {
  return (
    <main className="h-screen w-screen overflow-hidden"> {/* Prevent scrolling on main page */}
      {/* You could add a Header/Navbar here above the editor */}
      <PipelineEditor />
    </main>
  );
}

export default App;