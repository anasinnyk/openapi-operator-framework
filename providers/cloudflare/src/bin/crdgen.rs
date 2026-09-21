use cloudflare_provider::{Account, DnsRecord, PagesProject, Zone, ZoneRuleset};
use kube::CustomResourceExt;

fn main() -> anyhow::Result<()> {
    serde_yaml::to_writer(std::io::stdout(), &Account::crd())?;
    println!("---");
    serde_yaml::to_writer(std::io::stdout(), &Zone::crd())?;
    println!("---");
    serde_yaml::to_writer(std::io::stdout(), &DnsRecord::crd())?;
    println!("---");
    serde_yaml::to_writer(std::io::stdout(), &ZoneRuleset::crd())?;
    println!("---");
    serde_yaml::to_writer(std::io::stdout(), &PagesProject::crd())?;
    Ok(())
}
