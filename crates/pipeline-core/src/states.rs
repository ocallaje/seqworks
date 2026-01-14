use serde_json::{json, Map};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppParamsWrapper {
    pub params: AppParamsEnum,
}

#[derive(Deserialize)]
pub enum AppParamsEnum {
    AppParams(AppParams),
    AppSCParams(AppSCParams),
}

#[derive(serde::Deserialize)]
pub struct AppParams {
  pub illumina_stranded_kit: String,
  pub strandedness: String,
  pub paired_end: String,
  pub trimadaptors: String,
  pub verify: String,
  pub merge_fastqs: String,
  pub send_email: String,
  pub cc: String,
  pub custom_run_name: String,
  pub project: String,
  pub genome: String,
  pub genome_version: String,
  pub workflow: String,
  pub deseq_model: String,
  pub deseq_ref_var: String, 
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
    pub fn to_key_value_map(&self) -> serde_json::Map<String, serde_json::Value> {
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
    pub custom_run_name: String,
    pub project: String,
    pub organism: String,
    pub genome: String,
    pub genome_version: String,
    pub machine: String,
    pub workflow: String,           
    pub demultiplex: String,
    pub permit_method: String,
    pub chemistry: String,          
    pub send_email: String,
    pub cc: String,
    pub minnfeature: String,
    pub maxnfeature: String,
    pub mt: String,
    pub ribo: String,
    pub resolution: String,
    pub pcs: String,
    pub integrate: String, 
    pub nonlinear: String,
    pub identity: String,
    pub condition: String,
    pub annotation_method: String,
    pub regress: String,
    pub custom_annotations: String,
    pub inspect_list: String,
    pub annotation_file: String, 
    pub meta_group: String,
    pub de: String
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
    f32,
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
    Option<String>,
    Option<String>,
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
        resolution: f32,
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
        regress: Option<String>,
        custom_annotations: Option<String>,

    ) -> Self {
        Self(custom_run_name, organism, genome, genome_version, machine, workflow, demultiplex,
            permit_method, chemistry, send_email, cc, minnfeature, maxnfeature, mt, ribo,
            resolution, pcs, integrate, nonlinear, identity, condition, inspect_list, annotation_file,
            meta_group, DE, input, outdir, email, synology_link, publish_dir_mode, scriptDir,
            index_mapping_file, instrument_mapping_file, annotation_method, regress, custom_annotations
        )
    }
    // Method to convert struct into key-value pairs map
    pub fn to_key_value_map(&self) -> serde_json::Map<String, serde_json::Value> {
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
        map.insert("regress".to_string(), json!(self.34));
        map.insert("custom_annotations".to_string(), json!(self.35));
        map
    }
}

#[derive(Debug, serde::Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Workflow {
    default,
    qc_only,
    de_analysis_only,
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