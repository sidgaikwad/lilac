import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import App from './App.tsx'
import { ThemeProvider } from './components/providers/ThemeProvider.tsx' // Import ThemeProvider
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <BrowserRouter>
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme"> {/* Wrap App */}
        <App />
      </ThemeProvider>
    </BrowserRouter>
  </React.StrictMode>,
)
