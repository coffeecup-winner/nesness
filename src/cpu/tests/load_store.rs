use super::*;

#[test]
fn test_lda() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut mem) = test_cpu(&vec![LDA_IMM, i]);
        cpu.reg_a = !i;
        cpu.run_one(&mut mem);
        assert_eq!(i, cpu.reg_a);
        assert_zn!(cpu, i == 0, i >= 0x80);
    }

    // Addressing
    let x = 0x10;
    let y = 0x20;
    let y2 = 0xc0;
    let v = 0x40;
    let zpg = 0x80;
    let zpg_inx = 0xa0;
    let inx = 0x6789;
    let zpg_iny = 0xc0;
    let iny = 0x3456;
    let abs = 0x1234;
    let abs2 = 0x12f8;
    let (mut cpu, mut mem) = test_cpu(&vec![
        LDA_IMM, v + 0,
        LDA_ZPG, zpg,
        LDA_ZPX, zpg,
        LDA_ABS, lo(abs), hi(abs),
        LDA_ABX, lo(abs), hi(abs),
        LDA_ABY, lo(abs), hi(abs),
        LDA_INX, zpg_inx,
        LDA_INY, zpg_iny,

        // Page crossing
        LDA_ABX, lo(abs2), hi(abs2),
        LDA_ABY, lo(abs2), hi(abs2),
        LDA_INY, zpg_iny,
    ]);
    cpu.reg_x = x;
    cpu.reg_y = y;
    mem[zpg as usize] = v + 1;
    mem[(zpg + x) as usize] = v + 2;
    mem[(zpg_inx + x) as usize] = lo(inx);
    mem[(zpg_inx + x + 1) as usize] = hi(inx);
    mem[zpg_iny as usize] = lo(iny);
    mem[(zpg_iny + 1) as usize] = hi(iny);
    mem[abs as usize] = v + 3;
    mem[(abs + x as u16) as usize] = v + 4;
    mem[(abs + y as u16) as usize] = v + 5;
    mem[inx as usize] = v + 6;
    mem[(iny + y as u16) as usize] = v + 7;
    mem[(abs2 + x as u16) as usize] = v + 8;
    mem[(abs2 + y as u16) as usize] = v + 9;
    mem[(iny + y2 as u16) as usize] = v + 10;

    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(v + 0, cpu.reg_a);
    assert_eq!(3, cpu.run_one(&mut mem));
    assert_eq!(v + 1, cpu.reg_a);
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v + 2, cpu.reg_a);
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v + 3, cpu.reg_a);
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v + 4, cpu.reg_a);
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v + 5, cpu.reg_a);
    assert_eq!(6, cpu.run_one(&mut mem));
    assert_eq!(v + 6, cpu.reg_a);
    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(v + 7, cpu.reg_a);

    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(v + 8, cpu.reg_a);
    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(v + 9, cpu.reg_a);
    cpu.reg_y = y2;
    assert_eq!(6, cpu.run_one(&mut mem));
    assert_eq!(v + 10, cpu.reg_a);
}

#[test]
fn test_ldx() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut mem) = test_cpu(&vec![LDX_IMM, i]);
        cpu.reg_x = !i;
        cpu.run_one(&mut mem);
        assert_eq!(i, cpu.reg_x);
        assert_zn!(cpu, i == 0, i >= 0x80);
    }

    // Addressing
    let y = 0x20;
    let v = 0x40;
    let zpg = 0x80;
    let abs = 0x1234;
    let abs2 = 0x12f8;
    let (mut cpu, mut mem) = test_cpu(&vec![
        LDX_IMM, v + 0,
        LDX_ZPG, zpg,
        LDX_ZPY, zpg,
        LDX_ABS, lo(abs), hi(abs),
        LDX_ABY, lo(abs), hi(abs),

        // Page crossing
        LDX_ABY, lo(abs2), hi(abs2),
    ]);
    cpu.reg_y = y;
    mem[zpg as usize] = v + 1;
    mem[(zpg + y) as usize] = v + 2;
    mem[abs as usize] = v + 3;
    mem[(abs + y as u16) as usize] = v + 4;
    mem[(abs2 + y as u16) as usize] = v + 5;

    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(v + 0, cpu.reg_x);
    assert_eq!(3, cpu.run_one(&mut mem));
    assert_eq!(v + 1, cpu.reg_x);
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v + 2, cpu.reg_x);
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v + 3, cpu.reg_x);
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v + 4, cpu.reg_x);

    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(v + 5, cpu.reg_x);
}

#[test]
fn test_ldy() {
    // Values/flags
    for i in 0..=0xff {
        let (mut cpu, mut mem) = test_cpu(&vec![LDY_IMM, i]);
        cpu.reg_y = !i;
        cpu.run_one(&mut mem);
        assert_eq!(i, cpu.reg_y);
        assert_zn!(cpu, i == 0, i >= 0x80);
    }

    // Addressing
    let x = 0x20;
    let v = 0x40;
    let zpg = 0x80;
    let abs = 0x1234;
    let abs2 = 0x12f8;
    let (mut cpu, mut mem) = test_cpu(&vec![
        LDY_IMM, v + 0,
        LDY_ZPG, zpg,
        LDY_ZPX, zpg,
        LDY_ABS, lo(abs), hi(abs),
        LDY_ABX, lo(abs), hi(abs),

        // Page crossing
        LDY_ABX, lo(abs2), hi(abs2),
    ]);
    cpu.reg_x = x;
    mem[zpg as usize] = v + 1;
    mem[(zpg + x) as usize] = v + 2;
    mem[abs as usize] = v + 3;
    mem[(abs + x as u16) as usize] = v + 4;
    mem[(abs2 + x as u16) as usize] = v + 5;

    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(v + 0, cpu.reg_y);
    assert_eq!(3, cpu.run_one(&mut mem));
    assert_eq!(v + 1, cpu.reg_y);
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v + 2, cpu.reg_y);
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v + 3, cpu.reg_y);
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v + 4, cpu.reg_y);

    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(v + 5, cpu.reg_y);
}

