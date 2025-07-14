use crate::dao::{Cycle, Proposal, ProposalState, ProposalType, Vote, VoteOption};
use crate::token::StakingLogResult;
use chrono::{DateTime, Local};
use std::fmt::{Display, Formatter, Result};

impl Display for ProposalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Display for VoteOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Display for ProposalState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Cycle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Cycle {} ({} - {})",
            self.number,
            nanos_to_localtime_str(self.start),
            nanos_to_localtime_str(self.end)
        )
    }
}

impl Display for Proposal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Proposal #{} [{}]", self.id, self.proposal_type)?;
        writeln!(f, "  Author: {}", self.created_by)?;
        writeln!(
            f,
            "  Created on: {}",
            nanos_to_localtime_str(self.created_on)
        )?;
        writeln!(f, "  Start: {}", nanos_to_localtime_str(self.start))?;
        writeln!(f, "  End: {}", nanos_to_localtime_str(self.end))?;
        writeln!(f, "  Data: {}", self.data)?;
        writeln!(f, "  Votes: {:?}", self.votes)
    }
}

impl Display for Vote {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Vote ID: {}", self.id)?;
        writeln!(f, "  Proposal ID: {}", self.proposal_id)?;
        writeln!(f, "  Voter: {}", self.created_by)?;
        writeln!(
            f,
            "  Created on: {}",
            nanos_to_localtime_str(self.created_on)
        )?;
        writeln!(f, "  Result: {}", self.result)
    }
}

impl Display for StakingLogResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let from_dt = nanos_to_localtime_str(self.from);
        let to_dt = nanos_to_localtime_str(self.to);

        writeln!(f, "Staking Log (from {} to {}):", from_dt, to_dt)?;

        for entry in &self.log {
            let dt = nanos_to_localtime_str(entry.timestamp);

            writeln!(
                f,
                "- [{}] previous: {}, current: {}",
                dt, entry.previous_amount, entry.current_amount
            )?;
        }

        Ok(())
    }
}

fn nanos_to_localtime(nanos: u64) -> DateTime<Local> {
    let secs = nanos / 1_000_000_000;
    let sub_nanos = (nanos % 1_000_000_000) as u32;

    let utc = DateTime::from_timestamp(secs as i64, sub_nanos).unwrap();
    let local = utc.with_timezone(&Local);

    local
}

fn nanos_to_localtime_str(nanos: u64) -> String {
    let local_time = nanos_to_localtime(nanos);
    let result = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
    result
}
