use abbegm::tokio_peer::EgmPeer;
use structopt::StructOpt;
use structopt::clap::AppSettings;
use tokio::net::UdpSocket;

#[derive(Debug, StructOpt)]
#[structopt(setting = AppSettings::ColoredHelp)]
#[structopt(setting = AppSettings::DeriveDisplayOrder)]
#[structopt(setting = AppSettings::UnifiedHelpMessage)]
struct Options {
	/// Robot hostname or IP address.
	#[structopt(long)]
	#[structopt(value_name = "HOST:PORT")]
	robot: String,

	/// Local address to bind to.
	#[structopt(long)]
	#[structopt(value_name = "HOST:PORT")]
	#[structopt(default_value = "[::]:5693")]
	bind: String,

	/// Circle radius in meters.
	#[structopt(long)]
	#[structopt(default_value = "0.10")]
	radius: f64,

	/// Movement speed in meters per second.
	#[structopt(long)]
	#[structopt(default_value = "0.10")]
	speed: f64,
}

async fn do_main(options: Options) -> Result<(), String> {
	eprintln!("{:#?}", options);
	let socket = UdpSocket::bind(&options.bind).await
		.map_err(|e| format!("failed to bind to local enpoint {}: {}", options.bind, e))?;
	socket.connect(&options.robot).await
		.map_err(|e| format!("failed to connect to remote enpoint {}: {}", options.robot, e))?;
	let _peer = EgmPeer::new(socket);
	Ok(())
}

#[tokio::main]
async fn main() {
	if let Err(e) = do_main(Options::from_args()).await {
		eprintln!("Error: {}", e);
		std::process::exit(1);
	}
}
