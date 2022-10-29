use std::path;

use anyhow::Result;
use clap::Parser;

/// The eunomia-bpf compile tool
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "eunomia compiler",
    long_about = "see https://github.com/eunomia-bpf/eunomia-bpf for more information"
)]

pub struct Args {
    /// path of the bpf.c file to compile
    #[arg()]
    pub source_path: String,

    /// path of output bpf object
    #[arg(short, long, default_value_t = ("output").to_string())]
    pub output_path: String,

    /// include path of compile btf object
    #[arg(short, long, default_value_t = ("").to_string())]
    pub include_path: String,

    /// path of clang binary
    #[arg(short, long, default_value_t = ("clang").to_string())]
    pub clang_bin: String,

    /// path of llvm strip binary
    #[arg(short, long, default_value_t = ("llvm-strip").to_string())]
    pub llvm_strip_bin: String,
}

/// Get home directory from env
pub fn get_eunomia_home() -> Result<String> {
    let eunomia_home = std::env::var("EUNOMIA_HOME");
    match eunomia_home {
        Ok(home) => Ok(home),
        Err(_) => match home::home_dir() {
            Some(home) => {
                let home = home.join(".eunomia");
                Ok(home.to_str().unwrap().to_string())
            }
            None => return Err(anyhow::anyhow!("HOME is not found")),
        },
    }
}

/// Get output path for json: output.meta.json
pub fn get_output_json_path(output_path: &str) -> String {
    let output_path = path::Path::new(output_path);
    let output_json_path = output_path.with_extension("skel.json");
    output_json_path.to_str().unwrap().to_string()
}

/// Get output path for bpf object: output.bpf.o  
pub fn get_output_object_path(output_path: &str) -> String {
    let output_path = path::Path::new(output_path);
    let output_object_path = output_path.with_extension("bpf.o");
    output_object_path.to_str().unwrap().to_string()
}
