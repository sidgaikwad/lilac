
export default function NotebooksPage() {
  // TODO: Make this URL configurable
  const notebooksUrl = 'http://localhost:8082';

  return (
    <iframe
      src={notebooksUrl}
      className="h-full w-full border-0"
      title="JupyterLab"
    />
  );
}