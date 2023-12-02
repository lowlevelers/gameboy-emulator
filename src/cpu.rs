use crate::constants::{MAX_CGB_RAM_SIZE, MAX_ROM_SIZE, MAX_VRAM_SIZE};

pub struct CpuSm83 {
    general_registers: GeneralRegisters,
    ram: [u8; MAX_ROM_SIZE],
    rom: [u8; MAX_CGB_RAM_SIZE],
    vram: [u8; MAX_VRAM_SIZE],
    sixteen_bit_registers: SixteenBitRegister,
    flag_register: u8,
    pc: u16, // program counter
    sp: u16, // stack pointer
}

pub struct SixteenBitRegister {
    /**
     * AF does not allow writing F bits 0–3
     */
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
}

pub struct GeneralRegisters {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl CpuSm83 {
    pub fn new() -> Self {
        CpuSm83 {
            ram: [0; MAX_ROM_SIZE],
            rom: [0; MAX_CGB_RAM_SIZE],
            vram: [0; MAX_VRAM_SIZE],
            general_registers: GeneralRegisters {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0,
            },
            sixteen_bit_registers: SixteenBitRegister {
                af: 0,
                bc: 0,
                de: 0,
                hl: 0,
            },
            flag_register: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        let mut cpu = CpuSm83::new();
        if rom.len() > MAX_ROM_SIZE {
            panic!("ROM size is too large");
        }

        cpu.rom[..rom.len()].copy_from_slice(rom);
    }
}

/*
    CGB stand for Color Game Boy
*/

/*
   8-bit flags register (F)
   Bits 0–3 are grounded to 0
   Bit 4: C (carry flag)
   Bit 5: H (half-carry flag)
   Bit 6: N (negative flag)
   Bit 7: Z (zero flag)
*/

/*
    The gameboy is having a 16bit address bus, that is used to address ROM, RAM, and I/O registers.
*/

/*
Memory Banking Explain
https://chat.openai.com/share/03874eba-dfcb-4216-8503-9fa1c729f2e0
*/

/*
0000-3FFF   16KB ROM Bank 00     (in cartridge, fixed at bank 00)
4000-7FFF   16KB ROM Bank 01..NN (in cartridge, switchable bank number)
8000-9FFF   8KB Video RAM (VRAM) (switchable bank 0-1 in CGB Mode)
A000-BFFF   8KB External RAM     (in cartridge, switchable bank, if any)
C000-CFFF   4KB Work RAM Bank 0 (WRAM)
D000-DFFF   4KB Work RAM Bank 1 (WRAM)  (switchable bank 1-7 in CGB Mode)
E000-FDFF   Same as C000-DDFF (ECHO)    (typically not used)
FE00-FE9F   Sprite Attribute Table (OAM)
FEA0-FEFF   Not Usable
FF00-FF7F   I/O Ports
FF80-FFFE   High RAM (HRAM)
FFFF        Interrupt Enable Register
*/
