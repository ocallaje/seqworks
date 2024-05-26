use crate::app_state::AppState;
use tauri::State;
use crate::ftp_cmds;
use serde_json::{json, Map};

#[derive(serde::Deserialize)]
pub struct AppParams {
  illumina_stranded_kit: String,
  strandedness: String,
  paired_end: String,
  trimadaptors: String,
  verify: String,
  merge_fastqs: String,
  send_email: String,
  cc: String,
  custom_run_name: String,
  project: String,
  genome: String,
  genome_version: String,
  workflow: String,
  deseq_model: String,
  deseq_ref_var: String, 
}

#[derive(serde::Serialize, Debug)]
pub struct BulkParams (
    String,
    String,
    String,
    bool,
    String,
    bool,
    bool,
    bool,
    bool,
    String,
    bool,
    String,
    String,
    Genome,
    String,
    Workflow,
    String,
    String,
    String,
    String, 
    String,
    String,
);

impl BulkParams {
    #[allow(non_snake_case)]
    pub fn new(
        input: String,
        outdir: String,
        countsFile: String,
        illumina_stranded_kit: bool,
        strandedness: String,
        paired_end: bool,
        trimadaptors: bool,
        verify: bool,
        merge_fastqs: bool,
        email: String,
        send_email: bool,
        cc: String,
        custom_RunName: String,
        genome: Genome,
        genome_version: String,
        workflow: Workflow,
        deseq2SampleSheet: String,
        deseq2Model: String,
        deseq2ReferenceVar: String,
        deseq2ContrastVar: String, 
        deseq2TargetVar: String,
        synology_link: String,) -> Self {
        Self(input, outdir, countsFile, illumina_stranded_kit, strandedness, paired_end, trimadaptors, verify, 
            merge_fastqs, email, send_email, cc, custom_RunName, genome, genome_version, workflow, 
            deseq2SampleSheet, deseq2Model, deseq2ReferenceVar, deseq2ContrastVar, deseq2TargetVar, synology_link
        )
    }
    // Method to convert struct into key-value pairs map
    fn to_key_value_map(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut map = Map::new();
        map.insert("input".to_string(), json!(self.0));
        map.insert("outdir".to_string(), json!(self.1));
        map.insert("countsFile".to_string(), json!(self.2));
        map.insert("illumina_stranded_kit".to_string(), json!(self.3));
        map.insert("strandedness".to_string(), json!(self.4));
        map.insert("paired_end".to_string(), json!(self.5));
        map.insert("trimadaptors".to_string(), json!(self.6));
        map.insert("verify".to_string(), json!(self.7));
        map.insert("merge_fastqs".to_string(), json!(self.8));
        map.insert("email".to_string(), json!(self.9));
        map.insert("send_email".to_string(), json!(self.10));
        map.insert("cc".to_string(), json!(self.11));
        map.insert("custom_RunName".to_string(), json!(self.12));
        map.insert("genome".to_string(), json!(self.13));
        map.insert("genome_version".to_string(), json!(self.14));
        map.insert("workflow".to_string(), json!(self.15));
        map.insert("deseq2SampleSheet".to_string(), json!(self.16));
        map.insert("deseq2Model".to_string(), json!(self.17));
        map.insert("deseq2ReferenceVar".to_string(), json!(self.18));
        map.insert("deseq2ContrastVar".to_string(), json!(self.19));
        map.insert("deseq2TargetVar".to_string(), json!(self.20));
        map.insert("synology_link".to_string(), json!(self.21));
        map
    }
}

#[derive(serde::Deserialize)]
pub struct AppSCParams {
    custom_run_name: String,
    project: String,
    organism: String,
    genome: String,
    genome_version: String,
    machine: String,
    workflow: String,           
    demultiplex: String,
    permit_method: String,
    chemistry: String,          
    send_email: String,
    cc: String,
    minnfeature: String,
    maxnfeature: String,
    mt: String,
    ribo: String,
    resolution: String,
    pcs: String,
    integrate: String, 
    nonlinear: String,
    identity: String,
    condition: String,
    annotation_method: String,
    inspect_list: String,
    annotation_file: String, 
    meta_group: String,
    de: String
  }

