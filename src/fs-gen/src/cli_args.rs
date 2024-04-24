use std::{env, path::PathBuf};

use clap::{command, error::ErrorKind, CommandFactory, Parser};
use regex::Regex;

use once_cell::sync::Lazy;

// So, for any of you who may be scared, this is the regex from the OCI Distribution Sepcification for the image name + the tag
static RE_IMAGE_NAME: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[a-z0-9]+((\.|_|__|-+)[a-z0-9]+)*(\/[a-z0-9]+((\.|_|__|-+)[a-z0-9]+)*)*:[a-zA-Z0-9_][a-zA-Z0-9._-]{0,127}").unwrap()
});

/// Convert an OCI image into a CPIO file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// The name of the image to download
    pub image_name: String,

    /// The path to the output file
    #[arg(short='o', long="output", default_value=get_default_output_file().into_os_string())]
    pub output_file: PathBuf,

    /// The path to the temporary folder
    #[arg(short='t', long="tempdir", default_value=get_default_temp_directory().into_os_string())]
    pub temp_directory: PathBuf,

    /// The host path to the guest agent binary
    pub agent_host_path: PathBuf,
}

impl CliArgs {
    /// Get the cli arguments with additional validation
    pub fn get_args() -> Self {
        let args = CliArgs::parse();

        args.validate_image();
        args.validate_host_path();

        args
    }

    fn validate_image(&self) {
        if !RE_IMAGE_NAME.is_match(&self.image_name) {
            let mut cmd = CliArgs::command();
            cmd.error(
                ErrorKind::InvalidValue,
                format!("Invalid image name: \"{}\"", self.image_name),
            )
            .exit();
        }
    }

    fn validate_host_path(&self) {
        if !self.agent_host_path.exists() {
            let mut cmd = CliArgs::command();
            cmd.error(
                ErrorKind::InvalidValue,
                format!(
                    "File not found for agent binary: \"{}\"",
                    self.agent_host_path.to_string_lossy()
                ),
            )
            .exit();
        }
    }
}

/// Get the default output path for the cpio file.
fn get_default_temp_directory() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push(".cloudlet_temp/");
    path
}

fn get_default_output_file() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push("initramfs.img");
    path
}
