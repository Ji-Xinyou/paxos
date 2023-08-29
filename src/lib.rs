#![allow(unused)]

pub mod paxospb {
    include!(concat!(env!("OUT_DIR"), "/paxospb.rs"));
}

mod acceptor;
mod proposer;

impl PartialOrd for paxospb::BallotNum {
    fn lt(&self, other: &Self) -> bool {
        unimplemented!()
    }

    fn le(&self, other: &Self) -> bool {
        unimplemented!()
    }

    fn gt(&self, other: &Self) -> bool {
        unimplemented!()
    }

    fn ge(&self, other: &Self) -> bool {
        unimplemented!()
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unimplemented!()
    }
}
