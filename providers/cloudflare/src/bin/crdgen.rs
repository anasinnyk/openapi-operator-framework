use cloudflare_provider::Account;
use kube::CustomResourceExt;

fn main() -> anyhow::Result<()> {
    serde_yaml::to_writer(std::io::stdout(), &Account::crd())?;
    Ok(())
}
