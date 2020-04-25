use abbegm::tokio_peer::EgmPeer;
use structopt::StructOpt;
use structopt::clap::AppSettings;

#[derive(Debug, StructOpt)]
#[structopt(setting = AppSettings::ColoredHelp)]
#[structopt(setting = AppSettings::DeriveDisplayOrder)]
#[structopt(setting = AppSettings::UnifiedHelpMessage)]
struct Options {
	/// Local address to bind to.
	#[structopt(long)]
	#[structopt(value_name = "HOST:PORT")]
	#[structopt(default_value = "[::]:6510")]
	bind: String,
}

async fn do_main(options: Options) -> Result<(), String> {
	let mut peer = EgmPeer::bind(&options.bind).await
		.map_err(|e| format!("failed to bind to local enpoint {}: {}", options.bind, e))?;

	let local_address = peer.socket().local_addr().map_err(|e| format!("failed to get local socket address: {}", e))?;
	eprintln!("Listening for messages on {}", local_address);

	loop {
		let (state, address) = peer.recv_from().await
			.map_err(|e| format!("failed to receive robot state: {}", e))?;
		println!("Received EGM message from {}:\n{:#?}", address, state);
	}
}

#[tokio::main]
async fn main() {
	if let Err(e) = do_main(Options::from_args()).await {
		eprintln!("Error: {}", e);
		std::process::exit(1);
	}
}
