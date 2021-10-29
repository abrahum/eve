use std::sync::Arc;

use eve_core::{Client, ClientNet, Config, DeviceInfo, Password};

fn main() {
    std::env::set_var("RUST_LOG", "eve_core=trace");
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    tracing_subscriber::fmt().with_env_filter(filter).init();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap();
    let config = Config::load_or_new();

    // all config and cli work should be down outside.
    rt.block_on(async move {
        let (cli, receiver) = Client::new(
            config.clients[0].uid,
            Password::from_str(&config.clients[0].password),
            false,
            DeviceInfo::new(),
        )
        .await;
        let client = Arc::new(cli);
        let client_net = ClientNet::new(client.clone(), receiver);
        // client.update_servers().await;
        let stream = client_net.connect_tcp().await;
        let net = tokio::spawn(client_net.net_loop(stream));
        client.login().await;
        net.await.unwrap().unwrap();
    })
}
