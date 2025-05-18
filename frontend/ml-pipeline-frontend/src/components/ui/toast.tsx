import { Toaster as SonnerToaster, ToasterProps } from 'sonner';
import { useTheme } from '../providers/theme-provider';

const Toaster = ({ ...props }: ToasterProps) => {
  const { theme = 'system' } = useTheme();

  return (
    <SonnerToaster
      theme={theme as ToasterProps['theme']}
      className="toaster group"
      position="top-center"
      richColors
      style={
        {
          '--normal-bg': 'var(--popover)',
          '--normal-text': 'var(--popover-foreground)',
          '--normal-border': 'var(--border)',
        } as React.CSSProperties
      }
      {...props}
    />
  );
};

export { Toaster };
