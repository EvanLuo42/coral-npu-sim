use tracing::debug;
use crate::common::io::{Future, Poll};
use crate::scalar::instruction::RawInstruction;

/// ITCM (Instruction Tightly Coupled Memory)
pub struct Itcm {
    /// 8KB Itcm
    data: [RawInstruction; 2048],
    /// Simulated IO latency
    latency: u8,
    /// Pending read request (address, remaining latency)
    pending: Option<(u32, u8)>
}

impl Itcm {
    /// Create a new ITCM with given latency (in cycles)
    pub fn new(latency: u8) -> Self {
        let mut data = [RawInstruction::default(); 2048];
        // add x5, x1, x2
        data[0] = RawInstruction { data: 0x002082B3 };
        // add x6, x5, x3
        data[1] = RawInstruction { data: 0x00328333 };
        // add x7, x6, x4
        data[2] = RawInstruction { data: 0x004303B3 };
        // sw x7, 0(x0)
        data[3] = RawInstruction { data: 0x00702023 };
        // lw x8, 0(x0)
        data[4] = RawInstruction { data: 0x00002403 };
        // add x9, x8, x5
        data[5] = RawInstruction { data: 0x005404B3 };
        Self {
            data,
            latency,
            pending: None
        }
    }
}

/// ITCM read request future
#[derive(Copy, Clone)]
pub struct ItcmRead {
    pub addr: u32,
    pub remaining_cycles: u8
}

impl Future for ItcmRead {
    /// Raw instruction read from ITCM
    type Output = RawInstruction;
    /// ITCM reference as input context
    type Input = Itcm;

    /// Poll the read request
    fn poll(&mut self, context: &mut Self::Input) -> Poll<Self::Output> {
        if self.remaining_cycles > 0 {
            self.remaining_cycles -= 1;
            return Poll::Pending;
        }
        Poll::Ready(context._read(self.addr))
    }
}

impl Itcm {
    /// Issue a read request to ITCM
    pub fn read(&self, addr: u32) -> ItcmRead {
        debug!("ITCM read request addr=0x{:08x}", addr);
        ItcmRead {
            addr,
            remaining_cycles: self.latency.saturating_sub(1)
        }
    }

    /// Internal read function
    pub(crate) fn _read(&self, addr: u32) -> RawInstruction {
        debug_assert!(addr % 4 == 0, "Unaligend ITCM read: 0x{:08x}", addr);
        let index = ((addr / 4) as usize) % self.data.len();
        debug!("ITCM read addr=0x{:08x}, index={}, data=0x{:08x}", addr, index, self.data[index].data);
        self.data[index]
    }
}

/// DTCM (Data Tightly Coupled Memory)
pub struct Dtcm {
    data: [RawInstruction; 8192],
    latency: u16
}


