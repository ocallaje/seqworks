use ssh2::Session;
use std::net::TcpStream;
use std::borrow::Cow;
use ssh_jumper::{
    model::{AuthMethod, HostAddress, HostSocketParams, JumpHostAuthParams, SshTunnelParams},
    SshJumper,
};
use serde::Deserialize;
use trust_dns_resolver::{Resolver, config::ResolverConfig, config::ResolverOpts};
use std::io::Read;
include!(concat!("../env_vars.rs"));

// you need to set 
// AllowAgentForwarding yes 
// AllowTcpForwarding yes 
// PermitTunnel yes
// PermitOpen any
// TCPKeepAlive yes
//in the /etc/ssh/sshd_config file of the jump host to allow it to redirect ssh tunnel 

pub async fn run_ssh_command_via_jump(
    jump_host_address: &str,
    jump_host_port: u16, 
    jump_host_username: &str,
    jump_host_password: &str,
    target_host_address: &str,
    target_host_port: u16,
    target_host_username: &str,
    target_host_password: &str,
    command: &str,
) -> Result<String, String> {
    // Specify DNS addresses
    let custom_dns_servers = vec![
        "134.226.251.100".parse().unwrap(),
        "134.226.251.200".parse().unwrap(),
    ];
    let resolver_config = ResolverConfig::from_parts(None, custom_dns_servers, vec![]);
    let resolver_opts = ResolverOpts::default();

    // Move resolver creation to a blocking context
    let _resolver = Resolver::new(resolver_config, resolver_opts)
        .map_err(|e| format!("Failed to create DNS resolver: {}", e))?;


    let (local_socket_addr, _ssh_forwarder_end_rx) = {
        let jump_host = HostAddress::HostName(Cow::Borrowed(jump_host_address));
         let jump_host_auth_params = JumpHostAuthParams {
             user_name: Cow::Borrowed(jump_host_username),
             auth_method: AuthMethod::Password {
                 password: Cow::Borrowed(jump_host_password),
             },
         };
         
         let target_socket = HostSocketParams {
             address: HostAddress::HostName(Cow::Borrowed(target_host_address)),
             port: target_host_port,
         };
         
         let ssh_params = SshTunnelParams::new(jump_host, jump_host_auth_params, target_socket)
             .with_jump_host_port(jump_host_port)
             .with_local_port(4066);
         
         
        SshJumper::open_tunnel(&ssh_params).await.unwrap()
     };
     assert_eq!("127.0.0.1:4066".to_string(), local_socket_addr.to_string());
     
    let output = run_ssh_command_locally(local_socket_addr, target_host_username, target_host_password, command).await?;
    println!("{:?}", output);
    Ok(local_socket_addr.to_string())
}

// Function to run SSH command using the local socket address
 async fn run_ssh_command_locally(local_socket_addr: std::net::SocketAddr, target_host_username:&str, 
    target_host_password:&str, command: &str) -> Result<String, String> {
   
             
        let sess = match ssh_connect(local_socket_addr.to_string(), target_host_username.to_string(), target_host_password.to_string()) {
            Ok(sess) => sess,
            Err(e) => {
               let err_msg = format!("Failed to connect to SSH server: {}", e);
                eprintln!("{}", &err_msg);
                return Err(err_msg);
            }
        };      
    
        println!("Opening tunnell channel");
         //Open a channel for executing commands
        let mut channel = sess.channel_session()
            .map_err(|e| format!("Failed to open SSH channel: {}", e))?;

        println!("passing command");
         //Execute the command
         //Put in a timer in case of failure, especially because tmux stays alive
         //change "ls tmux-3.3a" back to command when implemented
        channel.exec(command)
            .map_err(|e| format!("Failed to execute command over SSH: {}", e))?;

        println!("reading output");
         //Read command output from the channel
        let mut output = String::new();
        channel.read_to_string(&mut output)
           .map_err(|e| format!("Failed to read command output from SSH channel: {}", e))?;

        println!("closing channel");
        let _ = channel.wait_close();  // Close connection
        println!("dropping session");
        drop(sess);

        println!("all done");
        Ok("all done".to_string())
   
 }


