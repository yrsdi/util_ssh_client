use ssh2::Session;
use std::error::Error;
use std::io::Read;
use tokio::net::TcpStream;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::ToSocketAddrs;

#[derive(Serialize, Deserialize)]
struct SystemUtilization {
    cpu: String,
    memory: String,
    disk: String,
}

async fn execute_command(session: &Session, command: &str) -> Result<String, Box<dyn Error>> {
    let mut channel = session.channel_session()?;
    channel.exec(command)?;
    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    channel.wait_close()?;
    Ok(output.trim().to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let username = "your_username"; // Replace with your SSH username
    let password = "your_password"; // Replace with your SSH password
    let host = "your_server_ip"; // Replace with your server IP
    let port = 22;

    // Attempt to resolve the host
    println!("Attempting to resolve host: {}", host);
    match (host, port).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                println!("Successfully resolved: {:?}", addr);
            } else {
                println!("No addresses found for the host");
                return Ok(());
            }
        }
        Err(e) => {
            println!("Failed to resolve host: {:?}", e);
            println!("Please check your internet connection and DNS settings.");
            return Ok(());
        }
    }

    // Connect to the server
    println!("Connecting to {}:{}...", host, port);
    let tcp = match TcpStream::connect((host, port)).await {
        Ok(stream) => stream,
        Err(e) => {
            println!("Failed to connect: {:?}", e);
            println!("Please check if the server is reachable and the port is correct.");
            return Ok(());
        }
    };
    let tcp_std = tcp.into_std()?;

    // Create SSH session
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp_std);
    session.handshake()?;

    // Authenticate using password
    session.userauth_password(username, password)?;

    // Commands to get CPU, memory, and disk utilization
    let cpu_command = "top -bn1 | grep 'Cpu(s)' | awk '{print $2 + $4}'";
    let mem_command = "free | grep Mem | awk '{print $3/$2 * 100.0}'";
    let disk_command = "df -h / | awk 'NR==2 {print $5}'";

    // Execute commands
    let cpu_usage = execute_command(&session, cpu_command).await?;
    let mem_usage = execute_command(&session, mem_command).await?;
    let disk_usage = execute_command(&session, disk_command).await?;

    // Create SystemUtilization struct
    let utilization = SystemUtilization {
        cpu: format!("{}%", cpu_usage),
        memory: format!("{:.2}%", mem_usage.parse::<f64>()?),
        disk: disk_usage,
    };

    // Convert to JSON
    let json_result = json!(utilization);

    // Print JSON result
    println!("{}", serde_json::to_string_pretty(&json_result)?);

    Ok(())
}
