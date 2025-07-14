use crate::domain::interfaces::storage::IMetadataStore;
use crate::domain::types::*;
use crate::icp::stable_storage::{get_metadata_memory, IcpMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{StableCell, Storable};
use std::borrow::Cow;

pub struct MetadataStoreStable {
    collection_metadata: StableCell<CollectionMetadataStorable, IcpMemory>,
}

impl MetadataStoreStable {
    pub fn init() -> Self {
        Self {
            collection_metadata: StableCell::init(
                get_metadata_memory(),
                CollectionMetadataStorable(CollectionMetadata::default()),
            )
            .unwrap(),
        }
    }
}

impl IMetadataStore for MetadataStoreStable {
    fn get_collection_metadata(&self) -> CollectionMetadata {
        self.collection_metadata.get().0.clone()
    }
}

struct CollectionMetadataStorable(pub CollectionMetadata);

impl Storable for CollectionMetadataStorable {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let inner: CollectionMetadata = candid::decode_one(&bytes).unwrap();
        CollectionMetadataStorable(inner)
    }

    const BOUND: Bound = Bound::Unbounded;
}
