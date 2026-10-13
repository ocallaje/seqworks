import {
  createContext,
  useState,
  useEffect,
  PropsWithChildren,
} from 'react';

interface ThemeContextValue {
  isDark: boolean;
  toggleDark: () => void;
}

export const ThemeContext = createContext<ThemeContextValue>({
  isDark: false,
  toggleDark: () => {},
});

export function ThemeProvider({ children }: PropsWithChildren) {
  const [isDark, setIsDark] = useState(() => {
    const savedTheme = localStorage.getItem('theme');

    if (savedTheme === 'dark') {
      return true;
    }

    if (savedTheme === 'light') {
      return false;
    }

    return window.matchMedia('(prefers-color-scheme: dark)').matches;
  });

  useEffect(() => {
    document.documentElement.classList.toggle('dark', isDark);
    localStorage.setItem('theme', isDark ? 'dark' : 'light');
  }, [isDark]);

  const toggleDark = () => {
    setIsDark((dark) => !dark);
  };

  return (
    <ThemeContext.Provider value={{ isDark, toggleDark }}>
      {children}
    </ThemeContext.Provider>
  );
}