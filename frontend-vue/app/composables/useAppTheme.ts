export const useAppTheme = () => {
  const theme = useState<'dark' | 'light'>('tae_theme', () => 'dark');

  const clerkAppearance = computed(() => {
    const isDark = theme.value === 'dark';
    return {
      variables: isDark
        ? {
            colorBackground: '#1c1b1b',
            colorInputBackground: '#131313',
            colorText: '#f0f1f3',
            colorTextSecondary: '#c8ccd6',
            colorPrimary: '#00daf3',
            colorTextOnPrimaryBackground: '#04181b',
            colorNeutral: '#ffffff',
            borderRadius: '12px',
            fontFamily: "'Hanken Grotesk', system-ui, sans-serif",
          }
        : {
            colorBackground: '#eef0f2',
            colorInputBackground: '#ffffff',
            colorText: '#2e3347',
            colorTextSecondary: '#5e6478',
            colorPrimary: '#009cb0',
            colorTextOnPrimaryBackground: '#ffffff',
            colorNeutral: '#000000',
            borderRadius: '12px',
            fontFamily: "'Hanken Grotesk', system-ui, sans-serif",
          },
      elements: {
        userButtonPopoverMainIdentifier: { color: isDark ? '#f0f1f3' : '#2e3347' },
        userButtonPopoverSecondaryIdentifier: { color: isDark ? '#c8ccd6' : '#5e6478' },
        userPreviewMainIdentifier: { color: isDark ? '#f0f1f3' : '#2e3347' },
        userPreviewSecondaryIdentifier: { color: isDark ? '#c8ccd6' : '#5e6478' },
        headerTitle: { color: isDark ? '#f0f1f3' : '#2e3347' },
        headerSubtitle: { color: isDark ? '#c8ccd6' : '#5e6478' },
        formFieldLabel: { color: isDark ? '#c8ccd6' : '#5e6478' },
        identityPreviewText: { color: isDark ? '#f0f1f3' : '#2e3347' },
        dividerText: { color: isDark ? '#c8ccd6' : '#5e6478' },
        dividerLine: { background: isDark ? '#2a2a2a' : '#d0d4de' },
        socialButtonsBlockButton: {
          borderColor: isDark ? '#2a2a2a' : '#d0d4de',
          background: isDark ? '#1c1b1b' : '#ffffff',
        },
        footerActionText: { color: isDark ? '#c8ccd6' : '#5e6478' },
        formButtonPrimary: { color: isDark ? '#04181b' : '#ffffff' },
      },
    };
  });

  return { theme, clerkAppearance };
};
