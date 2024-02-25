use crate::{
    consts::{HDMA1_ADDR, HDMA2_ADDR, HDMA3_ADDR, HDMA4_ADDR, HDMA5_ADDR},
    warnln,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DmaMode {
    General = 0x00,
    HBlank = 0x01,
}

pub struct Dma {
    source: u16,
    destination: u16,
    length: u16,
    pending: u16,
    mode: DmaMode,
    active: bool,
}

impl Dma {
    pub fn new() -> Self {
        Self {
            source: 0x0,
            destination: 0x0,
            length: 0x0,
            pending: 0x0,
            mode: DmaMode::General,
            active: false,
        }
    }

    pub fn reset(&mut self) {
        self.source = 0x0;
        self.destination = 0x0;
        self.length = 0x0;
        self.pending = 0x0;
        self.mode = DmaMode::General;
        self.active = false;
    }

    pub fn clock(&mut self, _cycles: u16) {}

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            // 0xFF55 — HDMA5: VRAM DMA length/mode/start (CGB only)
            HDMA5_ADDR => ((self.pending >> 4) as u8).wrapping_sub(1) | ((!self.active as u8) << 7),
            _ => {
                warnln!("Reading from unknown DMA location 0x{:04x}", addr);
                0xff
            }
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // 0xFF51 — HDMA1: VRAM DMA source high (CGB only)
            HDMA1_ADDR => self.source = (self.source & 0x00ff) | ((value as u16) << 8),
            // 0xFF52 — HDMA2: VRAM DMA source low (CGB only)
            HDMA2_ADDR => self.source = (self.source & 0xff00) | ((value & 0xf0) as u16),
            // 0xFF53 — HDMA3: VRAM DMA destination high (CGB only)
            HDMA3_ADDR => self.destination = (self.destination & 0x00ff) | ((value as u16) << 8),
            // 0xFF54 — HDMA4: VRAM DMA destination low (CGB only)
            HDMA4_ADDR => self.destination = (self.destination & 0xff00) | ((value & 0xf0) as u16),
            // 0xFF55 — HDMA5: VRAM DMA length/mode/start (CGB only)
            HDMA5_ADDR => {
                self.length = (((value & 0x7f) + 0x1) as u16) << 4;
                self.mode = match (value & 80) >> 7 {
                    0 => DmaMode::General,
                    1 => DmaMode::HBlank,
                    _ => DmaMode::General,
                };
                self.pending = self.length;
                self.active = true;
            }
            _ => warnln!("Writing to unknown DMA location 0x{:04x}", addr),
        }
    }

    pub fn source(&self) -> u16 {
        self.source
    }

    pub fn set_source(&mut self, value: u16) {
        self.source = value;
    }

    pub fn destination(&self) -> u16 {
        self.destination
    }

    pub fn set_destination(&mut self, value: u16) {
        self.destination = value;
    }

    pub fn length(&self) -> u16 {
        self.length
    }

    pub fn set_length(&mut self, value: u16) {
        self.length = value;
    }

    pub fn pending(&self) -> u16 {
        self.pending
    }

    pub fn set_pending(&mut self, value: u16) {
        self.pending = value;
    }

    pub fn mode(&self) -> DmaMode {
        self.mode
    }

    pub fn set_mode(&mut self, value: DmaMode) {
        self.mode = value;
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, value: bool) {
        self.active = value;
    }
}

impl Default for Dma {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Dma, DmaMode};

    #[test]
    fn test_dma_default() {
        let dma = Dma::default();
        assert!(!dma.active);
    }

    #[test]
    fn test_dma_reset() {
        let mut dma = Dma::new();
        dma.source = 0x1234;
        dma.destination = 0x5678;
        dma.length = 0x9abc;
        dma.pending = 0x9abc;
        dma.mode = DmaMode::HBlank;
        dma.active = true;

        dma.reset();

        assert_eq!(dma.source, 0x0);
        assert_eq!(dma.destination, 0x0);
        assert_eq!(dma.length, 0x0);
        assert_eq!(dma.pending, 0x0);
        assert_eq!(dma.mode, DmaMode::General);
        assert!(!dma.active);
    }

    #[test]
    fn test_dma_set_active() {
        let mut dma = Dma::new();
        dma.set_active(true);
        assert!(dma.active);
    }
}
