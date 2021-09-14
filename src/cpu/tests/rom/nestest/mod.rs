use super::*;

static NESTEST: &'static [u8] = include_bytes!("./nestest.nes");
static NESTEST_LOG: &'static str = include_str!("./nestest.log");

#[test]
fn test() {
    let rom = NESFile::load(&NESTEST).expect("Failed to load the nestest ROM");
    let mut nes = NES::new(rom);
    nes.cpu.pc = 0xc000;

    let mut total_cycles = 7;
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
        // TODO: PPU
        idx += 19;
        let cycles = u32::from_str_radix(&line[idx..], 10).expect("Failed to parse number of cycles");

        assert_eq!(pc, nes.cpu.pc);
        assert_eq!(cycles, total_cycles);
        assert_eq!(a, nes.cpu.reg_a);
        assert_eq!(x, nes.cpu.reg_x);
        assert_eq!(y, nes.cpu.reg_y);
        assert_eq!(s, nes.cpu.reg_s);
        assert_eq!(p, nes.cpu.pack_flags());

        total_cycles += nes.cpu.run_one(&mut nes.mmap) as u32;
    }

    // Verify internal error codes
    assert_eq!(0, nes.mmap.read_u8(2));
    assert_eq!(0, nes.mmap.read_u8(3));
}
