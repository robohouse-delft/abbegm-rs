use abbegm::tokio_peer::EgmPeer;
use std::convert::TryInto;
use std::time::Instant;
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

	/// Radius of the cirle, in meters.
	#[structopt(long)]
	#[structopt(value_name = "M")]
	#[structopt(default_value = "0.10")]
	radius: f64,

	/// Speed of the robot, in meters per second.
	#[structopt(long)]
	#[structopt(value_name = "M/S")]
	#[structopt(default_value = "0.10")]
	speed: f64,

	/// Confirm that the robot should perform motion.
	#[structopt(long)]
	confirm_motion: bool,
}

fn rotate_z(angle: f64) -> nalgebra::Rotation3<f64> {
	nalgebra::Rotation3::from_axis_angle(&nalgebra::Vector3::z_axis(), angle)
}

async fn do_main(options: Options) -> Result<(), String> {
	if !options.confirm_motion {
		return Err(String::from("refusing to send motion commands to the robot without --confirm-motion flag"))
	}

	let mut peer = EgmPeer::bind(&options.bind).await
		.map_err(|e| format!("failed to bind to local enpoint {}: {}", options.bind, e))?;

	let local_address = peer.socket().local_addr()
		.map_err(|e| format!("failed to get local socket address: {}", e))?;

	eprintln!("Listening for messages on {}", local_address);

	let (state, _address) = peer.recv_from().await
		.map_err(|e| format!("failed to receive robot state: {}", e))?;

	eprintln!("Received initial robot state.");

	// Retrieve start pose and compute center of circle.
	let start_time = Instant::now();
	let start_pose : nalgebra::Isometry3<f64> = state.feedback_pose().ok_or("state did not contain a pose")?
		.try_into().map_err(|e| format!("failed to convert pose to isometry: {}", e))?;
	let circle_center = start_pose * nalgebra::Translation3::new(-options.radius * 1e3, 0.0, 0.0);

	let mut sequence_number = 0u32;
	let angular_velocity = options.speed / options.radius;

	loop {
		let (state, address) = peer.recv_from().await
			.map_err(|e| format!("failed to receive robot state: {}", e))?;

		let time = state.feedback_time().ok_or("missing feedback.clock in robot message")?;
		println!("Received robot state message from {}:", address);

		// Compute new pose along the circle.
		let elapsed = start_time.elapsed().as_secs_f64();
		let offset  = rotate_z(elapsed * angular_velocity) * nalgebra::Vector3::new(options.radius * 1e3, 0.0, 0.0);
		let target  = circle_center * nalgebra::Translation::from(offset);

		peer.send_to(&abbegm::msg::EgmSensor::pose_target(sequence_number, target, time), &address).await
			.map_err(|e| format!("failed to send message to robot: {}", e))?;
		sequence_number = sequence_number.wrapping_add(1);
	}
}

#[tokio::main]
async fn main() {
	if let Err(e) = do_main(Options::from_args()).await {
		eprintln!("Error: {}", e);
		std::process::exit(1);
	}
}
