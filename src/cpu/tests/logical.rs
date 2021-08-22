use super::*;

#[test]
fn test_and() {
    // Values/flags
    for mask in 0..=0xff {
        for i in 0..=0xff {
            let (mut cpu, mut mem) = test_cpu(&vec![AND_IMM, mask]);
            cpu.reg_a = i;
            let expected = i & mask;
            cpu.run_one(&mut mem);
            assert_eq!(expected, cpu.reg_a);
            assert_zn!(cpu, expected == 0, expected >= 0x80);
        }
    }

    // Addressing
    let x = 0x10;
    let y = 0x20;
    let y2 = 0xc0;
    let v = 0x01u8;
    let zpg = 0x80;
    let zpg_inx = 0xa0;
    let inx = 0x6789;
    let zpg_iny = 0xc0;
    let iny = 0x3456;
    let abs = 0x1234;
    let abs2 = 0x12f8;
    let (mut cpu, mut mem) = test_cpu(&vec![
        AND_IMM, v.rotate_left(0),
        AND_ZPG, zpg,
        AND_ZPX, zpg,
        AND_ABS, lo(abs), hi(abs),
        AND_ABX, lo(abs), hi(abs),
        AND_ABY, lo(abs), hi(abs),
        AND_INX, zpg_inx,
        AND_INY, zpg_iny,

        // Page crossing
        AND_ABX, lo(abs2), hi(abs2),
        AND_ABY, lo(abs2), hi(abs2),
        AND_INY, zpg_iny,
    ]);
    cpu.reg_x = x;
    cpu.reg_y = y;
    mem[zpg as usize] = v.rotate_left(1);
    mem[(zpg + x) as usize] = v.rotate_left(2);
    mem[(zpg_inx + x) as usize] = lo(inx);
    mem[(zpg_inx + x + 1) as usize] = hi(inx);
    mem[zpg_iny as usize] = lo(iny);
    mem[(zpg_iny + 1) as usize] = hi(iny);
    mem[abs as usize] = v.rotate_left(3);
    mem[(abs + x as u16) as usize] = v.rotate_left(4);
    mem[(abs + y as u16) as usize] = v.rotate_left(5);
    mem[inx as usize] = v.rotate_left(6);
    mem[(iny + y as u16) as usize] = v.rotate_left(7);
    mem[(abs2 + x as u16) as usize] = v.rotate_left(8);
    mem[(abs2 + y as u16) as usize] = v.rotate_left(9);
    mem[(iny + y2 as u16) as usize] = v.rotate_left(10);

    cpu.reg_a = 0xff;
    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(0), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(3, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(1), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(2), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(3), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(4), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(5), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(6, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(6), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(7), cpu.reg_a);

    cpu.reg_a = 0xff;
    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(8), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(9), cpu.reg_a);
    cpu.reg_y = y2;
    cpu.reg_a = 0xff;
    assert_eq!(6, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(10), cpu.reg_a);
}

