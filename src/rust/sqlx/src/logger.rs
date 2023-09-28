use tracing_subscriber::fmt::layer;
use tracing_subscriber::{
    filter::{self},
    prelude::*,
    registry,
};

pub fn init() {
    // Production
    // let log_level = filter::LevelFilter::INFO;
    // Development
    let log_level = filter::LevelFilter::DEBUG;

    let env_filter = filter::EnvFilter::new("")
        .add_directive(log_level.into())
        .add_directive("sqlx::query=debug".parse().unwrap())
        // .add_directive("sqlx::query=error".parse().unwrap())
        .add_directive("hyper=warn".parse().unwrap())
        .add_directive("reqwest=warn".parse().unwrap());

    let fmt_layer = layer().with_filter(env_filter);
    registry().with(fmt_layer).init();
}
