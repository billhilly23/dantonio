// src/dashboard/mod.rs
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use handlebars::Handlebars;
use chrono::{DateTime, Utc};
use anyhow::Result;
use actix_web_actors::ws;
use jwt_simple::prelude::*;

#[derive(Debug, Clone, Serialize)]
pub struct DashboardState {
    pub strategies: Vec<StrategyStatus>,
    pub wallet_balance: String,
    pub total_profit: String,
    pub active_trades: u32,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StrategyStatus {
    pub name: String,
    pub enabled: bool,
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub total_profit: String,
    pub last_execution: Option<DateTime<Utc>>,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceMetrics {
    pub success_rate: f64,
    pub average_profit: String,
    pub gas_used: String,
    pub execution_time: String,
}

#[derive(Debug, Deserialize)]
pub struct DashboardConfig {
    pub port: u16,
    pub jwt_secret: String,
    pub ws_heartbeat_interval: u64,
    pub metrics_enabled: bool,
}

pub struct Dashboard {
    state: Arc<RwLock<DashboardState>>,
    templates: Handlebars<'static>,
    config: DashboardConfig,
    ws_clients: Arc<RwLock<Vec<ws::WebsocketContext<WebSocketSession>>>>,
}

impl Dashboard {
    pub async fn new(config: DashboardConfig) -> Result<Self> {
        let mut templates = Handlebars::new();
        
        // Register templates from files
        templates.register_template_file("index", "templates/dashboard/index.hbs")?;
        templates.register_template_file("strategies", "templates/dashboard/strategies.hbs")?;
        templates.register_template_file("metrics", "templates/dashboard/metrics.hbs")?;

        Ok(Self {
            state: Arc::new(RwLock::new(DashboardState::default())),
            templates,
            config,
            ws_clients: Arc::new(RwLock::new(Vec::new())),
        })
    }

    pub async fn start(&self) -> Result<()> {
        let state = self.state.clone();
        let templates = self.templates.clone();
        let config = self.config.clone();
        let ws_clients = self.ws_clients.clone();

        HttpServer::new(move || {
            App::new()
                .wrap(self.create_auth_middleware())
                .app_data(web::Data::new(state.clone()))
                .app_data(web::Data::new(templates.clone()))
                .app_data(web::Data::new(ws_clients.clone()))
                // Static files
                .service(fs::Files::new("/static", "static"))
                // WebSocket endpoint
                .route("/ws", web::get().to(ws_handler))
                // API routes
                .service(
                    web::scope("/api")
                        .route("/status", web::get().to(get_status))
                        .route("/strategy/{name}/toggle", web::post().to(toggle_strategy))
                        .route("/metrics", web::get().to(get_metrics))
                        .route("/performance", web::get().to(get_performance))
                )
                // Web routes
                .service(
                    web::scope("")
                        .route("/", web::get().to(index_page))
                        .route("/strategies", web::get().to(strategies_page))
                        .route("/login", web::post().to(handle_login))
                )
        })
        .bind(("127.0.0.1", self.config.port))?
        .run()
        .await?;

        Ok(())
    }

    pub async fn update_state(&self, new_state: DashboardState) -> Result<()> {
        let mut state = self.state.write().await;
        *state = new_state;
        
        // Notify all WebSocket clients
        self.broadcast_state_update().await?;
        Ok(())
    }

    async fn broadcast_state_update(&self) -> Result<()> {
        let state = self.state.read().await;
        let state_json = serde_json::to_string(&*state)?;
        
        let mut clients = self.ws_clients.write().await;
        clients.retain_mut(|client| {
            client.text(state_json.clone()).is_ok()
        });
        
        Ok(())
    }

    fn create_auth_middleware(&self) -> impl actix_web::middleware::Transform {
        // Implement JWT authentication middleware
        unimplemented!()
    }
}

async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    ws_clients: web::Data<Arc<RwLock<Vec<ws::WebsocketContext<WebSocketSession>>>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let (res, session, msg_stream) = ws::WebsocketContext::new(stream);
    
    // Store the client session
    ws_clients.write().await.push(session);
    
    Ok(res)
}

async fn get_status(state: web::Data<Arc<RwLock<DashboardState>>>) -> impl Responder {
    let state = state.read().await;
    web::Json(state.clone())
}

async fn get_metrics(state: web::Data<Arc<RwLock<DashboardState>>>) -> impl Responder {
    let state = state.read().await;
    
    // Format metrics in Prometheus format
    let metrics = format!(
        "# HELP trading_bot_wallet_balance Current wallet balance\n\
         # TYPE trading_bot_wallet_balance gauge\n\
         trading_bot_wallet_balance {}\n\
         # HELP trading_bot_total_profit Total profit\n\
         # TYPE trading_bot_total_profit counter\n\
         trading_bot_total_profit {}\n\
         # HELP trading_bot_active_trades Number of active trades\n\
         # TYPE trading_bot_active_trades gauge\n\
         trading_bot_active_trades {}\n",
        state.wallet_balance,
        state.total_profit,
        state.active_trades
    );
    
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(metrics)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_dashboard_creation() {
        // Test dashboard initialization
    }

    #[actix_rt::test]
    async fn test_websocket_connection() {
        // Test WebSocket functionality
    }

    #[actix_rt::test]
    async fn test_metrics_endpoint() {
        // Test metrics formatting
    }
}


