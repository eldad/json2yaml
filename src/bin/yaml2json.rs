use serde_yaml::Value;

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut yamls: Vec<Vec<String>> = Vec::new();
    let mut acc: Vec<String> = Vec::new();

    for line in stdin.lines().flat_map(Result::ok) {
        if line.trim() == "---" {
            if !acc.is_empty() {
                yamls.push(acc);
            }
            acc = Vec::new();
        }

        acc.push(line);
    }
    if !acc.is_empty() {
        yamls.push(acc);
    }

    if yamls.is_empty() {
        return Ok(());
    }

    let docs: Vec<_> = yamls.iter().map(|bundle| bundle.join("\n")).collect();

    // Single yaml document: map directly to json
    if docs.len() == 1 {
        let doc = docs.get(0).unwrap();
        let value: Value = serde_yaml::from_str(doc)?;
        serde_json::to_writer(&stdout, &value)?;
    }

    // Multiple yaml documents: output as json array
    let result: Result<Vec<Value>, serde_yaml::Error> = docs
        .into_iter()
        .map(|doc| serde_yaml::from_str(&doc))
        .collect();
    serde_json::to_writer(&stdout, &result?)?;

    Ok(())
}
