use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request, Response,
};
use std::time::Instant;

pub struct RequestLogger;

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        request.local_cache(|| Instant::now());
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let start_time = request.local_cache(|| Instant::now());
        let duration = start_time.elapsed();

        let status = response.status();
        let method = request.method();
        let uri = request.uri();
        
        let client_ip = request
            .client_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let user_agent = request
            .headers()
            .get_one("User-Agent")
            .unwrap_or("unknown");

        match status.code {
            200..=399 => tracing::info!(
                method = %method,
                uri = %uri,
                status = status.code,
                duration_ms = duration.as_millis() as u64,
                client_ip = %client_ip,
                user_agent = %user_agent,
                "Request completed"
            ),
            400..=499 => tracing::warn!(
                method = %method,
                uri = %uri,
                status = status.code,
                duration_ms = duration.as_millis() as u64,
                client_ip = %client_ip,
                user_agent = %user_agent,
                "Client error"
            ),
            500..=599 => tracing::error!(
                method = %method,
                uri = %uri,
                status = status.code,
                duration_ms = duration.as_millis() as u64,
                client_ip = %client_ip,
                user_agent = %user_agent,
                "Server error"
            ),
            _ => tracing::debug!(
                method = %method,
                uri = %uri,
                status = status.code,
                duration_ms = duration.as_millis() as u64,
                "Request completed"
            ),
        }
    }
}
