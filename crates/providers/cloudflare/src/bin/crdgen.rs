use cloudflare_provider::generated::{
    account::Account, dns_record::DNSRecord, pages_project::PagesProject, zone::Zone,
    zone_ruleset::Ruleset,
};
use kube::CustomResourceExt;

fn main() -> anyhow::Result<()> {
    serde_yaml::to_writer(std::io::stdout(), &Account::crd())?;
    println!("---");
    serde_yaml::to_writer(std::io::stdout(), &Zone::crd())?;
    println!("---");
    serde_yaml::to_writer(std::io::stdout(), &DNSRecord::crd())?;
    println!("---");
    serde_yaml::to_writer(std::io::stdout(), &Ruleset::crd())?;
    println!("---");
    serde_yaml::to_writer(std::io::stdout(), &PagesProject::crd())?;
    Ok(())
}
