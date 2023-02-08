use serde::Deserialize;
use web::WebConfig;

fn default_web_port() -> u16 {
    8080
}

#[derive(Deserialize, Debug)]
struct AppConfig {
    #[serde(default = "default_web_port")]
    web_port: u16,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Can load .env file.");

    // a builder for `FmtSubscriber`.
    tracing_subscriber::fmt::init();

    let app_config = envy::prefixed("RT_")
        .from_env::<AppConfig>()
        .expect("Can parse AppConfig");

    // initialize
    // * ECS
    // * Bastion
    // * Web

    let bastion_config = bastion::Config::new().show_backtraces();
    bastion::Bastion::init_with(bastion_config);
    bastion::Bastion::start();

    let _web_supervisor = web::start(WebConfig {
        listen_port: app_config.web_port,
    });

    bastion::Bastion::block_until_stopped();
}