use super::{get_config_memory, IcpMemory};
use crate::app::app_services::config::{AppConfig};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{StableCell, Storable};
use std::borrow::Cow;
use crate::app::IConfigStorage;

pub struct ConfigStorageStable {
    config: StableCell<AppConfig, IcpMemory>,
}

impl ConfigStorageStable {
    pub fn init() -> Self {
        Self {
            config: StableCell::init(get_config_memory(), AppConfig::default()).unwrap(),
        }
    }
}

impl IConfigStorage for ConfigStorageStable {
    fn set_config(&mut self, config: AppConfig) {
        self.config.set(config).unwrap();
    }

    fn get_config(&self) -> AppConfig {
        self.config.get().clone()
    }
}

impl Storable for AppConfig {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        ciborium::ser::into_writer(&self, &mut buf).unwrap();
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        ciborium::de::from_reader(bytes.as_ref()).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
