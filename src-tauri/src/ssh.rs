use ssh2::Session;
use std::net::TcpStream;
use std::io::prelude::*;
use std::borrow::Cow;
use std::env;
use ssh_jumper::{
    model::{AuthMethod, HostAddress, HostSocketParams, JumpHostAuthParams, SshTunnelParams},
    SshJumper,
};
use serde::Deserialize;
use trust_dns_resolver::{Resolver, config::ResolverConfig, config::ResolverOpts};

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
    // Specify custom DNS server addresses
    let custom_dns_servers = vec!["134.226.251.100".parse().unwrap(), "134.226.251.200".parse().unwrap()];

    // Create resolver configuration with custom DNS servers
    let resolver_config = ResolverConfig::from_parts(None, custom_dns_servers, vec![]);

    // Create resolver options
    let resolver_opts = ResolverOpts::default();

    // Create resolver instance
    let _resolver = Resolver::new(resolver_config, resolver_opts).map_err(|e| format!("Failed to create DNS resolver: {}", e))?;

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
             .with_local_port(3306);
        
         SshJumper::open_tunnel(&ssh_params).await.unwrap()
     };
     assert_eq!("127.0.0.1:3306".to_string(), local_socket_addr.to_string());
     
    let output = run_ssh_command_locally(local_socket_addr, target_host_username, target_host_password, command).await?;
    println!("{}", output);
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
    
        // Open a channel for executing commands
        let mut channel = sess.channel_session()
            .map_err(|e| format!("Failed to open SSH channel: {}", e))?;

        println!("{}", command);
        // Execute the command
        // Put in a timer in case of failure, especially because tmux stays alive
        // change "ls tmux-3.3a" back to command when implemented
        channel.exec(command)
            .map_err(|e| format!("Failed to execute command over SSH: {}", e))?;

        // Read command output from the channel
        let mut output = String::new();
        channel.read_to_string(&mut output)
            .map_err(|e| format!("Failed to read command output from SSH channel: {}", e))?;

        let _ = channel.wait_close();  // Close connection
        drop(sess);

        Ok(output)
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
    let ssh_auth_server:String = env::var("SSH_AUTH_SERVER").expect("SSH_JUMPHOST_IP must be set in .env (i.e. localhost)"); 
    let ssh_jumphost_user:String = env::var("SSH_JUMPHOST_USER").expect("SSH_JUMPHOST_USER must be set in .env (i.e. user)"); 
    let ssh_jumphost_pass:String = env::var("SSH_JUMPHOST_PASS").expect("SSH_JUMPHOST_PASS must be set in .env (i.e. password)"); 
    
    let command = format!("docker run -d -v /mnt/output/single_cell_RNAseq/{}/:/data \
    -p 5005:5005 cellxgene launch \
    --host 0.0.0.0 data/{} \
    --annotations-dir data/{}_new_annotations",
    params.project, params.h5_file, params.project);

    let sess = match ssh_connect(ssh_auth_server, ssh_jumphost_user, ssh_jumphost_pass) {
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
    let ssh_auth_server:String = env::var("SSH_AUTH_SERVER").expect("SSH_JUMPHOST_IP must be set in .env (i.e. localhost)"); 
    let ssh_jumphost_user:String = env::var("SSH_JUMPHOST_USER").expect("SSH_JUMPHOST_USER must be set in .env (i.e. user)"); 
    let ssh_jumphost_pass:String = env::var("SSH_JUMPHOST_PASS").expect("SSH_JUMPHOST_PASS must be set in .env (i.e. password)"); 

    let mut sess = match ssh_connect(ssh_auth_server, ssh_jumphost_user, ssh_jumphost_pass) {
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