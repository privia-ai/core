use crate::app::app_services::config::AppConfig;

pub mod service_builder;
pub mod app_services;

pub trait IConfigStorage {
    fn set_config(&mut self, config: AppConfig);
    fn get_config(&self) -> AppConfig;
}