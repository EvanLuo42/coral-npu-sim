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
        // 0x00: add x5, x1, x2
        data[0] = RawInstruction { data: 0x002082B3 };
        // 0x04: add x6, x5, x3
        data[1] = RawInstruction { data: 0x00328333 };
        // 0x08: add x7, x6, x4
        data[2] = RawInstruction { data: 0x004303B3 };
        // 0x0C: sw x7, 0(x0)
        data[3] = RawInstruction { data: 0x00702023 };
        // 0x10: lw x8, 0(x0)
        data[4] = RawInstruction { data: 0x00002403 };
        // 0x14: add x9, x8, x5
        data[5] = RawInstruction { data: 0x005404B3 };
        // 0x18: add x10, x9, x9
        data[6] = RawInstruction { data: 0x00948533 };
        // 0x1C: add x11, x10, x10
        data[7] = RawInstruction { data: 0x00A505B3 };
        // 0x20: add x12, x11, x11
        data[8] = RawInstruction { data: 0x00B58633 };
        // 0x24: add x13, x12, x12
        data[9] = RawInstruction { data: 0x00C606B3 };
        // 0x28: add x14, x13, x13
        data[10] = RawInstruction { data: 0x00D68733 };
        // 0x2C: add x15, x14, x14
        data[11] = RawInstruction { data: 0x00E707B3 };
        // 0x30: sw x15, 8(x14)
        data[12] = RawInstruction { data: 0x00F70823 };
        // 0x34: lw x17, 8(x14)
        data[13] = RawInstruction { data: 0x00872883 };
        // 0x38: add x17, x17, x17
        data[14] = RawInstruction { data: 0x011888B3 };
        // 0x3C: nop
        data[15] = RawInstruction { data: 0x00000013 };

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


