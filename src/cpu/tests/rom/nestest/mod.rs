use super::*;

static NESTEST: &'static [u8] = include_bytes!("./nestest.nes");
static NESTEST_LOG: &'static str = include_str!("./nestest.log");

#[test]
fn test() {
    let rom = NESFile::load(&NESTEST).expect("Failed to load the nestest ROM");
    let mut nes = NES::new(rom);
    nes.cpu.pc = 0xc000;

    for line in NESTEST_LOG.lines() {
        println!("{}", line);

        let pc = u16::from_str_radix(&line[0..4], 16).expect("Failed to parse PC");
        let mut idx = line.find("A:").expect("Failed to parse the log line");
        idx += 2;
        let a = u8::from_str_radix(&line[idx..idx + 2], 16).expect("Failed to parse A");
        idx += 5;
        let x = u8::from_str_radix(&line[idx..idx + 2], 16).expect("Failed to parse X");
        idx += 5;
        let y = u8::from_str_radix(&line[idx..idx + 2], 16).expect("Failed to parse Y");
        idx += 5;
        let p = u8::from_str_radix(&line[idx..idx + 2], 16).expect("Failed to parse P");
        idx += 6;
        let s = u8::from_str_radix(&line[idx..idx + 2], 16).expect("Failed to parse S");
        idx += 7;
        let ppu_scanline = u16::from_str_radix(&line[idx..idx + 3].trim_start(), 10).expect("Failed to parse PPU X");
        idx += 4;
        let ppu_cycle = u16::from_str_radix(&line[idx..idx + 3].trim_start(), 10).expect("Failed to parse PPU X");
        idx += 8;
        let cycles = u64::from_str_radix(&line[idx..], 10).expect("Failed to parse number of cycles");

        assert_eq!(pc, nes.cpu.pc);
        assert_eq!(a, nes.cpu.reg_a);
        assert_eq!(x, nes.cpu.reg_x);
        assert_eq!(y, nes.cpu.reg_y);
        assert_eq!(s, nes.cpu.reg_s);
        assert_eq!(p, nes.cpu.pack_flags());
        assert_eq!(ppu_scanline, nes.ppu.current_scanline);
        assert_eq!(ppu_cycle, nes.ppu.current_cycle);
        assert_eq!(cycles, nes.get_total_cycles());

        nes.tick();
        nes.wait_until_cpu_ready();
    }

    // Verify internal error codes
    assert_eq!(0, nes.mmap.read_u8(2));
    assert_eq!(0, nes.mmap.read_u8(3));
}
