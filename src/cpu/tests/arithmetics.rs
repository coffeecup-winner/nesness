use super::*;

#[test]
fn test_adc() {
    // Values/flags
    for carry in 0..=1 {
        for a in 0..=0xff {
            for b in 0..=0xff {
                let (mut cpu, mut ram) = test_cpu(&vec![ADC_IMM, b]);
                cpu.reg_a = a;
                cpu.flag_carry = carry != 0;
                let result_u16 = a as u16 + b as u16 + carry;
                let result_i16 = a as i8 as i16 + b as i8 as i16 + carry as i16;
                cpu.run_one(&mut ram);
                assert_eq!(result_u16 as u8, cpu.reg_a);
                assert_zn!(cpu, result_u16 as u8 == 0, (result_u16 & 0x80) != 0);
                assert_eq!((result_u16 & 0x0100) != 0, cpu.flag_carry);
                assert_eq!((result_i16 < -128) || (result_i16 > 127), cpu.flag_overflow);
            }
        }
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
    let (mut cpu, mut ram) = test_cpu(&vec![
        ADC_IMM, 0,
        ADC_ZPG, zpg,
        ADC_ZPX, zpg,
        ADC_ABS, lo(abs), hi(abs),
        ADC_ABX, lo(abs), hi(abs),
        ADC_ABY, lo(abs), hi(abs),
        ADC_INX, zpg_inx,
        ADC_INY, zpg_iny,

        // Page crossing
        ADC_ABX, lo(abs2), hi(abs2),
        ADC_ABY, lo(abs2), hi(abs2),
        ADC_INY, zpg_iny,
    ]);
    cpu.reg_x = x;
    cpu.reg_y = y;
    ram[zpg as usize] = 1;
    ram[(zpg + x) as usize] = 2;
    ram[(zpg_inx + x) as usize] = lo(inx);
    ram[(zpg_inx + x + 1) as usize] = hi(inx);
    ram[zpg_iny as usize] = lo(iny);
    ram[(zpg_iny + 1) as usize] = hi(iny);
    ram[abs as usize] = 3;
    ram[(abs + x as u16) as usize] = 4;
    ram[(abs + y as u16) as usize] = 5;
    ram[inx as usize] = 6;
    ram[(iny + y as u16) as usize] = 7;
    ram[(abs2 + x as u16) as usize] = 8;
    ram[(abs2 + y as u16) as usize] = 9;
    ram[(iny + y2 as u16) as usize] = 10;

    cpu.reg_a = v;
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(v + 0, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(3, cpu.run_one(&mut ram));
    assert_eq!(v + 1, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(v + 2, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(v + 3, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(v + 4, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(v + 5, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(v + 6, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(v + 7, cpu.reg_a);

    cpu.reg_a = v;
    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(v + 8, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(v + 9, cpu.reg_a);
    cpu.reg_y = y2;
    cpu.reg_a = v;
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(v + 10, cpu.reg_a);
}

#[test]
fn test_sbc() {
    // Values/flags
    for borrow in 0..=1 {
        for a in 0..=0xff {
            for b in 0..=0xff {
                let (mut cpu, mut ram) = test_cpu(&vec![SBC_IMM, b]);
                cpu.reg_a = a;
                cpu.flag_carry = borrow == 0;
                let result_u16 = (a as u16).wrapping_sub(b as u16).wrapping_sub(borrow);
                let result_i16 = (a as i8 as i16).wrapping_sub(b as i8 as i16).wrapping_sub(borrow as i16);
                cpu.run_one(&mut ram);
                assert_eq!(result_u16 as u8, cpu.reg_a);
                assert_zn!(cpu, result_u16 as u8 == 0, (result_u16 & 0x80) != 0);
                assert_eq!((result_u16 & 0x0100) == 0, cpu.flag_carry);
                assert_eq!((result_i16 < -128) || (result_i16 > 127), cpu.flag_overflow);
            }
        }
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
    let (mut cpu, mut ram) = test_cpu(&vec![
        SBC_IMM, 0,
        SBC_ZPG, zpg,
        SBC_ZPX, zpg,
        SBC_ABS, lo(abs), hi(abs),
        SBC_ABX, lo(abs), hi(abs),
        SBC_ABY, lo(abs), hi(abs),
        SBC_INX, zpg_inx,
        SBC_INY, zpg_iny,

        // Page crossing
        SBC_ABX, lo(abs2), hi(abs2),
        SBC_ABY, lo(abs2), hi(abs2),
        SBC_INY, zpg_iny,
    ]);
    cpu.reg_x = x;
    cpu.reg_y = y;
    ram[zpg as usize] = 1;
    ram[(zpg + x) as usize] = 2;
    ram[(zpg_inx + x) as usize] = lo(inx);
    ram[(zpg_inx + x + 1) as usize] = hi(inx);
    ram[zpg_iny as usize] = lo(iny);
    ram[(zpg_iny + 1) as usize] = hi(iny);
    ram[abs as usize] = 3;
    ram[(abs + x as u16) as usize] = 4;
    ram[(abs + y as u16) as usize] = 5;
    ram[inx as usize] = 6;
    ram[(iny + y as u16) as usize] = 7;
    ram[(abs2 + x as u16) as usize] = 8;
    ram[(abs2 + y as u16) as usize] = 9;
    ram[(iny + y2 as u16) as usize] = 10;
    cpu.flag_carry = true;

    cpu.reg_a = v;
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(v - 0, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(3, cpu.run_one(&mut ram));
    assert_eq!(v - 1, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(v - 2, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(v - 3, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(v - 4, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(v - 5, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(v - 6, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(v - 7, cpu.reg_a);

    cpu.reg_a = v;
    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(v - 8, cpu.reg_a);
    cpu.reg_a = v;
    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(v - 9, cpu.reg_a);
    cpu.reg_y = y2;
    cpu.reg_a = v;
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(v - 10, cpu.reg_a);
}

#[test]
fn test_cmp() {
    // Values/flags
    for a in 0..=0xff {
        for b in 0..=0xff {
            let (mut cpu, mut ram) = test_cpu(&vec![CMP_IMM, b]);
            cpu.reg_a = a;
            let result = a.wrapping_sub(b);
            cpu.run_one(&mut ram);
            assert_zn!(cpu, a == b, (result & 0x80) != 0);
            assert_eq!(a >= b, cpu.flag_carry);
        }
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
    let (mut cpu, mut ram) = test_cpu(&vec![
        CMP_IMM, v + 0,
        CMP_ZPG, zpg,
        CMP_ZPX, zpg,
        CMP_ABS, lo(abs), hi(abs),
        CMP_ABX, lo(abs), hi(abs),
        CMP_ABY, lo(abs), hi(abs),
        CMP_INX, zpg_inx,
        CMP_INY, zpg_iny,

        // Page crossing
        CMP_ABX, lo(abs2), hi(abs2),
        CMP_ABY, lo(abs2), hi(abs2),
        CMP_INY, zpg_iny,
    ]);
    cpu.reg_x = x;
    cpu.reg_y = y;
    ram[zpg as usize] = v + 1;
    ram[(zpg + x) as usize] = v + 2;
    ram[(zpg_inx + x) as usize] = lo(inx);
    ram[(zpg_inx + x + 1) as usize] = hi(inx);
    ram[zpg_iny as usize] = lo(iny);
    ram[(zpg_iny + 1) as usize] = hi(iny);
    ram[abs as usize] = v + 3;
    ram[(abs + x as u16) as usize] = v + 4;
    ram[(abs + y as u16) as usize] = v + 5;
    ram[inx as usize] = v + 6;
    ram[(iny + y as u16) as usize] = v + 7;
    ram[(abs2 + x as u16) as usize] = v + 8;
    ram[(abs2 + y as u16) as usize] = v + 9;
    ram[(iny + y2 as u16) as usize] = v + 10;

    cpu.reg_a = v;
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(3, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);

    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
    cpu.reg_y = y2;
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
}

#[test]
fn test_cpx() {
    // Values/flags
    for a in 0..=0xff {
        for b in 0..=0xff {
            let (mut cpu, mut ram) = test_cpu(&vec![CPX_IMM, b]);
            cpu.reg_x = a;
            let result = a.wrapping_sub(b);
            cpu.run_one(&mut ram);
            assert_zn!(cpu, a == b, (result & 0x80) != 0);
            assert_eq!(a >= b, cpu.flag_carry);
        }
    }

    // Addressing
    let v = 0x40;
    let zpg = 0x80;
    let abs = 0x1234;
    let (mut cpu, mut ram) = test_cpu(&vec![
        CPX_IMM, v + 0,
        CPX_ZPG, zpg,
        CPX_ABS, lo(abs), hi(abs),
    ]);
    ram[zpg as usize] = v + 1;
    ram[abs as usize] = v + 2;

    cpu.reg_x = v;
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(3, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
}

#[test]
fn test_cpy() {
    // Values/flags
    for a in 0..=0xff {
        for b in 0..=0xff {
            let (mut cpu, mut ram) = test_cpu(&vec![CPY_IMM, b]);
            cpu.reg_y = a;
            let result = a.wrapping_sub(b);
            cpu.run_one(&mut ram);
            assert_zn!(cpu, a == b, (result & 0x80) != 0);
            assert_eq!(a >= b, cpu.flag_carry);
        }
    }

    // Addressing
    let v = 0x40;
    let zpg = 0x80;
    let abs = 0x1234;
    let (mut cpu, mut ram) = test_cpu(&vec![
        CPY_IMM, v + 0,
        CPY_ZPG, zpg,
        CPY_ABS, lo(abs), hi(abs),
    ]);
    ram[zpg as usize] = v + 1;
    ram[abs as usize] = v + 2;

    cpu.reg_y = v;
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(3, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(4, cpu.run_one(&mut ram));
    assert_eq!(false, cpu.flag_carry);
}
