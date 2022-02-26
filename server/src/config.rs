use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use argh::FromArgs;
use serde::{Deserialize, Serialize};

#[derive(Debug, FromArgs)]
/// Server configuration arguments
struct Args {
	/// if specified, puts this server in "follower" mode and adds a "leader" server that will automatically be synchronized with
	#[argh(option, short = 'L')]
	leader_url: Option<String>,
	/// if specified, try to load configuration from this file instead of the default config.yaml
	#[argh(option, short = 'c')]
	config_file: Option<PathBuf>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
	pub leader_url: Option<String>,
	pub sync_interval: f32,
}

pub fn read_config() -> Config {
	let args: Args = argh::from_env();

	let config_file_path = args.config_file.unwrap_or_else(|| "config.yml".into());

	let config_file: Config = if let Ok(file) = File::open(config_file_path) {
		let reader = BufReader::new(file);
		serde_yaml::from_reader(reader).unwrap()
	} else {
		Config::default()
	};

	Config {
		leader_url: args.leader_url.or(config_file.leader_url),
		sync_interval: config_file.sync_interval,
	}
}
