use crate::nes::NES;
use crate::rom::nes::NESFile;

use raylib::prelude::*;

pub fn gui_main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("NESNESS v0.1")
        .build();

    rl.set_target_fps(60);

    let path = std::env::args().nth(1).expect("Expected an argument");
    let data = std::fs::read(path).expect("Failed to read the ROM file");
    let rom = NESFile::load(&data).expect("Failed to load the ROM");
    let mut nes = NES::new(rom);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::SKYBLUE);
        d.draw_text("TEST", 12, 12, 20, Color::BLACK);

        // TODO: Make this number internal to the NES type
        // TODO: Account for skipped dots
        for _ in 0..89342 * 4 {
            nes.tick();
        }
    }
}
