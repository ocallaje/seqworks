#SeqWorks
This app allows users to interact with the following custom RNAseq nextflow pipelines
* Bulk RNAseq
* Single Cell RNAseq
* CHIPseq (coming soon)

## Dev Notes
SeqWorks is built using the Tauri V2 framework, using rust to process user input and interface with servers, with HTML, JS and CSS for serving the frontend. 

## Using SeqWorks
### Login
A valid and active TCD user account is required to log into the app. A direct ethernet or TCD VPN connection must be used for the app to communicate with TCD servers. 
<img width="643" alt="Screenshot 2024-07-06 181727" src="https://github.com/ocallaje/seqworks/assets/95083099/456b7bec-a5f2-439d-9a8b-87d116dde4be">

### Dashboard (coming soon)
The dashboard allows for monitoring active analysis runs using a nextflow monitoring websocket service (https://github.com/ocallaje/nextflow_monitor)

### Bulk RNAseq


### Single Cell RNAseq
