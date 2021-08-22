mod cpu;
mod mem;
mod rom;

fn main() {
    for opcode in &cpu::rp2a03::opcodes::OPCODES[..] {
        let info = &cpu::rp2a03::info::INFO[*opcode as usize];
        println!(
            "{}: {}, {}, {}, {}",
            info.name, info.opcode, info.bytes, info.cycles, info.affected_flags
        );
    }
}