#[derive(serde::Serialize, Debug)]
pub struct SCParams (
    String,
    SCGenome,
    String,
    String,
    String,
    String,
    bool,
    String,
    String,
    bool,
    String,
    i32,
    i32,
    i32,
    i32,
    i32,
    i32,
    bool,
    bool,
    bool,
    bool,
    String,
    String, 
    String,
    bool,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
);

impl SCParams {
    #[allow(non_snake_case)]
    pub fn new(
        custom_run_name: String,
        organism: SCGenome,
        genome: String,
        genome_version: String,
        machine: String,
        workflow: String,
        demultiplex: bool,
        permit_method: String,
        chemistry: String,
        send_email: bool,
        cc: String,
        minnfeature: i32,
        maxnfeature: i32,
        mt: i32,
        ribo: i32,
        resolution: i32,
        pcs: i32,
        integrate: bool,
        nonlinear: bool, 
        identity: bool,
        condition: bool,
        inspect_list: String,
        annotation_file: String,
        meta_group: String,
        DE: bool,
        input: String,
        outdir: String,
        email: String,
        synology_link: String,
        publish_dir_mode: String,
        scriptDir: String,
        index_mapping_file: String,
        instrument_mapping_file: String,
        annotation_method: String,

    ) -> Self {
        Self(custom_run_name, organism, genome, genome_version, machine, workflow, demultiplex,
            permit_method, chemistry, send_email, cc, minnfeature, maxnfeature, mt, ribo,
            resolution, pcs, integrate, nonlinear, identity, condition, inspect_list, annotation_file,
            meta_group, DE, input, outdir, email, synology_link, publish_dir_mode, scriptDir,
            index_mapping_file, instrument_mapping_file, annotation_method
        )
    }
    // Method to convert struct into key-value pairs map
    fn to_key_value_map(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut map = Map::new();
        map.insert("custom_RunName".to_string(), json!(self.0));
        map.insert("organism".to_string(), json!(self.1));
        map.insert("genome".to_string(), json!(self.2));
        map.insert("genome_version".to_string(), json!(self.3));
        map.insert("machine".to_string(), json!(self.4));
        map.insert("workflow".to_string(), json!(self.5));
        map.insert("demultiplex".to_string(), json!(self.6));
        map.insert("permit_method".to_string(), json!(self.7));
        map.insert("chemistry".to_string(), json!(self.8));
        map.insert("send_email".to_string(), json!(self.9));
        map.insert("cc".to_string(), json!(self.10));
        map.insert("minnfeature".to_string(), json!(self.11));
        map.insert("maxnfeature".to_string(), json!(self.12));
        map.insert("mt".to_string(), json!(self.13));
        map.insert("ribo".to_string(), json!(self.14));
        map.insert("resolution".to_string(), json!(self.15));
        map.insert("pcs".to_string(), json!(self.16));
        map.insert("integrate".to_string(), json!(self.17));
        map.insert("nonlinear".to_string(), json!(self.18));
        map.insert("identity".to_string(), json!(self.19));
        map.insert("condition".to_string(), json!(self.20));
        map.insert("inspect_list".to_string(), json!(self.21));
        map.insert("annotation_file".to_string(), json!(self.22));
        map.insert("meta_group".to_string(), json!(self.23));
        map.insert("DE".to_string(), json!(self.24));
        map.insert("input".to_string(), json!(self.25));
        map.insert("outdir".to_string(), json!(self.26));
        map.insert("email".to_string(), json!(self.27));
        map.insert("synology_link".to_string(), json!(self.28));
        map.insert("publish_dir_mode".to_string(), json!(self.29));
        map.insert("scriptDir".to_string(), json!(self.30));
        map.insert("index_mapping_file".to_string(), json!(self.31));
        map.insert("instrument_mapping_file".to_string(), json!(self.32));
        map.insert("annotation_method".to_string(), json!(self.33));
        map
    }
}

#[derive(Debug, serde::Serialize, PartialEq)]
pub enum Workflow {
    Default,
    QcOnly,
    DeAnalysisOnly,
}

#[derive(Debug, serde::Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Genome {
    hg38,
    mm39,
    chlsab1,
}
#[derive(Debug, serde::Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SCGenome {
    human,
    mouse,
    NHP,
}

