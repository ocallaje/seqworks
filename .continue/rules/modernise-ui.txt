[metadata]
name = "modernise-ui-react"
description = "Redesign the vanilla‑JS front‑end as a professional React application using Vite + TypeScript + Tailwind CSS."
tags = ["frontend", "react", "tailwind", "design", "ui redesign"]

[steps]
1.  Bootstrap the React project
    - Create a fresh Vite + React + TypeScript boilerplate:
      ```bash
      npm create vite@latest myapp -- --template react-ts
      cd myapp
      npm i -D tailwindcss postcss autoprefixer
      npx tailwindcss init -p
      ```
    - In `tailwind.config.js` enable JIT mode and add a `content` array that covers all `src/*.tsx` and `public/*.html` files.

2.  Colour palette (cross‑theme)
    - **Primary**: `#0069c0` (light) → `#00a8ff` (dark)
    - **Secondary**: `#ff6a00` (light) → `#ffa500` (dark)
    - **Surface Card**: `#ffffff / #1e1e1e`
    - **Background**: `#f8f9fa / #121212`
    - **Accent** (status): success `#28a745`, warning `#ffc107`, error `#dc3545`
    - Store them as CSS variables in `src/styles/theme.css` and import in `index.css`.

3.  Typography
    - Primary font: **Inter** (weight 400, 600, 700) – body and headings.
    - Monospace font: **Source Code Pro** – for parameter inputs & code snippets.
    - Add fonts via Google Fonts import in `index.css`.
    - Standard classes (`.text-primary`, `.text-secondary`, `.body`, `.heading`) refer to theme variables.

4.  Layout & patterns
    - **Main Layout** (`components/Layout/App.tsx`):
      ```tsx
      <div className="flex h-screen bg-bg-page dark:bg-bg-page-dark">
        <Sidebar />
        <div className="flex-1 flex flex-col overflow-hidden">
          <Topbar />
          <main className="p-4 overflow-y-auto flex-1">
            <Outlet />   {/* react‑router v6 */}
          </main>
          <StatusBar />
        </div>
      </div>
      ```
    - **Dashboard cards** (`components/Dashboard/Card.tsx`):
      ```
      <section className="bg-surface dark:bg-surface-dark p-6 rounded-lg shadow-md">
        {children}
      </section>
      ```
    - **Pipeline submission** → Card with a form, standard `input`, `select`, `checkbox` styled via Tailwind.
    - **Progress indicator** → small horizontal bar (10‑step progress) + text, styled with status colours and `animate-pulse` for running state.

5.  Accessibility & validation
    - All `<label>` elements `htmlFor` matches `id`.
    - Required fields: `required`, `maxLength`, `min`/`max` where applicable.
    - ARIA: `role="alert"` for status messages, `aria-live="polite"` for live-updating panels.
    - Keyboard focus: Tailwind’s `focus:outline-none focus:ring-2 focus:ring-primary` utilities.

6.  Status panel (`components/Status/StatusPanel.tsx`)
    - **WebSocket**: listens to `websocket-message` events; updates a shared React context.
    - **Pipeline**: shows statuses `idle`, `queued`, `running`, `finished`, `error` with coloured badges.
    - **CellxGene**: similar, with icons – heroicons `CheckCircle`, `ExclamationCircle`, and a spinning `Refresh` icon when `running`.

7.  Card design
    - Fixed height for the “Run Form” card; overflow-y auto for long cases.
    - Buttons: `btn-primary` class (`bg-primary text-white hover:bg-primary-dark`) and `btn-secondary` for secondary actions.
    - Inputs: Tailwind form control classes – `border-gray-300 rounded-md shadow-sm focus:border-primary focus:ring-1 focus:ring-primary`.

8.  Utilities & component library
    - Create a `components/ui/` folder: `Button.tsx`, `Select.tsx`, `Input.tsx`, `Checkbox.tsx`.
    - Each wraps a base element, applies consistent Tailwind styling, and forwards all props.

9.  Dark mode toggle
    - Store in context – `ThemeProvider` – exposes `isDark`, `toggleDark`.
    - CSS: add `dark` variant prefixes in Tailwind (`dark:bg-surface-dark`, etc.).

10.  Wire‑up authentication
    - Use `backend.ts` (converted from `backend.js` using TS) to expose all RPC wrappers.
    - Store JWT token in `SessionContext`; on login success, set cookie/`localStorage`, then redirect to `/dashboard`.

11.  Testing
    - Verify with `npm run dev` that:
      * All pages load (login, dashboard, bulk, sc, cellxgene).
      * Form validations fire.
      * WebSocket messages show in StatusPanel.
      * Tailwind styles appear as per palette.
      * Dark mode toggles correctly.

[outputs]
- New `src/` folder with React + TS components: `App.tsx`, `index.tsx`, `backend.ts`, `store/` (React context), `components/` (Layout, Dashboard, Auth, Status, UI).
- Tailwind config adjusted with palette.
- `src/styles/theme.css` defining CSS vars.
- Updated `public/index.html` minimal template with `<div id="root"></div>`.
- Removed old `.html` / `.js` files that were legacy.

[requirements]
- Keep business logic untouched; only UI/UX changes.
- All components use TypeScript and are exported as `export default`.
- Design choices as outlined (colour palette, typography, patterns, progress) must be reflected in the CSS/Tailwind utilities.

[retry_policy]
- Any build failure: log the error and exit. Use CI if desired.