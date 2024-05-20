pub struct BulkParams {
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
    workflow: String,
    deseq2SampleSheet: String,
    deseq2Model: String,
    deseq2ReferenceVar: String,
    deseq2ContrastVar: String, 
    deseq2TargetVar: String,
    synology_link: String,
}

//impl Default for BulkParams {
//    fn default() -> Self {
//        BulkParams(
//            // Default values for each field
//            "default_input".to_string(),  // input
//            "default_outdir".to_string(),  // outdir
//            // Other default values...
//        )
//    }
//}