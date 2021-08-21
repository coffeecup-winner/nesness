use super::*;

#[test]
fn test_asl() {
    // Values/flags
    let (mut cpu, mut ram) = test_cpu(&vec![ASL_ACC; 10]);
    cpu.reg_a = 0b1111_1111;
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b1111_1110, cpu.reg_a);
    assert_zn!(cpu, false, true);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b1111_1100, cpu.reg_a);
    assert_zn!(cpu, false, true);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b1111_1000, cpu.reg_a);
    assert_zn!(cpu, false, true);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b1111_0000, cpu.reg_a);
    assert_zn!(cpu, false, true);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b1110_0000, cpu.reg_a);
    assert_zn!(cpu, false, true);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b1100_0000, cpu.reg_a);
    assert_zn!(cpu, false, true);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b1000_0000, cpu.reg_a);
    assert_zn!(cpu, false, true);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0000, cpu.reg_a);
    assert_zn!(cpu, true, false);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0000, cpu.reg_a);
    assert_zn!(cpu, true, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0000, cpu.reg_a);
    assert_zn!(cpu, true, false);
    assert_eq!(false, cpu.flag_carry);

    // Addressing
    let x = 0x10;
    let zpg = 0x80;
    let abs = 0x1234;
    let (mut cpu, mut ram) = test_cpu(&vec![
        ASL_ZPG, zpg,
        ASL_ZPX, zpg,
        ASL_ABS, lo(abs), hi(abs),
        ASL_ABX, lo(abs), hi(abs),
    ]);
    cpu.reg_x = x;
    ram[zpg as usize] = 1 << 0;
    ram[(zpg + x) as usize] = 1 << 1;
    ram[abs as usize] = 1 << 2;
    ram[(abs + x as u16) as usize] = 1 << 3;

    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(7, cpu.run_one(&mut ram));

    assert_eq!(1 << 1, ram[zpg as usize]);
    assert_eq!(1 << 2, ram[(zpg + x) as usize]);
    assert_eq!(1 << 3, ram[abs as usize]);
    assert_eq!(1 << 4, ram[(abs + x as u16) as usize]);
}

#[test]
fn test_lsr() {
    // Values/flags
    let (mut cpu, mut ram) = test_cpu(&vec![LSR_ACC; 10]);
    cpu.reg_a = 0b1111_1111;
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0111_1111, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0011_1111, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0001_1111, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_1111, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0111, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0011, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0001, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0000, cpu.reg_a);
    assert_zn!(cpu, true, false);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0000, cpu.reg_a);
    assert_zn!(cpu, true, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0000, cpu.reg_a);
    assert_zn!(cpu, true, false);
    assert_eq!(false, cpu.flag_carry);

    // Addressing
    let x = 0x10;
    let zpg = 0x80;
    let abs = 0x1234;
    let (mut cpu, mut ram) = test_cpu(&vec![
        LSR_ZPG, zpg,
        LSR_ZPX, zpg,
        LSR_ABS, lo(abs), hi(abs),
        LSR_ABX, lo(abs), hi(abs),
    ]);
    cpu.reg_x = x;
    ram[zpg as usize] = 1 << 1;
    ram[(zpg + x) as usize] = 1 << 2;
    ram[abs as usize] = 1 << 3;
    ram[(abs + x as u16) as usize] = 1 << 4;

    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(7, cpu.run_one(&mut ram));

    assert_eq!(1 << 0, ram[zpg as usize]);
    assert_eq!(1 << 1, ram[(zpg + x) as usize]);
    assert_eq!(1 << 2, ram[abs as usize]);
    assert_eq!(1 << 3, ram[(abs + x as u16) as usize]);
}

#[test]
fn test_rol() {
    // Values/flags
    let (mut cpu, mut ram) = test_cpu(&vec![ROL_ACC; 10]);
    cpu.reg_a = 0b0000_0001;
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0010, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0100, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_1000, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0001_0000, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0010_0000, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0100_0000, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b1000_0000, cpu.reg_a);
    assert_zn!(cpu, false, true);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0000, cpu.reg_a);
    assert_zn!(cpu, true, false);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0001, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0010, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);

    // Addressing
    let x = 0x10;
    let zpg = 0x80;
    let abs = 0x1234;
    let (mut cpu, mut ram) = test_cpu(&vec![
        ROL_ZPG, zpg,
        ROL_ZPX, zpg,
        ROL_ABS, lo(abs), hi(abs),
        ROL_ABX, lo(abs), hi(abs),
    ]);
    cpu.reg_x = x;
    ram[zpg as usize] = 1 << 0;
    ram[(zpg + x) as usize] = 1 << 1;
    ram[abs as usize] = 1 << 2;
    ram[(abs + x as u16) as usize] = 1 << 3;

    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(7, cpu.run_one(&mut ram));

    assert_eq!(1 << 1, ram[zpg as usize]);
    assert_eq!(1 << 2, ram[(zpg + x) as usize]);
    assert_eq!(1 << 3, ram[abs as usize]);
    assert_eq!(1 << 4, ram[(abs + x as u16) as usize]);
}

#[test]
fn test_ror() {
    // Values/flags
    let (mut cpu, mut ram) = test_cpu(&vec![ROR_ACC; 10]);
    cpu.reg_a = 0b0000_0001;
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0000, cpu.reg_a);
    assert_zn!(cpu, true, false);
    assert_eq!(true, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b1000_0000, cpu.reg_a);
    assert_zn!(cpu, false, true);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0100_0000, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0010_0000, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0001_0000, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_1000, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0100, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0010, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0001, cpu.reg_a);
    assert_zn!(cpu, false, false);
    assert_eq!(false, cpu.flag_carry);
    assert_eq!(2, cpu.run_one(&mut ram));
    assert_eq!(0b0000_0000, cpu.reg_a);
    assert_zn!(cpu, true, false);
    assert_eq!(true, cpu.flag_carry);

    // Addressing
    let x = 0x10;
    let zpg = 0x80;
    let abs = 0x1234;
    let (mut cpu, mut ram) = test_cpu(&vec![
        ROR_ZPG, zpg,
        ROR_ZPX, zpg,
        ROR_ABS, lo(abs), hi(abs),
        ROR_ABX, lo(abs), hi(abs),
    ]);
    cpu.reg_x = x;
    ram[zpg as usize] = 1 << 1;
    ram[(zpg + x) as usize] = 1 << 2;
    ram[abs as usize] = 1 << 3;
    ram[(abs + x as u16) as usize] = 1 << 4;

    assert_eq!(5, cpu.run_one(&mut ram));
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(6, cpu.run_one(&mut ram));
    assert_eq!(7, cpu.run_one(&mut ram));

    assert_eq!(1 << 0, ram[zpg as usize]);
    assert_eq!(1 << 1, ram[(zpg + x) as usize]);
    assert_eq!(1 << 2, ram[abs as usize]);
    assert_eq!(1 << 3, ram[(abs + x as u16) as usize]);
}
