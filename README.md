# SeqWorks
This app allows users to interact with the following custom RNAseq nextflow pipelines
* Bulk RNAseq
* Single Cell RNAseq
* CHIPseq (coming soon)

## Dev Notes
SeqWorks is built using the Tauri V2 framework, using rust to process user input and interface with servers, with HTML, JS and CSS for serving the frontend. You may experience layout issues if your display is set to custom scaling.

## Using SeqWorks
### Login
A valid and active TCD user account is required to log into the app. A direct ethernet or TCD VPN connection must be used for the app to communicate with TCD servers. 

<img width="643" alt="Screenshot 2024-07-06 181727" src="https://github.com/ocallaje/seqworks/assets/95083099/456b7bec-a5f2-439d-9a8b-87d116dde4be">


### Dashboard (coming soon)
The dashboard allows for monitoring active analysis runs using a nextflow monitoring websocket service (https://github.com/ocallaje/nextflow_monitor)



### Bulk RNAseq
1. Select and enter project details in the "setup" panel.
   * The "default" workflow will run everything from alignment to differential expression analysis
   * The "QC only" workflow will run alignment and QC reports without differential expression analysis
   * The "DESeq" workflow will use previously aligned data to run differential expression analysis

2. The pipeline parameters panel is used to select experimental-specific options.

3. Enter your DESEQ model and reference, based on the columns in the samplesheet_deseq.csv file

4. Click run to start the pipeline and wait for email confirmation
   

![Screenshot 2024-07-07 012218](https://github.com/ocallaje/seqworks/assets/95083099/3a475395-ec24-4120-a0cd-84e2a5289fa9)


### Single Cell RNAseq
##### Run Pipeline
1. Select and enter project details in the "setup" panel.
   * The "standard" workflow will run everything from raw BCL files to alignment to processing clusters (see below)
   * The "Demultiplex" workflow will demultiplex BCL files into FASTQ files only
   * The "Process clusters" workflow will use previously aligned data to determine clusters at different resolutions. It is recommended that the user select an appropriate resolution from this data output before proceeding
   * The "Analyse clusters" workflow takes processed data from either the standard or process clusters workflows, and using the chosen resolution, will perform all downstream analysis
   * The "inspect" workflow can be used on previously analysed data from the "analyse clusters" workflow to use clusters defined in cellxgene, or to examine expression patterns of specific genes

2. The pipeline parameters panel is used to select experimental-specific options.

3. Depending on the workflow, the seurat parameters should be filled in or left blank (for defaults) as appropriate

##### CellxGene
1. To explore clusters in detail, previously analysed data can be selected and the user can click "start cellxgene" to inject the data into a docker container running the cellxgene platform. After 10 seconds, a browser will open to the container. 
Data can be explored ad custom clusters can be annotated as per cellxgene documentation https://cellxgene.cziscience.com/docs/01__CellxGene
2. To save your custom annotations and safely close the container and data, click "stop cellxgene" in the seqworks cellxgene panel. These path to this new annotation file can be used as an input into the inspect workflow

- The "open cellxgene" button is only used to open any existing cellxgene containers.

![Screenshot 2024-07-07 013457](https://github.com/ocallaje/seqworks/assets/95083099/06ee3eb9-22f9-420e-b523-d52582e40f54)