pub fn parse_bulk_params(app_params: AppParams, state: State<'_, AppState>) -> Result<String, String> {
    let custom_run_name: String;
    if app_params.custom_run_name.is_empty() {
        custom_run_name = "".to_string();
        } else {
        custom_run_name = app_params.custom_run_name;
    }
    
    let strandedness = if app_params.strandedness.parse().map_err(|e| format!("Failed to parse strandedness: {}", e))?
     { "reverse".to_string() } else { "forward".to_string() };

    let username:String = {
        let username = state.username.lock().unwrap();
        username.clone().ok_or("Username not set")?
    };
    let contrast: String;
    if let Some(model_items) = app_params.deseq_model.split('+').last() {
        contrast = model_items.replace("~", "").to_string();
    } else {
        contrast = app_params.deseq_model.split('~').last().unwrap_or("").to_string();
    };

    let params = BulkParams::new(
        format!("/mnt/input/data/{}/samplesheet_reads.csv", app_params.project),
        format!("/mnt/output/bulk_RNAseq/{}", app_params.project),
        format!("/mnt/output/bulk_RNAseq/{}/counts.csv", app_params.project),
        app_params.illumina_stranded_kit.parse().map_err(|e| format!("Failed to parse illumina stranded kit: {}", e))?,
        strandedness,
        app_params.paired_end.parse().map_err(|e| format!("Failed to parse paired end: {}", e))?,
        app_params.trimadaptors.parse().map_err(|e| format!("Failed to parse trimadaptors: {}", e))?,
        app_params.verify.parse().map_err(|e| format!("Failed to parse verify: {}", e))?,
        app_params.merge_fastqs.parse().map_err(|e| format!("Failed to parse merge fastqs: {}", e))?,
        format!("{}@tcd.ie", username),
        app_params.send_email.parse().map_err(|e| format!("Failed to parse send email: {}", e))?,
        app_params.cc,
        custom_run_name.clone(),
        match app_params.genome.as_str() {
            "Human" => Genome::hg38,
            "Mouse" => Genome::mm39,
            "NHP" => Genome::chlsab1,
            _ => return Err("Invalid genome option".to_string()), // Handle invalid option
        },
        app_params.genome_version,
        match app_params.workflow.as_str() {
            "de_analysis_only" => Workflow::DeAnalysisOnly,
            "default" => Workflow::Default,
            "qc_only" => Workflow::QcOnly,
            // Add more options as needed
            _ => return Err("Invalid workflow option".to_string()), // Handle invalid option
        },
        format!("/mnt/input/data/{}/samplesheet_deseq.csv", app_params.project),
        app_params.deseq_model.replace(" ", ""),
        app_params.deseq_ref_var.clone(),
        contrast.clone(),
        ftp_cmds::parse_de_samplesheet(&app_params.project, contrast.clone(), app_params.deseq_ref_var)?,
        "http://CampbellLab.quickconnect.to/d/f/623389304994967313".to_string(),
    );
    

    //let encoded = serde_json::to_string(&params).map_err(|e| format!("Failed to encode JSON: {}", e))?;
    //std::fs::write("nextflowParams.json", encoded.as_bytes()).map_err(|e| format!("Failed to write JSON file: {}", e))?;
    
    // put this json file in project folder
    let _ = ftp_cmds::ftp_put_file(&app_params.project, params.to_key_value_map(), "bulk");

    let tmux_pre = format!("tmux new-session -d -s {}", custom_run_name); // Assuming custom_RunName is optional
    let next_pre = "nextflow run /home/carolina/git/NF-RNAseq/main.nf -params-file";
    let rnaseq_cmd = format!("{} '{} /mnt/input/data/{}/nextflowParams.json'", tmux_pre, next_pre, &app_params.project);
    //let rnaseq_cmd = format!("{} /mnt/input/data/{}/nextflowParams.json", next_pre, &app_params.project);
    
    //println!("{:?}", params);
    Ok(rnaseq_cmd)
    
}


