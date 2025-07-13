export default function ExperimentsPage() {
  // TODO: Make this URL configurable
  const mlflowUrl = 'http://localhost:5000/#/models';

  return (
    <iframe
      src={mlflowUrl}
      className='h-full w-full border-0'
      title='MLflow Experiments'
    />
  );
}
