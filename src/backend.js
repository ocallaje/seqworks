function isTauri() {
  return typeof window !== "undefined" && window.__TAURI__;
}


let invokeFn = null;
let listenFn = null;

if (isTauri()) {
  const { invoke } = window.__TAURI__.core;
  const { listen } = window.__TAURI__.event;
  invokeFn = invoke;
  listenFn = listen;
}


const API_BASE = "/api"; // Axum prefix


/* -------------------------
   Basic commands
-------------------------- */

export async function greethandler(name) {
  if (isTauri()) {
    return invokeFn("greet", { name });
  }

  const res = await fetch(`${API_BASE}/greet`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ name })
  });
  return res.text();
}

export async function registerhandler() {
  if (isTauri()) {
    return invokeFn("register");
  }

  const res = await fetch(`${API_BASE}/register`);
  return res.text();
}

/* -------------------------
   Auth
-------------------------- */

export async function loginWithSSH(user, pass) {
  if (isTauri()) {
    return invokeFn("login_with_ssh", { user, pass });
  }

  const res = await fetch(`${API_BASE}/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ user, pass })
  });

  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

export async function handleKeycloakCallback() {
  const params = new URLSearchParams(window.location.search);
  const code = params.get("code");
  if (!code) throw new Error("No code in URL");

  // Send code to your backend to exchange for tokens
  const res = await fetch(`${API_BASE}/auth/callback`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ code, redirect_uri: window.location.origin + "/callback" })
  });

  if (!res.ok) throw new Error(await res.text());

  const tokens = await res.json();
  // Save tokens in memory, localStorage, or cookie
  return tokens;
}


/* -------------------------
   Projects
-------------------------- */

export async function getProjectList(pipeType) {
  if (isTauri()) {
    return invokeFn("get_project_list", { pipeType });
  }

  const res = await fetch(`${API_BASE}/projects?pipeType=${pipeType}`);
  return res.json();
}

/* -------------------------
   Pipelines
-------------------------- */

export async function initPipe(payload) {
  if (isTauri()) {
    return invokeFn("init_pipe", payload);
  }

  const res = await fetch(`${API_BASE}/init_pipe`, {
    method: "POST",
    headers: {
    "Authorization": `Bearer ${token}`,
    "Content-Type": "application/json"
  },
    body: JSON.stringify(payload)
  });

  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

/* -------------------------
   Websocket / Events
-------------------------- */

export async function startWebsocket() {
  if (isTauri()) {
    return invokeFn("ws_start");
  }

  // Browser mode: handled by native WebSocket
  console.warn("ws_start is a no-op in browser mode");
}

export async function listenEvent(eventName, callback) {
  if (isTauri()) {
    return listenFn(eventName, callback);
  }

  console.warn(`Event ${eventName} not supported in browser mode`);
}

/* -------------------------
   CellxGene
-------------------------- */

export async function openCellxgeneInBrowser() {
  if (isTauri()) {
    return invokeFn("open_cellxgene_in_browser");
  }

  window.open("/cellxgene", "_blank");
}

export async function cellxgeneStartup(params) {
  if (isTauri()) {
    return invokeFn("cellxgene_startup", { params });
  }

  const res = await fetch(`${API_BASE}/cellxgene/start`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(params)
  });

  return res.json();
}

export async function cellxgeneTeardown(params) {
  if (isTauri()) {
    return invokeFn("cellxgene_teardown", { params });
  }

  const res = await fetch(`${API_BASE}/cellxgene/stop`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(params)
  });

  return res.json();
}