use actix_web::web;
use log::info;
use crate::api::*;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes");

    cfg.service(
        web::scope("/api")
            .service(ping::ping)        
    );
}
