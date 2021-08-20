mod cpu;
mod rp2a03;

fn main() {
    for opcode in &rp2a03::opcodes::OPCODES[..] {
        let info = &rp2a03::info::INFO[*opcode as usize];
        println!(
            "{}: {}, {}, {}, {}",
            info.name, info.opcode, info.bytes, info.cycles, info.affected_flags
        );
    }
}
