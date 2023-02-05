use serde_json::json;
use serde_json::Value;
use std::{collections::HashMap, fs::File, path::Path};

use clap::Parser;

/// json/yaml compile tree
///
/// Given a root to a directory with json or yaml files, compile a single json or yaml.
/// The output structure reflects the directory structure (excluding the file extensions).
#[derive(Parser)]
#[clap(author = "Eldad Zack <eldad@fogrefinery.com>", version, about)]
struct Args {
    /// Path to the root directory
    #[arg()]
    root_path: String,

    #[arg(short, long)]
    yaml_output: bool,
}

fn jy_from_file(path: &Path) -> anyhow::Result<Option<Value>> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => {
            let file = File::open(path)?;
            let value = serde_json::from_reader(file)?;
            Ok(Some(value))
        }
        Some("yaml") | Some("yml") => {
            let file = File::open(path)?;
            let value = serde_yaml::from_reader(file)?;
            Ok(Some(value))
        }
        Some(_) | None => Ok(None),
    }
}

fn compile_directory(dir: &Path) -> anyhow::Result<Value> {
    let mut ret: HashMap<String, Value> = HashMap::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        let entry_name = path
            .file_stem()
            .ok_or_else(|| anyhow::anyhow!("failed to get file stem of path"))?
            .to_string_lossy()
            .to_string();

        if path.is_dir() {
            ret.insert(entry_name, compile_directory(&path)?);
        } else {
            let maybe_entry = jy_from_file(&path)?;
            if let Some(entry_value) = maybe_entry {
                ret.insert(entry_name, entry_value);
            }
        }
    }

    Ok(json!(ret))
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let root_path = Path::new(&args.root_path);
    if !root_path.is_dir() {
        return Err(anyhow::anyhow!("{} is not a directory", &args.root_path));
    }
    let compiled_value = compile_directory(root_path)?;
    let stdout = std::io::stdout();

    if args.yaml_output {
        serde_yaml::to_writer(stdout, &compiled_value)?;
    } else {
        serde_json::to_writer(stdout, &compiled_value)?;
    }
    Ok(())
}
