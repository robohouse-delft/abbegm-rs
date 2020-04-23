use abbegm::msg;
use abbegm::tokio_peer::EgmPeer;
use std::time::Instant;
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
	use std::f64::consts::PI;

	eprintln!("{:#?}", options);
	let socket = UdpSocket::bind(&options.bind).await
		.map_err(|e| format!("failed to bind to local enpoint {}: {}", options.bind, e))?;
	socket.connect(&options.robot).await
		.map_err(|e| format!("failed to connect to remote enpoint {}: {}", options.robot, e))?;
	let mut peer = EgmPeer::new(socket);

	let mut sequence_number = 0u32;
	let start = Instant::now();

	let radians_per_second = options.speed / options.radius;

	loop {
		let robot_state = peer.recv().await
			.map_err(|e| format!("failed to receive robot state: {}", e))?;

		let feedback = robot_state.feed_back.ok_or(format!("robot state missing feedback field"))?;

		let position = feedback.cartesian.ok_or(format!("robot state missing feedback.cartesian field"))?;

		let elapsed = start.elapsed().as_secs_f64();
		let angle = (elapsed * radians_per_second) % (2.0 * PI);

		let command = msg::EgmSensor {
			header: Some(msg::EgmHeader {
				seqno: Some(sequence_number),
				tm: None,
				mtype: Some(msg::egm_header::MessageType::MsgtypeCorrection as i32),
			}),
			planned: Some(msg::EgmPlanned {
				time: feedback.time,
				cartesian: Some(msg::EgmPose {
					pos: Some(msg::EgmCartesian { x: 0.0, y: 0.0, z: 0.0 }),
					orient: Some(msg::EgmQuaternion { u0: 1.0, u1: 0.0, u2: 0.0, u3: 0.0 }),
					euler: None,
				}),
				joints: None,
				external_joints: None,
			}),
			speed_ref: Some(msg::EgmSpeedRef {
				joints: Some(msg::EgmJoints {
					joints: vec![0.01; 6],
				}),
				cartesians: Some(msg::EgmCartesianSpeed {
					value: vec![0.01; 6],
				}),
				external_joints: None,
			}),
		};

		peer.send(&command).await
			.map_err(|e| format!("failed to send robot command: {}", e))?;

		sequence_number = sequence_number.overflowing_add(1).0;
	}
}

#[tokio::main]
async fn main() {
	if let Err(e) = do_main(Options::from_args()).await {
		eprintln!("Error: {}", e);
		std::process::exit(1);
	}
}
