import {
  greethandler,
  registerhandler,
  loginWithSSH,
  getProjectList,
  initPipe,
  startWebsocket,
  listenEvent,
  openCellxgeneInBrowser,
  cellxgeneStartup,
  cellxgeneTeardown
} from "./backend.js";


//const { invoke } = window.__TAURI__.core;
//const { listen } = window.__TAURI__.event;
//import { emit, listen } from '@tauri-apps/api/event'

let greetInputEl;
let greetMsgEl;
let registerUrl;
let ws_message;
let loginButton;
let keycloakButton;

async function greet() {
  //greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
  greetMsgEl.textContent = await greethandler(greetInputEl.value)
}

async function register() {
  //registerUrl.textContent = await invoke("register");
  registerUrl.textContent = await registerhandler("register");
}

async function login() {
  const email = document.getElementById('InputEmail').value;
  const password = document.getElementById('InputPassword').value;
  loginButton.textContent = "Please Wait...";
  console.log("loggin in...")
  try {
    //const authenticated = await invoke("login_with_ssh", { user: email, pass: password });
    const authenticated = await loginWithSSH(email, password);
      if (authenticated) {
          console.log('Authentication successful');
          window.location.href = 'dashboard.html';  // Redirect to dashboard
          await invoke("ws_start");
      } else {
          document.getElementById('auth_error').style.display = 'block';
      }
  } catch (error) {
      console.error('Login failed:', error);
      document.getElementById('auth_error').style.display = 'block';
  } finally {
      loginButton.textContent = "Login";
  }
}

// Run on the callback page after Keycloak redirects back
// Make sure this runs on whatever page handles /auth/callback or you
// can just check the URL path
if (window.location.pathname === "/auth/callback" || window.location.search.includes("access_token")) {
  handleOidcCallback();
}

// example for auth endpoints
//const token = localStorage.getItem("auth_token");
//const response = await fetch("http://localhost:8000/api/some_endpoint", {
//  method: "POST",
//  headers: {
//    "Authorization": `Bearer ${token}`,
//    "Content-Type": "application/json",
//  },
//  body: JSON.stringify(payload),
//});




async function handleOidcCallback() {
  // The Axum callback handler returns JSON — fetch it
  // Read token from URL params
  const params = new URLSearchParams(window.location.search);
  const token = params.get("access_token");

  if (token) {
    localStorage.setItem("auth_token", token);
    window.location.href = "dashboard.html";
  } else {
    console.error("No token received");
    window.location.href = "index.html";
  }
}


export async function request_projects(pipeline) {
  const result = await getProjectList(pipeline)
   return result
}

export async function openCXG() {
  //await invoke("open_cellxgene_in_browser")
  await openCellxgeneInBrowser
}

// Collect and send params for bulk rnaseq
export async function sendBulk() {
  document.getElementById('run_status').textContent = "Submitting run ...";

  const params =  {
      // Params
      illumina_stranded_kit: document.getElementById('check1').getAttribute('data-clicked'),
      strandedness: document.getElementById('strandedness').getAttribute('data-clicked'),
      paired_end: document.getElementById('check2').getAttribute('data-clicked'),
      trimadaptors: document.getElementById('check3').getAttribute('data-clicked'),
      verify: document.getElementById('check4').getAttribute('data-clicked'),
      merge_fastqs: document.getElementById('check5').getAttribute('data-clicked'),
      send_email: document.getElementById('check6').getAttribute('data-clicked'),
      cc: document.getElementById('ccfield').value,
      // Setup info
      custom_run_name: document.getElementById('runid').value,
      project: document.getElementById('projectDropdown').textContent,
      genome: document.getElementById('genomeDropdown').textContent,
      genome_version: document.getElementById('gencodeDropdown').textContent,
      workflow: document.getElementById('workflowDropdown').textContent,
      // Deseq params
      deseq_model: document.getElementById('modelfield').value,
      deseq_ref_var: document.getElementById('reffield').value
  }
  //console.log(params)
  //await invoke("init_pipe", { 
   // wrapper: {
    //  params: {
     //   AppParams: params // Ensure this matches your Rust enum variant
      //}
    //}
  //});
  const payload = {
    wrapper: {
      params: {
        AppParams: params
      }
    }
  };

  console.log(payload);
  await initPipe(payload);
  

}    

