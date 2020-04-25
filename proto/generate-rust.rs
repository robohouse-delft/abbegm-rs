//! Run this from the repository root to generate Rust code from the protobuf files.

fn main() {
	std::env::set_var("OUT_DIR", "src/generated");
	prost_build::compile_protos(&["proto/egm.proto"], &["proto"]).unwrap()
}
