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
    String,
    String,
    String,
    String,
    String,
    String,
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
        illumina_stranded_kit: String,
        strandedness: String,
        paired_end: String,
        trimadaptors: String,
        verify: String,
        merge_fastqs: String,
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

#[derive(Debug, serde::Serialize, PartialEq)]
pub enum Workflow {
    Default,
    QcOnly,
    DeAnalysisOnly,
}

#[derive(Debug, serde::Serialize, PartialEq)]
pub enum Genome {
    Human,
    Mouse,
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
            "Human" => Genome::Human,
            "Mouse" => Genome::Mouse,
            "NHP" => Genome::NHP,
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
    let _ = ftp_cmds::ftp_put_file(&app_params.project, params.to_key_value_map());

    let tmux_pre = format!("tmux new-session -d -s {}", custom_run_name); // Assuming custom_RunName is optional
    let next_pre = "nextflow run /home/carolina/git/NF-RNAseq/main.nf -params-file";
    let rnaseq_cmd = format!("{} {} /mnt/input/data/{}/nextflowParams.json; exec bash -i", tmux_pre, next_pre, &app_params.project);
    
    //println!("{:?}", params);
    Ok(rnaseq_cmd)
    
}

