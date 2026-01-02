use anyhow::{bail, Context, Result};
use clap::Parser;
use ipnetwork::IpNetwork;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::timeout;

#[derive(Parser, Debug)]
#[command(
    name = "ipscan",
    about = "High-performance IP scanner that detects live hosts in a CIDR range"
)]
struct Args {
    /// CIDR range to scan, e.g. 192.168.64.0/20
    cidr: String,
    /// Timeout in milliseconds per probe
    timeout_ms: u64,
    /// Maximum number of concurrent probes
    #[arg(short, long, default_value_t = 512)]
    concurrency: usize,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();
    let cidr: IpNetwork = args.cidr.parse().context("invalid CIDR")?;

    let v4 = match cidr {
        IpNetwork::V4(net) => net,
        IpNetwork::V6(_) => bail!("IPv6 is not supported yet"),
    };

    let timeout = Duration::from_millis(args.timeout_ms);
    let semaphore = Arc::new(Semaphore::new(args.concurrency));

    let mut handles = Vec::new();
    for ip in v4.iter() {
        let permit = semaphore.clone().acquire_owned().await?;
        let timeout = timeout;
        handles.push(tokio::spawn(async move {
            let _permit = permit;
            if is_live(IpAddr::V4(ip), timeout).await {
                println!("{ip}");
            }
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}

async fn is_live(ip: IpAddr, timeout_duration: Duration) -> bool {
    let ports: [u16; 4] = [80, 443, 22, 445];

    for port in ports {
        match timeout(timeout_duration, TcpStream::connect((ip, port))).await {
            Ok(Ok(_stream)) => return true,
            Ok(Err(err)) => {
                if err.kind() == std::io::ErrorKind::ConnectionRefused {
                    return true;
                }
            }
            Err(_) => continue,
        }
    }

    false
}
