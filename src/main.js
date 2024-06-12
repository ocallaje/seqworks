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
  try {
    const authenticated = await invoke("login_with_ssh", { user: email, pass: password });
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

  await invoke("init_pipe", { 
    wrapper: {
      params: {
        AppParams: params // Ensure this matches your Rust enum variant
      }
    }
  });
}    

async function sendSC() {
  document.getElementById('run_status').textContent = "Submitting run ...";

  params = {
      // Setup info
      custom_run_name: document.getElementById('runid').value,
      project: document.getElementById('projectDropdown').textContent,
      organism: document.getElementById('Organism').textContent,
      genome: document.getElementById('genomeDropdown').textContent,
      genome_version: document.getElementById('gencodeDropdown').textContent,
      machine: document.getElementById('machine').textContent,
      workflow: document.getElementById('workflowDropdown').textContent,           

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
      resolution: document.getElementById('resolution').value,
      pcs: document.getElementById('pcs').value,
      integrate: document.getElementById('Integrate').getAttribute('data-clicked'), 
      nonlinear: document.getElementById('nonlinear').getAttribute('data-clicked'),
      identity: document.getElementById('identity').getAttribute('data-clicked'),
      condition: document.getElementById('condition').getAttribute('data-clicked'),
      annotation_method: document.getElementById('annotation_method').textContent,
      regress: document.getElementById('regress').value,
      custom_annotations: document.getElementById('custom_annotations').value,

      // Inspect
      inspect_list: document.getElementById('inspect_list').value,
      annotation_file: document.getElementById('annotation_file').value, 
      meta_group: document.getElementById('meta_group').value,
      de: document.getElementById('DE').getAttribute('data-clicked')
    }

    await invoke("init_pipe", { 
      wrapper: {
        params: {
          AppSCParams: params // Ensure this matches your Rust enum variant
        }
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


async function cellxgene_start() {
  await invoke('cellxgene_startup', {
    params: {
      project: document.getElementById("projectDropdown").textContent,
      h5_file: document.getElementById("h5").textContent
    }
  });
}

async function cellxgene_stop() {
  await invoke('cellxgene_teardown', {
    params: {
      project: document.getElementById("projectDropdown").textContent,
      h5_file: document.getElementById("h5").textContent
    }
  });
}

