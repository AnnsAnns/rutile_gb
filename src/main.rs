#![feature(exclusive_range_pattern)]

use cpu::CPU;

mod cpu;
mod memory;
mod ui;

use eframe::egui;
use ui::MyApp;

fn main() {
    let cpu: CPU = CPU::new();
    let ui = MyApp::init(cpu);

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 1250.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Rutil Gameboy Emulator - by AnnsAnn",
        options,
        Box::new(|_cc| Box::new(ui)),
    ).unwrap();
}
