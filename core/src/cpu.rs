use crate::emu::Emu;

pub mod bus;
pub mod dma;
mod irqs;
pub mod regs;
pub use irqs::Irqs;

mod common;
#[cfg(feature = "disasm")]
pub mod disasm;
mod interpreter;

use regs::Regs;

pub struct Cpu {
    #[cfg(feature = "log")]
    logger: slog::Logger,
    pub regs: Regs,
    pub stopped: bool,
    pub irqs: Irqs,
    pub dmac: dma::Controller,
}

impl Cpu {
    pub(crate) fn new(#[cfg(feature = "log")] logger: slog::Logger) -> Self {
        Cpu {
            #[cfg(feature = "log")]
            logger,
            regs: Regs::new(),
            stopped: false,
            irqs: Irqs::new(),
            dmac: dma::Controller::new(),
        }
    }

    pub(crate) fn soft_reset(emu: &mut Emu) {
        interpreter::soft_reset(emu);
    }

    #[inline]
    pub(crate) fn run_until_next_event(emu: &mut Emu) {
        interpreter::run_until_next_event(emu)
    }
}
