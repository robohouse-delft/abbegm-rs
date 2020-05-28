//! Run this from the repository root to generate Rust code from the protobuf files.

use std::path::Path;

fn main() {
	let stat = match Path::new("../proto/egm.proto").metadata() {
		Ok(x) => x,
		Err(e) => {
			eprintln!("Failed to inspect `../proto/egm.proto: {}", e);
			eprintln!("Make sure to run this program from `abbegm-rs/generated.");
			std::process::exit(1);
		}
	};

	if !stat.is_file() {
		eprintln!("../proto/egm.proto is not a file.");
		eprintln!("Make sure to run this program from `abbegm-rs/generated.");
		std::process::exit(1);
	}

	std::env::set_var("OUT_DIR", "../src/generated");
	prost_build::compile_protos(&["../proto/egm.proto"], &["../proto"]).unwrap()
}
