// react-app/src/store/SessionContext.tsx
import { createContext, useState, PropsWithChildren } from 'react';

export const SessionContext = createContext({
  user: null as null | { email: string; token: string },
  login: async (_: { email: string; token: string }) => {},
  logout: () => {}
});

export function SessionProvider({ children }: PropsWithChildren<{}>) {
  const [user, setUser] = useState<null | { email: string; token: string }>(null);

  const login = async ({ email, token }) => {
    localStorage.setItem('token', token);
    setUser({ email, token });
  };

  const logout = () => {
    localStorage.removeItem('token');
    setUser(null);
  };

  return (
    <SessionContext.Provider value={{ user, login, logout }}>
      {children}
    </SessionContext.Provider>
  );
}