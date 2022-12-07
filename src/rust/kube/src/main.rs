use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams, ResourceExt},
    config::{KubeConfigOptions, Kubeconfig},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kube_config = Kubeconfig::read_from("kube.cfg").unwrap();
    let kube_config_options = KubeConfigOptions {
        context: None,
        cluster: None,
        user: None,
    };
    let config = kube::Config::from_custom_kubeconfig(kube_config, &kube_config_options)
        .await
        .unwrap();
    let client = Client::try_from(config).unwrap();

    let pods: Api<Pod> = Api::namespaced(client, "hyosho");
    let lp = ListParams::default(); // for this app only
    for p in pods.list(&lp).await? {
        println!("Found Pod: {}", p.name_any());
    }

    Ok(())
}
