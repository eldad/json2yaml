use serde_json::Value;

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let value: Value = serde_json::from_reader(stdin)?;
    serde_yaml::to_writer(stdout, &value)?;
    Ok(())
}