#[test]
fn test_sta() {
    // Values
    for i in 0..=0xff {
        let (mut cpu, mut mem) = test_cpu(&vec![STA_ZPG, 0x00]);
        cpu.reg_a = i;
        mem[0x00] = !i;
        cpu.run_one(&mut mem);
        assert_eq!(i, mem[0x00]);
    }

    // Addressing
    let x = 0x10;
    let y = 0x20;
    let v = 0x40;
    let zpg = 0x80;
    let zpg_inx = 0xa0;
    let inx = 0x6789;
    let zpg_iny = 0xc0;
    let iny = 0x3456;
    let abs = 0x1234;
    let (mut cpu, mut mem) = test_cpu(&vec![
        STA_ZPG, zpg,
        STA_ZPX, zpg,
        STA_ABS, lo(abs), hi(abs),
        STA_ABX, lo(abs), hi(abs),
        STA_ABY, lo(abs), hi(abs),
        STA_INX, zpg_inx,
        STA_INY, zpg_iny,
    ]);
    cpu.reg_x = x;
    cpu.reg_y = y;
    mem[(zpg_inx + x) as usize] = lo(inx);
    mem[(zpg_inx + x + 1) as usize] = hi(inx);
    mem[zpg_iny as usize] = lo(iny);
    mem[(zpg_iny + 1) as usize] = hi(iny);

    cpu.reg_a = v + 0;
    assert_eq!(3, cpu.run_one(&mut mem));
    cpu.reg_a = v + 1;
    assert_eq!(4, cpu.run_one(&mut mem));
    cpu.reg_a = v + 2;
    assert_eq!(4, cpu.run_one(&mut mem));
    cpu.reg_a = v + 3;
    assert_eq!(5, cpu.run_one(&mut mem));
    cpu.reg_a = v + 4;
    assert_eq!(5, cpu.run_one(&mut mem));
    cpu.reg_a = v + 5;
    assert_eq!(6, cpu.run_one(&mut mem));
    cpu.reg_a = v + 6;
    assert_eq!(6, cpu.run_one(&mut mem));

    assert_eq!(v + 0, mem[zpg as usize]);
    assert_eq!(v + 1, mem[(zpg + x) as usize]);
    assert_eq!(v + 2, mem[abs as usize]);
    assert_eq!(v + 3, mem[(abs + x as u16) as usize]);
    assert_eq!(v + 4, mem[(abs + y as u16) as usize]);
    assert_eq!(v + 5, mem[inx as usize]);
    assert_eq!(v + 6, mem[(iny + y as u16) as usize]);
}

#[test]
fn test_stx() {
    // Values
    for i in 0..=0xff {
        let (mut cpu, mut mem) = test_cpu(&vec![STX_ZPG, 0x00]);
        cpu.reg_x = i;
        mem[0x00] = !i;
        cpu.run_one(&mut mem);
        assert_eq!(i, mem[0x00]);
    }

    // Addressing
    let y = 0x20;
    let v = 0x40;
    let zpg = 0x80;
    let abs = 0x1234;
    let (mut cpu, mut mem) = test_cpu(&vec![
        STX_ZPG, zpg,
        STX_ZPY, zpg,
        STX_ABS, lo(abs), hi(abs),
    ]);
    cpu.reg_y = y;

    cpu.reg_x = v + 0;
    assert_eq!(3, cpu.run_one(&mut mem));
    cpu.reg_x = v + 1;
    assert_eq!(4, cpu.run_one(&mut mem));
    cpu.reg_x = v + 2;
    assert_eq!(4, cpu.run_one(&mut mem));

    assert_eq!(v + 0, mem[zpg as usize]);
    assert_eq!(v + 1, mem[(zpg + y) as usize]);
    assert_eq!(v + 2, mem[abs as usize]);
}

#[test]
fn test_sty() {
    // Values
    for i in 0..=0xff {
        let (mut cpu, mut mem) = test_cpu(&vec![STY_ZPG, 0x00]);
        cpu.reg_y = i;
        mem[0x00] = !i;
        cpu.run_one(&mut mem);
        assert_eq!(i, mem[0x00]);
    }

    // Addressing
    let x = 0x10;
    let v = 0x40;
    let zpg = 0x80;
    let abs = 0x1234;
    let (mut cpu, mut mem) = test_cpu(&vec![
        STY_ZPG, zpg,
        STY_ZPX, zpg,
        STY_ABS, lo(abs), hi(abs),
    ]);
    cpu.reg_x = x;

    cpu.reg_y = v + 0;
    assert_eq!(3, cpu.run_one(&mut mem));
    cpu.reg_y = v + 1;
    assert_eq!(4, cpu.run_one(&mut mem));
    cpu.reg_y = v + 2;
    assert_eq!(4, cpu.run_one(&mut mem));

    assert_eq!(v + 0, mem[zpg as usize]);
    assert_eq!(v + 1, mem[(zpg + x) as usize]);
    assert_eq!(v + 2, mem[abs as usize]);
}