pub fn parse_sc_params(app_params: AppSCParams, state: State<'_, AppState>) -> Result<String, String> {
    let custom_run_name: String;
    if app_params.custom_run_name.is_empty() {
        custom_run_name = "".to_string();
        } else {
        custom_run_name = app_params.custom_run_name;
    }

    let username:String = {
        let username = state.username.lock().unwrap();
        username.clone().ok_or("Username not set")?
    };

    let chemistry: String;
    if app_params.chemistry == "true" {
        chemistry = "chromiumV3".to_string();
    } else {
        chemistry = "chromiumV2".to_string();
    }

    let params: SCParams = SCParams::new(
        custom_run_name.clone(),
        match app_params.organism.as_str() {
            "human" => SCGenome::human,
            "mouse" => SCGenome::mouse,
            "NHP" => SCGenome::NHP,
            _ => return Err("Invalid genome option".to_string()),
        },
        app_params.genome,
        app_params.genome_version, 
        app_params.machine,
        app_params.workflow,
        app_params.demultiplex.parse().map_err(|e| format!("Failed to parse merge fastqs: {}", e))?,
        app_params.permit_method,
        chemistry,
        app_params.send_email.parse().map_err(|e| format!("Failed to parse merge fastqs: {}", e))?,
        app_params.cc,
        match app_params.minnfeature.as_str() {
            "" => "5000".parse().expect("Not a valid number for minnfeature"),
            _ => app_params.minnfeature.parse().expect("Not a valid number for minnfeature"),
        },
        match app_params.maxnfeature.as_str() {
            "" => "200".parse().expect("Not a valid number for maxnfeature"),
            _ => app_params.maxnfeature.parse().expect("Not a valid numberfor maxnfeature"),
        },
        match app_params.mt.as_str() {
            "" => "20".parse().expect("Not a valid number for mt"),
            _ => app_params.mt.parse().expect("Not a valid number for mt"),
        },
        match app_params.ribo.as_str() {
            "" => "100".parse().expect("Not a valid number for ribo"),
            _ => app_params.ribo.parse().expect("Not a valid number for ribo"),
        },
        match app_params.resolution.as_str() {
            "" => "0".parse().expect("Not a valid number for resolution"),
            _ => app_params.resolution.parse().expect("Not a valid number for resolution"),
        },
        match app_params.pcs.as_str() {
            "" => "0".parse().expect("Not a valid number for pcs"),
            _ => app_params.pcs.parse().expect("Not a valid number for pcs"),
        },
        app_params.integrate.parse().map_err(|e| format!("Failed to parse merge fastqs: {}", e))?,
        app_params.nonlinear.parse().map_err(|e| format!("Failed to parse merge fastqs: {}", e))?, 
        app_params.identity.parse().map_err(|e| format!("Failed to parse merge fastqs: {}", e))?,
        app_params.condition.parse().map_err(|e| format!("Failed to parse merge fastqs: {}", e))?,
        app_params.inspect_list,
        match app_params.annotation_file.as_str() {
            "" => "none".to_string(),
            _ => format!("/mnt/output/single_cell_RNAseq/{}/{}_new_annotations/{}", app_params.project, app_params.project, app_params.annotation_file),
        },
        match app_params.meta_group.as_str() {
            "" => "seurat_clusters".to_string(),
            _ => app_params.meta_group,
        },
        app_params.de.parse().map_err(|e| format!("Failed to parse merge fastqs: {}", e))?,
        format!("/mnt/input/data_singlecell/{}/", app_params.project),
        format!("/mnt/output/single_cell_RNAseq/{}", app_params.project),
        format!("{}@tcd.ie", username),
        "http://CampbellLab.quickconnect.to/d/f/623389304994967313".to_string(),
        "copy".to_string(),
        "/mnt/input/refs/bin".to_string(),
        "/mnt/input/refs/index_sets/Dual_Index_Kit_TT_Set_A.json".to_string(),
        "/mnt/input/refs/instruments/instrumentation.json".to_string(),
        app_params.annotation_method,
    );

    let _ = ftp_cmds::ftp_put_file(&app_params.project, params.to_key_value_map(), "single_cell");

    let tmux_pre = format!("tmux new-session -d -s {}", custom_run_name); // Assuming custom_RunName is optional
    let next_pre = "'nextflow run /home/carolina/pipelines/NF-scRNAseq/main.nf -params-file";
    let rnaseq_cmd = format!("{} {} /mnt/input/data_singlecell/{}/nextflowParams.json'", tmux_pre, next_pre, &app_params.project);
    
    Ok(rnaseq_cmd)
}


