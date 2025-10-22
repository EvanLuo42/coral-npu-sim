use crate::scalar::decode::DecodeStage;
use crate::scalar::dispatch::DispatchStage;
use crate::scalar::fetch::FetchStage;
use crate::scalar::instruction::InstructionBuffer;
use crate::scalar::memory::Itcm;

/// The ScalarFrontend struct encapsulates the fetch, decode, and dispatch stages
pub struct ScalarFrontend {
    pub fetch: FetchStage,
    pub decode: DecodeStage,
    pub dispatch: DispatchStage,
    pub instr_buffer: InstructionBuffer,
    pub itcm: Itcm
}

impl ScalarFrontend {
    /// Creates a new ScalarFrontend instance with initialized stages and buffers
    pub fn new() -> Self {
        let instr_buffer = InstructionBuffer::new(4);
        let itcm = Itcm::new(1);
        let fetch = FetchStage::new();
        let decode = DecodeStage::new();
        let dispatch = DispatchStage::new();
        ScalarFrontend {
            fetch,
            decode,
            dispatch,
            instr_buffer,
            itcm,
        }
    }

    /// Advances the frontend by one tick, processing fetch, decode, and dispatch stages
    pub fn tick(&mut self) {
        self.fetch.tick(&mut self.instr_buffer, &mut self.itcm);

        let batch = self.instr_buffer.pop_batch(4);
        if !batch.is_empty() {
            self.decode.accept_batch(batch);
        }

        self.decode.tick(&mut self.instr_buffer, &mut self.dispatch.queue);
        self.dispatch.tick();
    }
}