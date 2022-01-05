use adb_client::{AdbCommandProvider, AdbTcpConnexion, Device};
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Sets the listening address of ADB server
    #[clap(short = 'a', long = "address", default_value = "127.0.0.1")]
    pub address: String,
    /// Sets the listening port of ADB server
    #[clap(short = 'p', long = "port", default_value = "5037")]
    pub port: u16,
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Prints current ADB version
    Version,
    /// Asks ADB server to quit immediately
    Kill,
    /// List connected devices
    Devices {
        #[clap(short = 'l', long = "long")]
        long: bool,
    },
    /// Tracks new devices showing up
    TrackDevices,
}

fn main() -> Result<()> {
    let opt = Args::parse();

    let connexion = AdbTcpConnexion::new().address(opt.address)?.port(opt.port);

    match opt.command {
        Command::Version => {
            let version = connexion.version()?;

            println!("Android Debug Bridge version {}", version);
            println!("Package version {}-rust", std::env!("CARGO_PKG_VERSION"));
        }
        Command::Kill => {
            connexion.kill()?;
        }
        Command::Devices { long } => {
            if long {
                println!("List of devices attached (long)");
                connexion.devices_long()?;
            } else {
                println!("List of devices attached");
                for device in connexion.devices()? {
                    println!("{}\t{}", device.identifier, device.state);
                }
            }
        }
        Command::TrackDevices => {
            let callback = |device: Device| {
                println!("{}", device);
                Ok(())
            };
            println!("Live list of devices attached");
            connexion.track_devices(callback)?;
        }
    }

    Ok(())
}