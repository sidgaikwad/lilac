import { useTheme } from 'next-themes';
import { Toaster as Sonner, ToasterProps } from 'sonner';

const Toaster = ({ ...props }: ToasterProps) => {
  const { theme = 'system' } = useTheme();

  return (
    <Sonner
      theme={theme as ToasterProps['theme']}
      className='toaster group'
      position='top-center'
      richColors
      style={
        {
          '--normal-bg': 'var(--accent-secondary)',
          '--normal-text': 'var(--accent-text-muted)',
          '--normal-border': 'var(--accent-border-hover)',
          '--success-bg': 'var(--green-3)',
          '--success-text': 'var(--green-11)',
          '--success-border': 'var(--green-8)',
          '--error-bg': 'var(--red-3)',
          '--error-text': 'var(--red-11)',
          '--error-border': 'var(--red-8)',
          '--info-bg': 'var(--blue-3)',
          '--info-text': 'var(--blue-11)',
          '--info-border': 'var(--blue-8)',
        } as React.CSSProperties
      }
      {...props}
    />
  );
};

export { Toaster };
