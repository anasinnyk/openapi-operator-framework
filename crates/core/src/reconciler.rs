pub struct Observation {
    pub exists: bool,
    pub at_provider: Option<serde_json::Value>,
}

#[derive(Clone)]
pub struct ControllerContext<P> {
    pub kube_client: kube::Client,
    pub provider_client: P,
}

impl<P> ControllerContext<P> {
    pub fn new(kube_client: kube::Client, provider_client: P) -> Self {
        Self {
            kube_client,
            provider_client,
        }
    }
}
