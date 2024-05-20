const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
//import { emit, listen } from '@tauri-apps/api/event'

let greetInputEl;
let greetMsgEl;
let registerUrl;
let ws_message;

async function greet() {
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function register() {
  registerUrl.textContent = await invoke("register");
}

async function login() {
  const email = document.getElementById('InputEmail').value;
  const password = document.getElementById('InputPassword').value;
  loginButton.textContent = "Please Wait...";
  authenticated = await invoke("login_with_ssh", {user: email, pass: password});
  if (authenticated) {
    // Redirect or perform other actions on successful login
    console.log('Authentication successful');  
    window.location.href = 'dashboard.html';        // redirect to dashboard
    connect_to_socket = await invoke("ws_listen");  // connect to websocket 
    
  } else {
        // Display error message
    document.getElementById('auth_error').style.display = 'block';
  }
}

async function setupListener() {
  const listener = await listen('websocket-message', (event) => {
    console.log('WebSocket message received:', event.payload);
    const messageObject = event.payload;
    const messageText = messageObject.message;
    ws_message = messageText;
    document.getElementById('ws_status').textContent = messageText;
  });
}

async function request_projects(pipeline) {
  const result = await invoke("get_project_list", { pipeType: pipeline });
   return result
}

// Collect and send params for bulk rnaseq
async function sendBulk() {
  document.getElementById('run_status').textContent = "Submitting run ...";

  params = await invoke("init_bulk", { 
    appParams: {
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
  });
}    

async function pipe_listener() {
  const listener = await listen('init_result', (event) => {
    console.log('Pipeline initiation status: ', event.payload);
    document.getElementById('run_status').textContent = event.payload;
  });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  //document.querySelector("#greet-form").addEventListener("submit", (e) => {
   // e.preventDefault();
   // greet();
  //});
  if (document.getElementById('loginbtn')) { // Check if login button exist
    loginButton = document.getElementById('loginbtn');
    loginButton.addEventListener('click', login);
  }
  if (document.getElementById('ws_status')) { // Check if login button exist
    setupListener();                                // Set up the websocket listener
  }
  if (document.getElementById('run_status')) { // Check if run button exist
    pipe_listener();                                // Set up the pipe listener
  }
});

