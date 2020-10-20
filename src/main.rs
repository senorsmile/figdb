use serde_yaml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = include_str!("../sample.yaml");
    let yaml2 = include_str!("../sample2.yaml");
    let value: serde_yaml::Value = serde_yaml::from_str(yaml)?;
    let mut value2: serde_yaml::Value = serde_yaml::from_str(yaml2)?;
    println!("{:?}", value);
    println!("{:?}", value2);

    Ok(())
}
