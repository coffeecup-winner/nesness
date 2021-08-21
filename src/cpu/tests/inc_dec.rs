use super::*;

#[test]
fn test_inc() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut ram) = test_cpu(&vec![INC_ZPG, 0x00]);
        ram[0x00] = i;
        let expected = i.wrapping_add(1);
        cpu.run_one(&mut ram);
        assert_eq!(expected, ram[0x00]);
        assert_zn!(cpu, expected == 0, expected >= 0x80);
    }

    // Addressing
    let x = 0x10;
    let v = 0x40;
    let zpg = 0x80;
    let abs = 0x1234;
    let (mut cpu, mut ram) = test_cpu(&vec![
        INC_ZPG, zpg,
        INC_ZPX, zpg,
        INC_ABS, lo(abs), hi(abs),
        INC_ABX, lo(abs), hi(abs),
    ]);
    cpu.reg_x = x;
    ram[zpg as usize] = v + 0;
    ram[(zpg + x) as usize] = v + 1;
    ram[abs as usize] = v + 2;
    ram[(abs + x as u16) as usize] = v + 3;

    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(7, cpu.run_one(&mut ram));

    assert_eq!(v + 1, ram[zpg as usize]);
    assert_eq!(v + 2, ram[(zpg + x) as usize]);
    assert_eq!(v + 3, ram[abs as usize]);
    assert_eq!(v + 4, ram[(abs + x as u16) as usize]);
}

#[test]
fn test_inx() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut ram) = test_cpu(&vec![INX_IMP]);
        cpu.reg_x = i;
        let expected = i.wrapping_add(1);
        assert_eq!(2, cpu.run_one(&mut ram));
        assert_eq!(expected, cpu.reg_x);
        assert_zn!(cpu, expected == 0, expected >= 0x80);
    }
}

#[test]
fn test_iny() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut ram) = test_cpu(&vec![INY_IMP]);
        cpu.reg_y = i;
        let expected = i.wrapping_add(1);
        assert_eq!(2, cpu.run_one(&mut ram));
        assert_eq!(expected, cpu.reg_y);
        assert_zn!(cpu, expected == 0, expected >= 0x80);
    }
}

#[test]
fn test_dec() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut ram) = test_cpu(&vec![DEC_ZPG, 0x00]);
        ram[0x00] = i;
        let expected = i.wrapping_sub(1);
        cpu.run_one(&mut ram);
        assert_eq!(expected, ram[0x00]);
        assert_zn!(cpu, expected == 0, expected >= 0x80);
    }

    // Addressing
    let x = 0x10;
    let v = 0x40;
    let zpg = 0x80;
    let abs = 0x1234;
    let (mut cpu, mut ram) = test_cpu(&vec![
        DEC_ZPG, zpg,
        DEC_ZPX, zpg,
        DEC_ABS, lo(abs), hi(abs),
        DEC_ABX, lo(abs), hi(abs),
    ]);
    cpu.reg_x = x;
    ram[zpg as usize] = v + 1;
    ram[(zpg + x) as usize] = v + 2;
    ram[abs as usize] = v + 3;
    ram[(abs + x as u16) as usize] = v + 4;

    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(7, cpu.run_one(&mut ram));

    assert_eq!(v + 0, ram[zpg as usize]);
    assert_eq!(v + 1, ram[(zpg + x) as usize]);
    assert_eq!(v + 2, ram[abs as usize]);
    assert_eq!(v + 3, ram[(abs + x as u16) as usize]);
}

#[test]
fn test_dex() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut ram) = test_cpu(&vec![DEX_IMP]);
        cpu.reg_x = i;
        let expected = i.wrapping_sub(1);
        assert_eq!(2, cpu.run_one(&mut ram));
        assert_eq!(expected, cpu.reg_x);
        assert_zn!(cpu, expected == 0, expected >= 0x80);
    }
}

#[test]
fn test_dey() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut ram) = test_cpu(&vec![DEY_IMP]);
        cpu.reg_y = i;
        let expected = i.wrapping_sub(1);
        assert_eq!(2, cpu.run_one(&mut ram));
        assert_eq!(expected, cpu.reg_y);
        assert_zn!(cpu, expected == 0, expected >= 0x80);
    }
}
