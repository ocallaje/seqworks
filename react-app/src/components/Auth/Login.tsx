import { useContext } from 'react';
import { SessionContext } from '../../store/SessionContext';

export default function Login() {
  const { isAuthenticated } = useContext(SessionContext);

  const handleGeneLinkLogin = () => {
    window.location.href = '/api/auth/oidc/login';
  };

  if (isAuthenticated) {
    window.location.href = '/bulk';
    return null;
  }

  return (
    <div className="flex h-full flex-col items-center justify-center">
      <h2 className="mb-4 text-2xl font-semibold">
        Welcome to GeneLink
      </h2>

      <div className="w-80">
        <div className="rounded-lg bg-surface p-6 shadow-md">
          <p className="mb-4 text-gray-700 dark:text-gray-300">
            Sign in to access your GeneLink account
          </p>

          <button
            type="button"
            onClick={handleGeneLinkLogin}
            className="flex w-full items-center justify-center rounded bg-primary py-2 text-white hover:opacity-90"
          >
            Log in with GeneLink
          </button>
        </div>
      </div>
    </div>
  );
}