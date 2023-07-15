use self::instructions::Instructions;

mod registry;
mod flags;
mod instructions;

pub struct CPU {
    // The Main Engine of the Emulator
    registry: registry::CPURegistry,
}