export async function sendSC() {
  document.getElementById('run_status').textContent = "Submitting run ...";
  const visiblePanels = document.querySelectorAll('.panel[style*="block"]');

   // Helper function to safely get element value
   function getValue(selector, visiblePanels = document) {
    if (visiblePanels.length > 1) {
      for (let panel of visiblePanels) {
        var element = panel.querySelector(selector);
        if (element) {
          break;
        }
      }
    } else {
      var element = visiblePanels[0].querySelector(selector);
    }
    
    return element ? element.value : null;
  }
  // Helper function to safely get element attribute
  function getAttributeValue(selector, attribute, visiblePanels = document) {
    if (visiblePanels.length > 1) {
      for (let panel of visiblePanels) {
        var element = panel.querySelector(selector);
        if (element) {
          break;
        }
      }
    } else {
      var element = visiblePanels[0].querySelector(selector);
    }
    return element ? element.getAttribute(attribute) : "false";
  }

  params = {
      // Setup info
      custom_run_name: document.getElementById('runid').value,
      project: document.getElementById('projectDropdown').textContent,
      organism: document.getElementById('Organism').textContent,
      genome: document.getElementById('genomeDropdown').textContent,
      genome_version: document.getElementById('gencodeDropdown').textContent,
      machine: document.getElementById('machine').textContent,
      workflow: document.getElementById('workflowDropdown').dataset.value,           

      // Params
      demultiplex: document.getElementById('demultiplex').getAttribute('data-clicked'),
      permit_method: document.getElementById('permit').textContent,
      chemistry: document.getElementById('Chemistry').getAttribute('data-clicked'),          
      send_email: document.getElementById('send_email').getAttribute('data-clicked'),
      cc: document.getElementById('ccfield').value,
      
      // Seurat params
      minnfeature: document.getElementById('min-nfeature').value,
      maxnfeature: document.getElementById('max-nfeature').value,
      mt: document.getElementById('max-percent-mt').value,
      ribo: document.getElementById('max-percent-ribo').value,
      resolution: getValue('#resolution', visiblePanels) || "0",
      pcs: getValue('#pcs', visiblePanels) || "0",
      integrate: document.getElementById('Integrate').getAttribute('data-clicked'), 
      nonlinear: document.getElementById('nonlinear').getAttribute('data-clicked'),
      identity: getAttributeValue('#identity', 'data-clicked', visiblePanels),
      condition: document.getElementById('condition').getAttribute('data-clicked'),
      annotation_method: document.getElementById('annotation_method').textContent,
      regress: document.getElementById('regress').value,
      custom_annotations: document.getElementById('custom_annotations').value,

      // Inspect
      inspect_list: document.getElementById('inspect_list').value,
      annotation_file: document.getElementById('annotation_file').value, 
      meta_group: document.getElementById('meta_group').value,
      de: getAttributeValue('#DE', 'data-clicked', visiblePanels)
    }
    console.log(params)
    //await invoke("init_pipe", { 
    //  wrapper: {
    //    params: {
    //      AppSCParams: params // Ensure this matches your Rust enum variant
    //    }
    //  }
    //});

    await initPipe({
    wrapper: {
      params: {
        AppSCParams: params
      }
    }
  });
}


// LISTENERS 

async function setupListener() {
  //const listener = await listen('websocket-message', (event) => {
  const listener = await listenEvent('websocket-message', (event) => {
    console.log('WebSocket message received:', event.payload);
    const messageObject = event.payload;
    const messageText = messageObject.message;
    ws_message = messageText;
    document.getElementById('ws_status').textContent = messageText;
  });
}

async function pipe_listener() {
  //const listener = await listen('init_result', (event) => {
  const listener = await listenEvent('init_result', (event) => {
    console.log('Pipeline initiation status: ', event.payload);
    document.getElementById('run_status').textContent = event.payload;
  });
}

async function cellxgene_listener() {
  //const listener = await listen('cellxgene_result', (event) => {
  const listener = await listenEvent('cellxgene_result', (event) => {
    console.log('CellxGene status: ', event.payload);
    document.getElementById('cellxgene_status').textContent = event.payload;
  });
}

// end listeners


window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
 
  loginButton = document.getElementById('loginbtn');
  if (loginButton) { // Check if login button exist
    loginButton.addEventListener('click', login);
    keycloakButton = document.getElementById('login-keycloak');
    keycloakButton.addEventListener('click', loginWithKeycloak);
  }
  if (document.getElementById('ws_status')) { // Check if login button exist
    setupListener();                                // Set up the websocket listener
  }
  if (document.getElementById('run_status')) { // Check if run button exist
    pipe_listener();                                // Set up the pipe listener
  }
  if (document.getElementById('cellxgene_status')) { // Check if run button exist
    cellxgene_listener();                                // Set up the pipe listener
  }
});


export async function cellxgene_start() {
  //await invoke('cellxgene_startup', {
    const params = {
      project: document.getElementById("projectDropdown").textContent,
      h5_file: document.getElementById("h5").textContent
    };
  //});
  await cellxgeneStartup(params);
}

export async function cellxgene_stop() {
  //await invoke('cellxgene_teardown', {
    const params = {
      project: document.getElementById("projectDropdown").textContent,
      h5_file: document.getElementById("h5").textContent
    };
    await cellxgeneTeardown(params);
  //});
}

