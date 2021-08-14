mod rp2a03;

fn main() {
    println!("{}: {}, {}, {}, {}",
        rp2a03::names::ADC_IMM,
        rp2a03::opcodes::ADC_IMM,
        rp2a03::bytes::ADC_IMM,
        rp2a03::cycles::ADC_IMM,
        rp2a03::affected_flags::ADC_IMM);
}
