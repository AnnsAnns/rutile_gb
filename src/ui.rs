use eframe::{egui::{self, RichText}, epaint::Color32};

use crate::cpu::CPU;

pub struct MyApp {
    name: String,
    speed: u64,
    cpu: CPU,
    halt: bool,
    img: egui::ColorImage
}

impl MyApp {
    pub fn init(cpu: CPU) -> Self {
        Self {
            name: "Arthur".to_owned(),
            speed: 1,
            cpu: cpu,
            halt: false,
            img: egui::ColorImage::new([160, 144], Color32::WHITE)
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.halt && (ctx.frame_nr() % (100 - self.speed) == 0) {
            self.cpu.step();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let texture: egui::TextureHandle = ui.ctx().load_texture(
                "my-image",
                self.img.to_owned(),
                Default::default()
            );

            ui.heading("Rutil Gameboy Emulator");
            ui.add(egui::Slider::new(&mut self.speed, 0..=100).text("Emulator Speed"));
            if ui.button("Stop/Resume").clicked() {
                self.halt = !self.halt;
            }
            ui.horizontal(|ui| {
                ui.image(&texture, texture.size_vec2());
                ui.vertical(|ui| {
                    ui.label(RichText::new("Registers:").strong().underline());
                    ui.label(format!("
                        B: {:04X}\n
                        C: {:04X}\n
                        D: {:04X}\n
                        E: {:04X}\n
                        H: {:04X}\n
                        L: {:04X}\n
                        A: {:04X}\n
                        SP: {:08X}\n
                        PC: {:08X}\n
                        ",
                        self.cpu.registry.b,
                        self.cpu.registry.c,
                        self.cpu.registry.d,
                        self.cpu.registry.e,
                        self.cpu.registry.h,
                        self.cpu.registry.l,
                        self.cpu.registry.a,
                        self.cpu.registry.sp,
                        self.cpu.registry.pc,
                        ));
                });
                ui.vertical(|ui| {
                    ui.label(RichText::new("Flags:").strong().underline());
                    ui.label(format!("
                        Z: {}\n
                        N: {}\n
                        H: {}\n
                        C: {}\n
                        ",
                        self.cpu.registry.f.z_zero,
                        self.cpu.registry.f.n_subtraction_bcd,
                        self.cpu.registry.f.h_half_carry_bcd,
                        self.cpu.registry.f.c_carry,
                        ));
                });
                ui.vertical(|ui| {
                    ui.label(RichText::new("Instruction:").strong().underline());
                    ui.label(format!("{:?}", self.cpu.last_instruction))
                })
            });
            ui.vertical(|ui| {
                ui.label(RichText::new("Memory:").strong().underline());
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(egui::RichText::new(format!("{:?}", self.cpu.memory.memory)).monospace());
                });
            })
        });

        // std::thread::sleep(std::time::Duration::from_millis(1000 / self.speed as u64));
    }
}