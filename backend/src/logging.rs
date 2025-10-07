use std::env;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_logging() {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let log_format = env::var("LOG_FORMAT").unwrap_or_else(|_| "pretty".to_string());

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("auth_rs={},rocket=info", log_level)));

    let registry = tracing_subscriber::registry().with(env_filter);

    match log_format.as_str() {
        "json" => {
            registry
                .with(fmt::layer().json().with_target(true).with_thread_ids(true))
                .init();
        }
        "compact" => {
            registry.with(fmt::layer().compact()).init();
        }
        _ => {
            registry
                .with(
                    fmt::layer()
                        .pretty()
                        .with_target(true)
                        .with_thread_ids(false)
                        .with_line_number(true),
                )
                .init();
        }
    }
}