#[test]
fn test_eor() {
    // Values/flags
    for mask in 0..=0xff {
        for i in 0..=0xff {
            let (mut cpu, mut mem) = test_cpu(&vec![EOR_IMM, mask]);
            cpu.reg_a = i;
            let expected = i ^ mask;
            cpu.run_one(&mut mem);
            assert_eq!(expected, cpu.reg_a);
            assert_zn!(cpu, expected == 0, expected >= 0x80);
        }
    }

    // Addressing
    let x = 0x10;
    let y = 0x20;
    let y2 = 0xc0;
    let v = 0x01u8;
    let zpg = 0x80;
    let zpg_inx = 0xa0;
    let inx = 0x6789;
    let zpg_iny = 0xc0;
    let iny = 0x3456;
    let abs = 0x1234;
    let abs2 = 0x12f8;
    let (mut cpu, mut mem) = test_cpu(&vec![
        EOR_IMM, v.rotate_left(0),
        EOR_ZPG, zpg,
        EOR_ZPX, zpg,
        EOR_ABS, lo(abs), hi(abs),
        EOR_ABX, lo(abs), hi(abs),
        EOR_ABY, lo(abs), hi(abs),
        EOR_INX, zpg_inx,
        EOR_INY, zpg_iny,

        // Page crossing
        EOR_ABX, lo(abs2), hi(abs2),
        EOR_ABY, lo(abs2), hi(abs2),
        EOR_INY, zpg_iny,
    ]);
    cpu.reg_x = x;
    cpu.reg_y = y;
    mem[zpg as usize] = v.rotate_left(1);
    mem[(zpg + x) as usize] = v.rotate_left(2);
    mem[(zpg_inx + x) as usize] = lo(inx);
    mem[(zpg_inx + x + 1) as usize] = hi(inx);
    mem[zpg_iny as usize] = lo(iny);
    mem[(zpg_iny + 1) as usize] = hi(iny);
    mem[abs as usize] = v.rotate_left(3);
    mem[(abs + x as u16) as usize] = v.rotate_left(4);
    mem[(abs + y as u16) as usize] = v.rotate_left(5);
    mem[inx as usize] = v.rotate_left(6);
    mem[(iny + y as u16) as usize] = v.rotate_left(7);
    mem[(abs2 + x as u16) as usize] = v.rotate_left(8);
    mem[(abs2 + y as u16) as usize] = v.rotate_left(9);
    mem[(iny + y2 as u16) as usize] = v.rotate_left(10);

    cpu.reg_a = 0xff;
    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(!v.rotate_left(0), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(3, cpu.run_one(&mut mem));
    assert_eq!(!v.rotate_left(1), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(!v.rotate_left(2), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(!v.rotate_left(3), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(!v.rotate_left(4), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(!v.rotate_left(5), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(6, cpu.run_one(&mut mem));
    assert_eq!(!v.rotate_left(6), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(!v.rotate_left(7), cpu.reg_a);

    cpu.reg_a = 0xff;
    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(!v.rotate_left(8), cpu.reg_a);
    cpu.reg_a = 0xff;
    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(!v.rotate_left(9), cpu.reg_a);
    cpu.reg_y = y2;
    cpu.reg_a = 0xff;
    assert_eq!(6, cpu.run_one(&mut mem));
    assert_eq!(!v.rotate_left(10), cpu.reg_a);
}

#[test]
fn test_ora() {
    // Values/flags
    for mask in 0..=0xff {
        for i in 0..=0xff {
            let (mut cpu, mut mem) = test_cpu(&vec![ORA_IMM, mask]);
            cpu.reg_a = i;
            let expected = i | mask;
            cpu.run_one(&mut mem);
            assert_eq!(expected, cpu.reg_a);
            assert_zn!(cpu, expected == 0, expected >= 0x80);
        }
    }

    // Addressing
    let x = 0x10;
    let y = 0x20;
    let y2 = 0xc0;
    let v = 0x01u8;
    let zpg = 0x80;
    let zpg_inx = 0xa0;
    let inx = 0x6789;
    let zpg_iny = 0xc0;
    let iny = 0x3456;
    let abs = 0x1234;
    let abs2 = 0x12f8;
    let (mut cpu, mut mem) = test_cpu(&vec![
        ORA_IMM, v.rotate_left(0),
        ORA_ZPG, zpg,
        ORA_ZPX, zpg,
        ORA_ABS, lo(abs), hi(abs),
        ORA_ABX, lo(abs), hi(abs),
        ORA_ABY, lo(abs), hi(abs),
        ORA_INX, zpg_inx,
        ORA_INY, zpg_iny,

        // Page crossing
        ORA_ABX, lo(abs2), hi(abs2),
        ORA_ABY, lo(abs2), hi(abs2),
        ORA_INY, zpg_iny,
    ]);
    cpu.reg_x = x;
    cpu.reg_y = y;
    mem[zpg as usize] = v.rotate_left(1);
    mem[(zpg + x) as usize] = v.rotate_left(2);
    mem[(zpg_inx + x) as usize] = lo(inx);
    mem[(zpg_inx + x + 1) as usize] = hi(inx);
    mem[zpg_iny as usize] = lo(iny);
    mem[(zpg_iny + 1) as usize] = hi(iny);
    mem[abs as usize] = v.rotate_left(3);
    mem[(abs + x as u16) as usize] = v.rotate_left(4);
    mem[(abs + y as u16) as usize] = v.rotate_left(5);
    mem[inx as usize] = v.rotate_left(6);
    mem[(iny + y as u16) as usize] = v.rotate_left(7);
    mem[(abs2 + x as u16) as usize] = v.rotate_left(8);
    mem[(abs2 + y as u16) as usize] = v.rotate_left(9);
    mem[(iny + y2 as u16) as usize] = v.rotate_left(10);

    cpu.reg_a = 0x00;
    assert_eq!(2, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(0), cpu.reg_a);
    cpu.reg_a = 0x00;
    assert_eq!(3, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(1), cpu.reg_a);
    cpu.reg_a = 0x00;
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(2), cpu.reg_a);
    cpu.reg_a = 0x00;
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(3), cpu.reg_a);
    cpu.reg_a = 0x00;
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(4), cpu.reg_a);
    cpu.reg_a = 0x00;
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(5), cpu.reg_a);
    cpu.reg_a = 0x00;
    assert_eq!(6, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(6), cpu.reg_a);
    cpu.reg_a = 0x00;
    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(7), cpu.reg_a);

    cpu.reg_a = 0x00;
    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(8), cpu.reg_a);
    cpu.reg_a = 0x00;
    assert_eq!(5, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(9), cpu.reg_a);
    cpu.reg_y = y2;
    cpu.reg_a = 0x00;
    assert_eq!(6, cpu.run_one(&mut mem));
    assert_eq!(v.rotate_left(10), cpu.reg_a);
}

#[test]
fn test_bit() {
    // Values/flags
    for mask in 0..=0xff {
        for i in 0..=0xff {
            let (mut cpu, mut mem) = test_cpu(&vec![BIT_ZPG, 0x00]);
            mem[0x00] = i;
            cpu.reg_a = mask;
            cpu.run_one(&mut mem);
            assert_eq!(i & mask == 0, cpu.flag_zero);
            assert_eq!(i & 0x40 > 0, cpu.flag_overflow);
            assert_eq!(i & 0x80 > 0, cpu.flag_negative);
        }
    }

    // Addressing
    let v = 0x01;
    let zpg = 0x80;
    let abs = 0x1234;
    let (mut cpu, mut mem) = test_cpu(&vec![
        BIT_ZPG, zpg,
        BIT_ABS, lo(abs), hi(abs),
    ]);
    cpu.reg_a = 0x01;
    mem[zpg as usize] = v + 0;
    mem[abs as usize] = v + 1;

    assert_eq!(3, cpu.run_one(&mut mem));
    assert_eq!(false, cpu.flag_zero);
    assert_eq!(4, cpu.run_one(&mut mem));
    assert_eq!(true, cpu.flag_zero);
}
