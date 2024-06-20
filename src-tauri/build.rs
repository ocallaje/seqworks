use std::env;
use std::fs::File;
use std::io::Write;
use dotenvy::dotenv;

fn main() {
    tauri_build::build();

    dotenv().ok();

    let ssh_jumphost = env::var("SSH_JUMPHOST").expect("SSH_JUMPHOST not set");
    let ssh_jumphost_user = env::var("SSH_JUMPHOST_USER").expect("SSH_JUMPHOST_USER not set");
    let ssh_jumphost_pass = env::var("SSH_JUMPHOST_PASS").expect("SSH_JUMPHOST_PASS not set");
    let ftp_server = env::var("FTP_SERVER").expect("FTP_SERVER not set");

    //let out_dir = "src-tauri/src/".to_string();
    //let dest_path = Path::new(&out_dir).join("env_vars.rs");
    let mut file = File::create("env_vars.rs").unwrap();

    writeln!(file, "pub const SSH_JUMPHOST: &str = \"{}\";", ssh_jumphost).unwrap();
    writeln!(file, "pub const SSH_JUMPHOST_USER: &str = \"{}\";", ssh_jumphost_user).unwrap();
    writeln!(file, "pub const SSH_JUMPHOST_PASS: &str = \"{}\";", ssh_jumphost_pass).unwrap();
    writeln!(file, "pub const FTP_SERVER: &str = \"{}\";", ftp_server).unwrap();
}
