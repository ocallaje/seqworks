use std::{str, fs};
use std::path::PathBuf;
use suppaftp::FtpStream;
use std::fs::File;
include!(concat!("env_vars.rs"));

pub fn parse_de_samplesheet(
    root_dir: &PathBuf,
    project: &str, 
    contrast_var: String, 
    ref_var: String
) -> Result<String, String> {
 
    let csv_path = root_dir
        .join("data")
        .join(project)
        .join("samplesheet_deseq.csv");

    // Open the CSV file
    let file = File::open(&csv_path)
        .map_err(|e| format!("Failed to open {:?}: {}", csv_path, e))?;

    // Create CSV reader
    let mut rdr = csv::Reader::from_reader(file);

    let headers = rdr.headers().map_err(|e| e.to_string())?;
    let mut all_targets = Vec::new();
    let mut contrast_column_index = None;

    // Find the contrast variable column index
    for (i, header) in headers.iter().enumerate() {
        if header == contrast_var {
            contrast_column_index = Some(i);
            break;
        }
    }

    let contrast_column_index = contrast_column_index.ok_or("Contrast variable not found in samplesheet_deseq.csv")?;

    // Collect all unique values from the contrast variable column
    for result in rdr.records() {
        let record = result.map_err(|e| e.to_string())?;
        all_targets.push(record[contrast_column_index].to_string());
    }

    all_targets.sort();
    all_targets.dedup();

    // Validate the reference variable
    let refidx = all_targets.iter().position(|x| x == &ref_var);
    if refidx.is_none() {
        return Err("Reference Variable not found in samplesheet_deseq.csv. Check sheet and check for capital letters.".to_string());
    }

    // Remove the reference variable from the list
    let refidx = refidx.unwrap();
    all_targets.remove(refidx);

    // Construct the target variable string
    let targets = all_targets.join(",");
    let deseq2_target_var = targets.replace(" ", "");

    Ok(deseq2_target_var)

}

pub fn get_dirs(
    data_root: &PathBuf,
    pipe_type: &str,
) -> Result<Vec<String>,String> {

    let mut base = data_root.clone();

    match pipe_type {
        "bulk" => base.push("data"),
        "single_cell" => base.push("data_singlecell"),
        _ => return Err("Invalid pipe_type".to_string()),
    }

    let entries = fs::read_dir(&base)
        .map_err(|e| format!("Failed to read dir {:?}: {}", base, e))?;

    // List all entries in the current directory.
    let dirs = entries
        .filter_map(|entry| entry.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    
    Ok(dirs)
}


pub fn save_nextflow_params(
    root_dir: &PathBuf,
    project: &str, 
    params_map: serde_json::Map<String, serde_json::Value>, 
    pipe_type: &str
) -> Result<u64, String> {
    
    let data_dir: String = match pipe_type {
        "bulk" => String::from("RNAseq_datasets/data/"),
        "single_cell" => String::from("RNAseq_datasets/data_singlecell/"),
        _ => {
            eprintln!("incompatible directory");
            String::from("default_directory/")
        }
    };

    let project_dir = root_dir
        .join(data_dir)
        .join(project);
    
    let file_path = project_dir.join("nextflowParams.json");

    let json_data = serde_json::to_string_pretty(&params_map).unwrap();       // Serialize the struct to JSON
    // write to disk
    fs::write(&file_path, json_data.as_bytes())
        .map_err(|e| format!("Failed to write file {:?}: {}", file_path, e))?; 

    println!("Successfully wrote JSON parameters to {:?}", file_path);


    Ok(json_data.len() as u64)
}


pub fn build_tmux_command(custom_run_name: String) -> (String, String) {
    // Function to create the tmux commands prior to initiating nextflow
    let tmux_pre = format!("tmux new-session -d -s {}", custom_run_name);
    let tmux_keys = format!("tmux send-keys -t {}", custom_run_name);

    (tmux_pre, tmux_keys)
}


fn ftp_connect_and_login() -> Result<FtpStream, String> {
    let mut ftp_stream = match FtpStream::connect(FTP_SERVER) {
        Ok(stream) => stream,
        Err(e) => {
            let err_msg = format!("Failed to connect to FTP server: {}", e);
            eprintln!("{}", &err_msg);
            return Err(err_msg);
        }
    };

    match ftp_stream.login("commonUSER", "Claudin5!") {
        Ok(_) => Ok(ftp_stream),
        Err(e) => {
            let err_msg = format!("Failed to login to FTP server: {}", e);
            eprintln!("{}", &err_msg);
            Err(err_msg)
        }
    }
}