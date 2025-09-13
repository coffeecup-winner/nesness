use crate::nes::trace::fceux::FceuxTrace;
use crate::nes::NES;
use crate::rom::nes::NESFile;

use raylib::prelude::*;

pub fn gui_main() {
    let path = std::env::args().nth(1).expect("Expected an argument");
    let data = std::fs::read(path).expect("Failed to read the ROM file");
    let rom = NESFile::load(&data).expect("Failed to load the ROM");
    let mut nes = NES::new(rom);

    if let Some(trace_path) = std::env::args().nth(2) {
        let data = std::fs::read(trace_path).expect("Failed to read the trace file");
        let text = String::from_utf8_lossy(&data).into_owned();
        let trace = FceuxTrace::new(&text);
        nes.run_with_trace(trace);
        println!("Trace run finished");
        return;
    }

    let (mut rl, thread) = raylib::init()
        .size(256 * 4, 240 * 4)
        .title("NESNESS v0.1")
        .build();

    rl.set_target_fps(60);

    let mut last = std::time::Instant::now();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        #[cfg(debug_assertions)]
        {
            if d.is_key_down(KeyboardKey::KEY_D) {
                nes.dump();
                break;
            }
        }

        // TODO: Make this number internal to the NES type
        // TODO: Account for skipped dots
        for _ in 0..89342 * 4 {
            nes.tick();
        }

        for x in 0..256 {
            for y in 0..240 {
                let p = nes.mmap.ppu.frame_buffer[(y * 256 + x) as usize];
                let color = crate::ppu::palette::ppu_pixel_to_color(p);
                d.draw_rectangle(
                    x * 4,
                    y * 4,
                    4,
                    4,
                    Color::new(color.r, color.g, color.b, 255),
                );
            }
        }

        std::mem::drop(d);

        let now = std::time::Instant::now();
        let elapsed = now.duration_since(last);
        let fps = 1.0 / elapsed.as_secs_f32();
        last = now;
        rl.set_window_title(&thread, &format!("NESNESS v0.1 - FPS: {:.2}", fps));
    }
}
