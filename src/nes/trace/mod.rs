pub mod fceux;

pub trait ExecutionTrace {
    fn cycles_start_with_0(&self) -> bool;
    fn advance(&mut self) -> bool;

    // CPU state
    fn total_cycles(&self) -> u64;
    fn reg_a(&self) -> u8;
    fn reg_x(&self) -> u8;
    fn reg_y(&self) -> u8;
    fn reg_s(&self) -> u8;
    fn pc(&self) -> u16;
    fn flag_carry(&self) -> bool;
    fn flag_zero(&self) -> bool;
    fn flag_interrupt_disable(&self) -> bool;
    fn flag_decimal_mode(&self) -> bool;
    fn flag_break(&self) -> bool;
    fn flag_overflow(&self) -> bool;
    fn flag_negative(&self) -> bool;
}
