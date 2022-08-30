use serde_yaml::Value;

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let value: Value = serde_yaml::from_reader(stdin)?;
    serde_json::to_writer(stdout, &value)?;
    Ok(())
}
