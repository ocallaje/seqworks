use ssh2::Session;
use std::net::TcpStream;
use std::io::prelude::*;
use std::borrow::Cow;
use ssh_jumper::{
    model::{AuthMethod, HostAddress, HostSocketParams, JumpHostAuthParams, SshTunnelParams},
    SshJumper,
};

// you need to set 
// AllowAgentForwarding yes 
// AllowTcpForwarding yes 
// PermitTunnel yes
// PermitOpen any
// TCPKeepAlive yes
//in the /etc/ssh/sshd_config file of the jump host to allow it to redirect ssh tunnel 

/* pub async fn run_ssh_command(host: String, username: String, password: String, command: String) -> Result<String, String> {
    task::spawn_blocking(move || {
        // Connect to the remote server
        let tcp = TcpStream::connect(host).map_err(|e| format!("Failed to connect to host: {}", e))?;
        let mut sess = Session::new().map_err(|e| format!("Failed to create session: {}", e))?;
        sess.set_tcp_stream(tcp); // Associate the TcpStream with the Session
        sess.handshake().map_err(|e| format!("Failed to perform handshake: {}", e))?;
        
        println!("SSH handshake succeeded");

        // Authenticate
        sess.userauth_password(&username, &password).map_err(|e| format!("Failed to authenticate: {}", e))?;
        
        // Check if authentication succeeded
        if !sess.authenticated() {
            return Err("Authentication failed".to_string());
        }

        println!("SSH authentication succeeded");

        // Execute the command
        
        let mut channel = sess.channel_session().map_err(|e| format!("Failed to open channel: {}", e))?;
        //let full_command = format!("ssh -o StrictHostKeyChecking=no root@172.17.0.4 -p 22 '{}'", command); // orig
        let full_command = format!("ssh -o StrictHostKeyChecking=no root@172.17.0.4 -p 22 {}", "ls tmux-3.3a");
        //let full_command = format!("tmux new-session -d -s sdsfdf '{}'; tmux capture-pane -pS -3000", command);   // capture pane?
        channel.exec(&full_command).map_err(|e| format!("Failed to execute command: {}", e))?;
        

        println!("Command execution started: {}", full_command);

        // Set a timeout for waiting for EOF state
        let timeout_duration = std::time::Duration::from_secs(5); // Adjust timeout duration as needed
        let start_time = Instant::now();
        //while !channel.eof() {
            // Check if the timeout has been exceeded
        //    if Instant::now() - start_time >= timeout_duration {
        //        return Err("Timeout waiting for command to finish".to_string());
        //    }
        //    std::thread::sleep(std::time::Duration::from_millis(100)); // Sleep for a short duration
        //}

        // Read the output
        let mut s = String::new();
        //channel.read_to_string(&mut s).map_err(|e| format!("Failed to read output: {}", e))?;
        channel.send_eof().map_err(|e| format!("Failed to send EOF: {}", e))?;  
        // Wait for a short duration before closing the channel
        std::thread::sleep(std::time::Duration::from_millis(100));
        channel.wait_close().map_err(|e| format!("Failed to close channel: {}", e))?;
        let exit_status = channel.exit_status().map_err(|e| format!("Failed to get exit status: {}", e))?;

        println!("Command execution finished with exit status: {}", exit_status);

        if exit_status == 0 {
            Ok(s)
        } else {
            Err(format!("Command executed with exit status: {}", exit_status))
        }
    })
    .await
    .unwrap_or_else(|e| Err(format!("Failed to run command in async context: {}", e)))
}
*/

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
     println!("{}", local_socket_addr);
    let output = run_ssh_command_locally(local_socket_addr, target_host_username, target_host_password, command).await?;
    println!("{}", output);
    Ok(local_socket_addr.to_string())
}

// Function to run SSH command using the local socket address
 async fn run_ssh_command_locally(local_socket_addr: std::net::SocketAddr, target_host_username:&str, target_host_password:&str, command: &str) -> Result<String, String> {
    // Connect to the SSH Tunnel
    let tcp = TcpStream::connect(local_socket_addr).map_err(|e| format!("Failed to connect to host: {}", e))?;
    let mut sess = Session::new().map_err(|e| format!("Failed to create session: {}", e))?;
    sess.set_tcp_stream(tcp); // Associate the TcpStream with the Session
    sess.handshake().map_err(|e| format!("Failed to perform handshake: {}", e))?;

    // Authenticate
    sess.userauth_password(target_host_username, target_host_password).map_err(|e| format!("Failed to authenticate: {}", e))?;
    
    // Check if authentication succeeded
    if !sess.authenticated() {
        return Err("Authentication failed".to_string());
    }

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

    Ok(output)
 }

 pub fn ssh_authenticate(user:String, password:String, ip_address: String) -> Result<String, String> {
    // Connect to the SSH Tunnel
    let tcp = TcpStream::connect(ip_address).map_err(|e| format!("Failed to connect to host: {}", e))?;
    let mut sess = Session::new().map_err(|e| format!("Failed to create session: {}", e))?;
    sess.set_tcp_stream(tcp); // Associate the TcpStream with the Session
    sess.handshake().map_err(|e| format!("Failed to perform handshake: {}", e))?;

    // Authenticate
    sess.userauth_password(&user, &password).map_err(|e| format!("Failed to authenticate: {}", e))?;
    
    // Check if authentication succeeded
    if !sess.authenticated() {
        return Err("Authentication failed".to_string());
    }

    Ok("Auth Success".to_string())
 }