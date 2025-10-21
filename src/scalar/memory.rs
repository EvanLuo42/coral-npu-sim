use crate::common::io::{Future, Poll};
use crate::common::tick::Tickable;
use crate::scalar::instruction::RawInstruction;

pub struct Itcm {
    /// 8KB Itcm
    data: [RawInstruction; 2048],
    latency: u8,
    /// Pending read request (address, remaining latency)
    pending: Option<(u32, u8)>
}

#[derive(Copy, Clone)]
pub struct ItcmRead {
    pub addr: u32,
    pub remaining_cycles: u8
}

impl Future for ItcmRead {
    type Output = RawInstruction;
    type Input = Itcm;

    fn poll(&mut self, context: &mut Self::Input) -> Poll<Self::Output> {
        if self.remaining_cycles > 0 {
            self.remaining_cycles -= 1;
            return Poll::Pending;
        }
        Poll::Ready(context._read(self.addr))
    }
}

impl Itcm {
    pub fn read(&self, addr: u32) -> ItcmRead {
        ItcmRead {
            addr,
            remaining_cycles: self.latency
        }
    }

    pub(crate) fn _read(&self, addr: u32) -> RawInstruction {
        todo!()
    }
}

impl Tickable for Itcm {
    fn tick(&mut self) {

    }
}

pub struct Dtcm {
    data: [RawInstruction; 8192],
    latency: u16
}


