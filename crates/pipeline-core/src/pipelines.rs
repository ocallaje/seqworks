use crate::{states, utils};
use serde_json::{Map, Value};

pub struct PipelineResult {
    pub rnaseq_cmd: String,
    pub params_map: Map<String, Value>,
}

pub fn parse_bulk_params(app_params: states::AppParams, username: &str) -> Result<PipelineResult, String> {
    let mut custom_run_name: String;
    if app_params.custom_run_name.is_empty() {
        custom_run_name = "".to_string();
        } else {
        custom_run_name = app_params.custom_run_name;
    }
    custom_run_name = custom_run_name.replace(" " ,"_");
    
    let strandedness = if app_params.strandedness.parse().map_err(|e| format!("Failed to parse strandedness: {}", e))?
     { "reverse".to_string() } else { "forward".to_string() };
    
    let contrast: String;
    if let Some(model_items) = app_params.deseq_model.split('+').last() {
        contrast = model_items.replace("~", "").to_string();
    } else {
        contrast = app_params.deseq_model.split('~').last().unwrap_or("").to_string();
    };
    
    let params = states::BulkParams::new(
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
            "Human" => states::Genome::hg38,
            "Mouse" => states::Genome::mm39,
            "NHP" => states::Genome::chlsab1,
            _ => return Err("Invalid genome option".to_string()), // Handle invalid option
        },
        app_params.genome_version,
        match app_params.workflow.as_str() {
            "de_analysis_only" => states::Workflow::de_analysis_only,
            "default" => states::Workflow::default,
            "qc_only" => states::Workflow::qc_only,
            // Add more options as needed
            _ => return Err("Invalid workflow option".to_string()), // Handle invalid option
        },
        format!("/mnt/input/data/{}/samplesheet_deseq.csv", app_params.project),
        app_params.deseq_model.replace(" ", ""),
        app_params.deseq_ref_var.clone(),
        contrast.clone(),
        utils::parse_de_samplesheet(&app_params.project, contrast.clone(), app_params.deseq_ref_var)?,
        "http://CampbellLab.quickconnect.to/d/f/623389304994967313".to_string(),
    );
    
    
    // put this json file in project folder
    let _ = utils::ftp_put_file(&app_params.project, params.to_key_value_map(), "bulk");
    
    // Build command
    let (tmux_pre, tmux_keys) = utils::build_tmux_command(custom_run_name);
    
    let next_pre = "\"nextflow run /home/carolina/pipelines/NF-RNAseq/main.nf -params-file";
    let rnaseq_cmd = format!("{} \n{} {} /mnt/input/data/{}/nextflowParams.json\" C-m", tmux_pre, tmux_keys, next_pre, &app_params.project);


    let params_map = params.to_key_value_map();
    Ok(PipelineResult { rnaseq_cmd, params_map })
    
}


pub fn parse_sc_params(app_params: states::AppSCParams, username: &str) -> Result<PipelineResult, String> {
    let mut custom_run_name: String;
    if app_params.custom_run_name.is_empty() {
        custom_run_name = "".to_string();
        } else {
        custom_run_name = app_params.custom_run_name;
    }
    custom_run_name = custom_run_name.replace(" ","_");

    let chemistry: String;
    if app_params.chemistry == "true" {
        chemistry = "chromiumV3".to_string();
    } else {
        chemistry = "chromiumV2".to_string();
    }

    let params: states::SCParams = states::SCParams::new(
        custom_run_name.clone(),
        match app_params.organism.as_str() {
            "human" => states::SCGenome::human,
            "mouse" => states::SCGenome::mouse,
            "NHP" => states::SCGenome::NHP,
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
            "" => "200".parse().expect("Not a valid number for minnfeature"),
            _ => app_params.minnfeature.parse().expect("Not a valid number for minnfeature"),
        },
        match app_params.maxnfeature.as_str() {
            "" => "5000".parse().expect("Not a valid number for maxnfeature"),
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
        match app_params.regress.as_str().trim() {
            "" => None,
            s => Some(s.to_string()),
        },
        match app_params.custom_annotations.as_str().trim() {
            "" => None, 
            s => Some(format!("\"{}\"", s)),
        },
    );

    let _ = utils::ftp_put_file(&app_params.project, params.to_key_value_map(), "single_cell");

    
    // Build command
    let (tmux_pre, tmux_keys) = utils::build_tmux_command(custom_run_name);
    
    let next_pre = "\"nextflow run /home/carolina/pipelines/NF-scRNAseq/main.nf -params-file";
    let rnaseq_cmd = format!("{} \n{} {} /mnt/input/data_singlecell/{}/nextflowParams.json\" C-m", tmux_pre, tmux_keys, next_pre, &app_params.project);
    
    let params_map = params.to_key_value_map();
    Ok(PipelineResult { rnaseq_cmd, params_map })
}
