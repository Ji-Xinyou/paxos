use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;

use crate::paxospb::*;

const MAX_RETRY: u64 = 3;

pub struct Proposer {
    quorum: Vec<String>,
    // ballot
    ts: AtomicU64,
    id: String,
}

impl Proposer {
    pub fn new(id: String, quorum: Vec<String>) -> Self {
        Self {
            quorum,
            ts: AtomicU64::new(0),
            id,
        }
    }

    pub fn propose_v(&mut self, v: Vec<u8>) -> bool {
        let mut proposing_v = None;
        for _ in 0..MAX_RETRY {
            let (new_v, ok) = self.propose_phase1();
            if ok {
                if let Some(new_v) = new_v {
                    proposing_v = Some(new_v);
                    break;
                }
            }
        }

        if proposing_v.is_none() {
            return false;
        }

        for _ in 0..MAX_RETRY {
            if self.propose_phase2(proposing_v.clone().unwrap()) {
                return true;
            }
        }

        false
    }

    /// repair v, start phase2?
    fn propose_phase1(&mut self) -> (Option<Vec<u8>>, bool) {
        let req = Phase1Request {
            ballot_num: Some(self.get_nextbal()),
        };

        let replies = self.mock_sendphase1(req);
        let quorum_sz = self.quorum.len() / 2 + 1;

        let mut accepted = 0;
        let mut seen_v = None;
        let mut seen_vbal = None;
        let bal = self.get_bal();

        for reply in replies {
            if let Some(last_ballot) = reply.last_ballot {
                // if any last_bal is bigger than this proposer, failed
                if last_ballot > bal {
                    return (None, false);
                }

                if last_ballot == bal {
                    accepted += 1;
                }

                // acceptor has seen a v, record the one with highest vbal
                if !reply.v.is_empty() {
                    if seen_vbal.is_none() || reply.vbal > seen_vbal {
                        seen_vbal = reply.vbal;
                        seen_v = Some(reply.v);
                    }
                }
            }
        }

        // only return on accepted by majority of the quorum
        if accepted < quorum_sz {
            (None, false)
        } else {
            (seen_v, true)
        }
    }

    fn propose_phase2(&self, v: Vec<u8>) -> bool {
        let req = Phase2Request {
            ballot_num: Some(self.get_bal()),
            v,
        };
        let replies = self.mock_sendphase2(req);
        let quorum_sz = self.quorum.len() / 2 + 1;

        let accepted = replies
            .into_iter()
            .filter(|r| r.ok)
            .collect::<Vec<_>>()
            .len();

        accepted >= quorum_sz
    }

    fn mock_sendphase1(&self, req: Phase1Request) -> Vec<Phase1Reply> {
        unimplemented!("Unimplemented on purpose....")
    }

    fn mock_sendphase2(&self, req: Phase2Request) -> Vec<Phase2Reply> {
        unimplemented!("Unimplemented on purpose....")
    }

    fn get_bal(&self) -> BallotNum {
        BallotNum {
            ballot: self.ts.load(SeqCst),
            node_id: self.id.clone(),
        }
    }

    fn get_nextbal(&mut self) -> BallotNum {
        let bal = BallotNum {
            ballot: self.ts.load(SeqCst),
            node_id: self.id.clone(),
        };

        self.ts.fetch_add(1, SeqCst);

        bal
    }
}
