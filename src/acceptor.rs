use crate::paxospb::*;
use std::cmp::max;

/// Paxos Acceptor
#[derive(Debug, Default)]
pub struct Acceptor {
    /// last ballot number that the acceptor seen
    last_bal: Option<BallotNum>,
    /// the value that this Acceptor accepts
    v: Vec<u8>,
    /// the round that the value is accepted
    vbal: Option<BallotNum>,
}

impl Acceptor {
    pub fn new() -> Self {
        Self {
            last_bal: None,
            v: vec![],
            vbal: None,
        }
    }
    /// Phase1 of paxos, the whole paxos instance is trying to fix a state on ballot number
    fn handle_phase1(&mut self, request: Phase1Request) -> Phase1Reply {
        let bal = request.ballot_num.unwrap();

        if let Some(last_bal) = self.last_bal.as_ref() {
            if last_bal < &bal {
                self.last_bal = Some(bal);
            }
        }

        Phase1Reply {
            last_ballot: self.last_bal.clone(),
            v: self.v.clone(),
            vbal: self.vbal.clone(),
        }
    }

    /// Phase2 of paxos, the proposer is trying to write a value into acceptor
    fn handle_phase2(&mut self, request: Phase2Request) -> Phase2Reply {
        if request.ballot_num.as_ref().unwrap() >= self.last_bal.as_ref().unwrap() {
            self.v = request.v;
            self.last_bal = request.ballot_num;
            Phase2Reply { ok: true }
        } else {
            Phase2Reply { ok: false }
        }
    }
}
