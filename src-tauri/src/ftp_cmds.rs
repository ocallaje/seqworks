use std::str;
use std::io::Cursor;
use suppaftp::FtpStream;

pub fn parse_de_samplesheet(project: &str, contrast_var: String, ref_var: String) -> Result<String, String> {
    let mut ftp_stream = match ftp_connect_and_login() {
        Ok(stream) => stream,
        Err(e) => return Err(e),
    };

    // Change into a new directory, relative to the one we are currently in.
    let project_dir = format!("RNAseq_datasets/data/{}", project);
    let _ = ftp_stream.cwd(project_dir).unwrap();
    println!("Current directory: {}", ftp_stream.pwd().unwrap());

    // Retrieve (GET) a file from the FTP server in the current working directory.
    let data = ftp_stream.retr_as_buffer("samplesheet_deseq.csv").unwrap();
    let binding = data.into_inner();
    let csv_data = str::from_utf8(&binding).map_err(|e| e.to_string())?;

    // Terminate the connection to the server.
    let _ = ftp_stream.quit();

    // Parse CSV data
    let mut rdr = csv::Reader::from_reader(csv_data.as_bytes());
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


pub fn get_dirs(pipe_type: &str) -> Result<Vec<String>,String> {
    let mut ftp_stream = match ftp_connect_and_login() {
        Ok(stream) => stream,
        Err(e) => return Err(e),
    };

    // Change into a new directory, relative to the one we are currently in.
    let project_dir: String = match pipe_type {
        "bulk" => String::from("RNAseq_datasets/data/"),
        "single_cell" => String::from("RNAseq_datasets/data_singlecell/"),
        _ => {
            eprintln!("incompatible directory");
            String::from("default_directory/")
        }
    };
    let _ = ftp_stream.cwd(project_dir).unwrap();

    // List all entries in the current directory.
    let entries = match ftp_stream.nlst(None) {
        Ok(entries) => entries,
        Err(e) => {
            let err_msg = format!("Failed to list directory contents: {}", e);
            eprintln!("{}", &err_msg);
            return Err(err_msg);
        }
    };

    println!("Current directory: {:?}", entries);
    
    let _ = ftp_stream.quit();  // Terminate the connection to the server.

    Ok(entries)
}

pub fn ftp_put_file(project: &str, params_map: serde_json::Map<String, serde_json::Value>) -> Result<u64, String> {
    let mut ftp_stream = match ftp_connect_and_login() {
        Ok(stream) => stream,
        Err(e) => return Err(e),
    };
    let project_dir = format!("RNAseq_datasets/data/{}", project);
    let _ = ftp_stream.cwd(project_dir).unwrap();

    // PUT file to the current working directory of the server.
    let json_data = serde_json::to_string_pretty(&params_map);             // Serialize the struct to JSON
    let mut reader = Cursor::new(json_data.unwrap().into_bytes());   // Convert JSON data to Cursor
    let bytes_uploaded = ftp_stream.put_file("nextflowParams.json", &mut reader);
    println!("Successfully wrote JSON parameters");

    let _ = ftp_stream.quit();  // Terminate the connection to the server.

    Ok(bytes_uploaded.unwrap())
}

fn ftp_connect_and_login() -> Result<FtpStream, String> {
    let mut ftp_stream = match FtpStream::connect("192.168.1.9:21") {
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