pub async fn ssh_chain(rnaseq_cmd: &str) -> Result<i32, String> {

    let err_val:i32 = 0;

    // Connect to the intermediary host
    let sess = match ssh_connect(SSH_JUMPHOST.to_string(), SSH_JUMPHOST_USER.to_string(), SSH_JUMPHOST_PASS.to_string()) {
        Ok(sess) => Ok(sess),
        Err(e) => {
            let err_msg = format!("Failed to connect to SSH server: {}", e);
            eprintln!("{}", &err_msg);  
            Err(err_val)
        }
    }.unwrap();
    
    // Execute SSH command on the final destination server via the intermediary host
    println!("Executing SSH command on final destination...");
    let mut channel = sess.channel_session().unwrap();
    let command = format!("ssh reaper {}", rnaseq_cmd);
    println!("{}", command);

    if let Err(err) = channel.exec(&command) {
        eprintln!("Error executing command: {}", err);
        
    }

    // Capture and print stdout and stderr
    let mut stdout = String::new();
    let mut stderr = String::new();

    channel.read_to_string(&mut stdout).unwrap();
    channel.stderr().read_to_string(&mut stderr).unwrap();

    println!("Command output: {}", stdout);
    println!("Command error output: {}", stderr);

    // Check the exit status
    let exit_status = channel.exit_status().unwrap();
    println!("Command exit status: {}", exit_status);

    // Close the channel
    channel.send_eof().unwrap();
    channel.wait_close().unwrap();

    Ok(exit_status)

 }


 pub fn ssh_authenticate(user:String, password:String, ip_address: String) -> Result<String, String> {
    let sess = match ssh_connect(ip_address, user, password) {
        Ok(sess) => sess,
        Err(e) => {
            let err_msg = format!("Failed to connect to SSH server: {}", e);
            eprintln!("{}", &err_msg);
            return Err(err_msg);
        }
    };
    drop(sess);  // Close connection
    Ok("Auth Success".to_string())
 }

 #[derive(Debug, Deserialize)]
 pub struct CxgParams {
    project: String,
    h5_file: String,
}

 pub async fn start_cellxgene(params: CxgParams) -> Result<(), String> {
    
    let command = format!("docker run -d -v /mnt/output/single_cell_RNAseq/{}/:/data \
    -p 5005:5005 cellxgene launch \
    --host 0.0.0.0 data/{} \
    --annotations-dir data/{}_new_annotations",
    params.project, params.h5_file, params.project);

    let sess = match ssh_connect(SSH_JUMPHOST.to_string(), SSH_JUMPHOST_USER.to_string(), SSH_JUMPHOST_PASS.to_string()) {
        Ok(sess) => sess,
        Err(e) => {
            let err_msg = format!("Failed to connect to SSH server: {}", e);
            eprintln!("{}", &err_msg);
            return Err(err_msg);
        }
    };

    println!("Viper connected, sending cellxgene launch command");
    let mut channel = sess.channel_session()
        .map_err(|e| format!("Failed to open SSH channel: {}", e))?;
    channel.exec(&command)
        .map_err(|e| format!("Failed to execute command over SSH: {}", e))?;

    let _ = channel.wait_close();  // Close connection
    drop(sess);

    Ok(())

 }

 pub async fn stop_cellxgene(params: CxgParams) -> Result<(), String> {
    
    let mut sess = match ssh_connect(SSH_JUMPHOST.to_string(), SSH_JUMPHOST_USER.to_string(), SSH_JUMPHOST_PASS.to_string()) {
        Ok(sess) => sess,
        Err(e) => {
            let err_msg = format!("Failed to connect to SSH server: {}", e);
            eprintln!("{}", &err_msg);
            return Err(err_msg);
        }
    };

    // Define a helper function to execute commands over SSH
    fn execute_ssh_command(session: &mut Session, command: &str) -> Result<(), String> {
        let mut channel = session.channel_session().map_err(|e| format!("Failed to open SSH channel: {}", e))?;
        channel.exec(command).map_err(|e| format!("Failed to execute SSH command: {}", e))?;
        Ok(())
    }

    println!("Viper connected, sending cellxgene stop commands");
    // Get cellxgene container ID
    execute_ssh_command(&mut sess, &format!("container_id=$(docker ps -qf \"ancestor=cellxgene\")"))?;

    // Copy Annotations from docker to output server
    execute_ssh_command(&mut sess, &format!("docker cp \"$container_id:/data/{}_new_annotations/\" /mnt/output/single_cell_RNAseq/{}/cellxgene/", 
    params.project, params.project))?;

    // Stop the cellxgene container
    execute_ssh_command(&mut sess, &format!("docker stop $container_id"))?;
    
    drop(sess);  // Close connection

    Ok(())

 }


 fn ssh_connect(address: String, user: String, pass: String) -> Result<Session, String> {
    // Connect to the SSH Tunnel
    let tcp = TcpStream::connect(address).map_err(|e| format!("Failed to connect to host: {}", e))?;
    let mut sess = Session::new().map_err(|e| format!("Failed to create session: {}", e))?;
    sess.set_tcp_stream(tcp); // Associate the TcpStream with the Session
    sess.handshake().map_err(|e| format!("Failed to perform handshake: {}", e))?;

    // Authenticate
    sess.userauth_password(&user, &pass)
        .map_err(|e| format!("Failed to authenticate: {}", e))?;
    
    // Check if authentication succeeded
    if !sess.authenticated() {
        return Err("Authentication failed".to_string());
    }
    Ok(sess)
 }