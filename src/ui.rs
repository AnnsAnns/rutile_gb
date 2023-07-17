use eframe::{egui::{self, RichText, Widget}, epaint::Color32};

use crate::cpu::CPU;
use crate::cpu::instructions::Instructions;

pub struct MyApp {
    speed: u64,
    cpu: CPU,
    halt: bool,
    single_step: bool,
    img: egui::ColorImage,
    picked_path: String,
}

impl MyApp {
    pub fn init(cpu: CPU) -> Self {
        Self {
            speed: 0,
            cpu: cpu,
            halt: false,
            img: egui::ColorImage::new([160, 144], Color32::WHITE),
            single_step: false,
            picked_path: "No Game Selected".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.single_step || ((self.speed == 100) ||(!self.halt && self.speed > 0 && (ctx.frame_nr() % (100 - self.speed) == 0))) {
            self.cpu.step();
            self.single_step = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let texture: egui::TextureHandle = ui.ctx().load_texture(
                "my-image",
                self.img.to_owned(),
                Default::default()
            );

            ui.heading(format!("Rutil Gameboy Emulator - {}", self.picked_path));
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = path.display().to_string();
                    let data: Vec<u8> = std::fs::read(path.display().to_string()).unwrap();
                    self.cpu.memory.load_rom(data)
                }
            }
            ui.add(egui::Slider::new(&mut self.speed, 0..=100).text("Emulator Speed"));
            if ui.button("Stop/Resume").clicked() {
                self.halt = !self.halt;
            }
            if ui.button("Single Step").clicked() {
                self.single_step = true;
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
                        Bootrom: {}\n
                        ",
                        self.cpu.registry.f.z_zero,
                        self.cpu.registry.f.n_subtraction_bcd,
                        self.cpu.registry.f.h_half_carry_bcd,
                        self.cpu.registry.f.c_carry,
                        self.cpu.memory.in_bootrom
                        ));
                });
                ui.vertical(|ui| {
                    ui.label(RichText::new("Instruction Info:").strong().underline());
                    ui.label(format!("Current Instruction: {:?}", self.cpu.last_instruction));
                    let mut opcode = self.cpu.memory.read_byte(self.cpu.registry.pc);
                    let prefixed = opcode == 0xCB;
                    if prefixed {
                        opcode = self.cpu.memory.read_byte(self.cpu.registry.pc + 1);
                    }
                    let next_instruction = Instructions::read_byte(opcode, prefixed).unwrap_or(Instructions::NOP());
                    ui.label(format!("Next Instruction if not SP change: {:?} - Opcode: {:02X}", next_instruction, opcode));
                    ui.label(format!("Is prefixed: {}", prefixed));
                    ui.label(format!("N8: {}", self.cpu.memory.read_byte(self.cpu.registry.sp)));
                    ui.label(format!("N16: {}", self.cpu.memory.read_word(self.cpu.registry.sp)));
                    ui.label(format!("E8: {}", self.cpu.memory.read_byte(self.cpu.registry.pc) as i8));
                })
            });
            ui.vertical(|ui| {
                ui.label(RichText::new("Memory:").strong().underline());
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(egui::RichText::new(format!("{:02X?}", self.cpu.memory.memory)).monospace());
                });
            })
        });

        ctx.request_repaint(); // The loop runs at VSYNC, Emulator runs on it's own speed
    }
}