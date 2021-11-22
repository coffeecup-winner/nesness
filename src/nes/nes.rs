use crate::{cpu::CPU, mem::Memory, rom::nes::NESFile, util::ClockDivider};

use super::{mmap::CpuMemoryMap, trace::ExecutionTrace};

#[allow(clippy::upper_case_acronyms)]
pub struct NES {
    pub cpu: CPU,
    pub mmap: CpuMemoryMap,

    total_ticks: u64, // Enough for ~25k years
    cpu_clock_divider: ClockDivider<12>,
    ppu_clock_divider: ClockDivider<4>,
}

impl NES {
    pub fn new(rom: NESFile) -> Self {
        let mut nes = NES {
            cpu: CPU::new(),
            mmap: CpuMemoryMap::new(rom.header.mapper, rom.prg_rom, rom.chr_rom),
            total_ticks: 0,
            cpu_clock_divider: ClockDivider::new(),
            ppu_clock_divider: ClockDivider::new(),
        };
        nes.reset();
        nes
    }

    pub fn get_total_cycles(&self) -> u64 {
        self.total_ticks / 12
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&self.mmap);
        self.total_ticks = 7 * 12; // CPU reset takes 7 cycles
                                   // Meanwhile PPU progresses through the first 7 * 3 dots
        for _ in 0..21 {
            self.mmap.ppu.run_one(&self.mmap.ppu_mmap);
        }
        self.cpu_clock_divider.reset();
        self.ppu_clock_divider.reset();
    }

    #[inline]
    pub fn tick(&mut self) {
        // NTSC timings
        if self.cpu_clock_divider.is_triggered() {
            let cycles = if self.mmap.ppu.is_cpu_interrupt_requested {
                self.mmap.ppu.is_cpu_interrupt_requested = false;
                self.cpu.execute_interrupt(&mut self.mmap)
            } else {
                self.cpu.run_one(&mut self.mmap)
            };
            self.cpu_clock_divider.delay(cycles as u64);
            if let Some(page) = self.mmap.ppu.oam_dma_page.take() {
                self.cpu_clock_divider.delay_ticks(512);
                // Copying is not cycle accurate
                let base_addr = (page as u16) << 8;
                for i in 0..=255 {
                    self.mmap
                        .ppu
                        .write_oamdata_raw(i, self.mmap.read_u8(base_addr + i as u16));
                }
            }
        }
        if self.ppu_clock_divider.is_triggered() {
            self.mmap.ppu.run_one(&self.mmap.ppu_mmap);
        }
        self.total_ticks += 1;
        self.cpu_clock_divider.global_tick();
        self.ppu_clock_divider.global_tick();
    }

    pub fn run_with_trace<T: ExecutionTrace>(&mut self, mut trace: T) {
        self.wait_until_cpu_ready();
        let cycle_offset = if trace.cycles_start_with_0() {
            self.get_total_cycles()
        } else {
            0
        };
        let mut step = 0;
        loop {
            if !trace.advance() {
                break;
            }
            if trace.total_cycles() != (self.get_total_cycles() - cycle_offset) {
                panic!(
                    "Failed verification trace on step {}: cycles {}, CPU: {:?}",
                    step,
                    trace.total_cycles(),
                    self.cpu
                )
            }
            if trace.reg_a() != self.cpu.reg_a {
                panic!(
                    "Failed verification trace on step {}: A {}, CPU: {:?}",
                    step,
                    trace.reg_a(),
                    self.cpu
                )
            }
            if trace.reg_x() != self.cpu.reg_x {
                panic!(
                    "Failed verification trace on step {}: X {}, CPU: {:?}",
                    step,
                    trace.reg_x(),
                    self.cpu
                )
            }
            if trace.reg_y() != self.cpu.reg_y {
                panic!(
                    "Failed verification trace on step {}: Y {}, CPU: {:?}",
                    step,
                    trace.reg_y(),
                    self.cpu
                )
            }
            if trace.reg_s() != self.cpu.reg_s {
                panic!(
                    "Failed verification trace on step {}: S {}, CPU: {:?}",
                    step,
                    trace.reg_s(),
                    self.cpu
                )
            }
            if trace.pc() != self.cpu.pc {
                panic!(
                    "Failed verification trace on step {}: PC {}, CPU: {:?}",
                    step,
                    trace.pc(),
                    self.cpu
                )
            }
            if trace.flag_carry() != self.cpu.flag_carry {
                panic!(
                    "Failed verification trace on step {}: C {}, CPU: {:?}",
                    step,
                    trace.flag_carry(),
                    self.cpu
                )
            }
            if trace.flag_zero() != self.cpu.flag_zero {
                panic!(
                    "Failed verification trace on step {}: Z {}, CPU: {:?}",
                    step,
                    trace.flag_zero(),
                    self.cpu
                )
            }
            if trace.flag_interrupt_disable() != self.cpu.flag_interrupt_disable {
                panic!(
                    "Failed verification trace on step {}: I {}, CPU: {:?}",
                    step,
                    trace.flag_interrupt_disable(),
                    self.cpu
                )
            }
            if trace.flag_decimal_mode() != self.cpu.flag_decimal_mode {
                panic!(
                    "Failed verification trace on step {}: D {}, CPU: {:?}",
                    step,
                    trace.flag_decimal_mode(),
                    self.cpu
                )
            }
            if trace.flag_break() != self.cpu.flag_break {
                panic!(
                    "Failed verification trace on step {}: B {}, CPU: {:?}",
                    step,
                    trace.flag_break(),
                    self.cpu
                )
            }
            if trace.flag_overflow() != self.cpu.flag_overflow {
                panic!(
                    "Failed verification trace on step {}: V {}, CPU: {:?}",
                    step,
                    trace.flag_overflow(),
                    self.cpu
                )
            }
            if trace.flag_negative() != self.cpu.flag_negative {
                panic!(
                    "Failed verification trace on step {}: N {}, CPU: {:?}",
                    step,
                    trace.flag_negative(),
                    self.cpu
                )
            }
            self.tick();
            self.wait_until_cpu_ready();
            step += 1;
        }
    }

    #[inline]
    pub fn wait_until_cpu_ready(&mut self) {
        while !self.cpu_clock_divider.is_triggered() {
            self.tick();
        }
    }

    #[cfg(debug_assertions)]
    pub fn dump(&self) {
        self.mmap.ppu.dump(&self.mmap.ppu_mmap);
    }
}
