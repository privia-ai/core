use crate::domain::interfaces::storage::IVotingStorage;
use crate::icp::stable_storage::IcpMemory;
use abstractions::dao::{Proposal, Vote};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{StableBTreeMap, Storable};
use std::borrow::Cow;

pub struct VotingStorageStable {
    proposals: StableBTreeMap<u64, StorableProposal, IcpMemory>,
    votes: StableBTreeMap<u64, StorableVote, IcpMemory>,
}

impl VotingStorageStable {
    pub fn init() -> Self {
        Self {
            proposals: StableBTreeMap::init(super::get_proposals_memory()),
            votes: StableBTreeMap::init(super::get_votes_memory()),
        }
    }
}

impl IVotingStorage for VotingStorageStable {
    fn add_proposal(&mut self, proposal: Proposal) -> u64 {
        let id = self.proposals.len();
        let mut proposal = StorableProposal(proposal);
        proposal.0.id = id;
        self.proposals.insert(id, proposal);
        id
    }

    fn get_proposal(&self, id: &u64) -> Option<Proposal> {
        let mut proposal = self.proposals.get(id).map(|p| p.0)?;
        proposal.id = id.clone();
        Some(proposal)
    }

    fn update_proposal(&mut self, proposal: Proposal) {
        let id = proposal.id;
        let proposal = StorableProposal(proposal);
        self.proposals.insert(id, proposal);
    }

    fn add_vote(&mut self, vote: Vote) -> u64 {
        let vote_id = self.votes.len();
        let mut vote = StorableVote(vote);
        vote.0.id = vote_id;
        self.votes.insert(vote_id, vote);
        vote_id
    }

    fn get_vote(&self, id: &u64) -> Option<Vote> {
        let mut vote = self.votes.get(id).map(|v| v.0)?;
        vote.id = id.clone();
        Some(vote)
    }

    fn get_all_votes(&self, proposal_id: &u64) -> Vec<Vote> {
        let proposal = self.proposals.get(proposal_id);
        match proposal {
            None => vec![],
            Some(p) => {
                let mut result = Vec::new();
                for vote_id in p.0.votes {
                    let v = self.get_vote(&vote_id).unwrap();
                    result.push(v);
                }
                result
            }
        }
    }
}

struct StorableProposal(pub Proposal);

impl Storable for StorableProposal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let inner: Proposal = candid::decode_one(&bytes).unwrap();
        StorableProposal(inner)
    }

    const BOUND: Bound = Bound::Unbounded;
}

struct StorableVote(pub Vote);

impl Storable for StorableVote {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let inner: Vote = candid::decode_one(&bytes).unwrap();
        StorableVote(inner)
    }

    const BOUND: Bound = Bound::Unbounded;
}
