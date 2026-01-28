use crate::app::service_builder;

pub fn join() {
    let service = service_builder::build_hiving_service();
    service.add_hiving_canister();
}

pub fn leave() {
    let service = service_builder::build_hiving_service();
    service.remove_hiving_canister();
}
