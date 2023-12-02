use std::env;
use std::fs;

// main documentation
// https://gbdev.io/pandocs/Specifications.html
const BYTE: usize = 1;
const KB: usize = 1024 * BYTE;
const MB: usize = 1024 * KB;

/**
 * 32 Kib
 */
const WRAM_SIZE: usize = 0x8000;
/**
 * 16 Kib
 */
const VRAM_SIZE: usize = 0x4000;
/**
 * Some games up to 8 Mib
 * Pokemon Red/Blue: 1 Mib
 * 8 Mib
 */
const ROM_SIZE: usize = 0x800000;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No ROM file provided");
    }
    let path_to_rom = &args[1];
    let rom_content = fs::read(path_to_rom).expect("Unable to read file");
    print_size(&rom_content);
    let mut cpu = CpuSm83::new();
}

struct CpuSm83 {
    /**
     * 8-bit registers
     * A: 0 Accumulator
     * F: 1 Flags
     * B: 2
     * C: 3
     * D: 4
     * E: 5
     * H: 6
     * L: 7
     */
    cpu_regs: [u8; 8],
    /**
     * Stack Pointer
     */
    sp: u16,
    /**
     * Program Counter
     */
    pc: u16,
    wram: [u8; WRAM_SIZE],
    vram: [u8; VRAM_SIZE],
    rom: [u8; ROM_SIZE],
}

impl CpuSm83 {
    pub fn new() -> Self {
        CpuSm83 {
            sp: 0,
            pc: 0,
            cpu_regs: [0; 8],
            wram: [0; WRAM_SIZE],
            vram: [0; VRAM_SIZE],
            rom: [0; ROM_SIZE],
        }
    }

    fn read_hl(&self) -> u16 {
        let h = self.cpu_regs[6];
        let l = self.cpu_regs[7];
        let hl = (h as u16) << 8 | l as u16;
        hl
    }

    pub fn idx_regs(&self, encoded_register: u8) -> usize {
        match encoded_register {
            // A
            0b111 => 0,
            // B
            0b000 => 2,
            // C
            0b001 => 3,
            // D
            0b010 => 4,
            // E
            0b011 => 5,
            // H
            0b100 => 6,
            // L
            0b101 => 7,
            _ => panic!("Invalid encoded register"),
        }
    }
}

// Instructions
impl CpuSm83 {
    pub fn ld_rs(&mut self, r: u8, s: u8) {
        if s == 0xe {
            let hl = self.read_hl();
            let value = self.access_memory(hl);
            let r_idx = self.idx_regs(r);
            self.cpu_regs[r_idx] = value;
        }
    }
}

// Memory
impl CpuSm83 {
    pub fn access_memory(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3fff => self.rom[address as usize],
            // 0x4000..=0x7fff => self.rom[address as usize], (TODO: bank switching)
            0x8000..=0x9fff => self.vram[address as usize],
            0xc000..=0xcfff => self.wram[address as usize],
            // 0xd000..=0xdfff => self.wram[address as usize], (TODO: bank switching)
            _ => unimplemented!(),
        }
    }
}

fn print_size(vec: &Vec<u8>) {
    let size = vec.len();
    let readable_size = match size {
        x if x >= MB => format!("{:.2} MB", size as f64 / MB as f64),
        x if x >= KB => format!("{:.2} KB", size as f64 / KB as f64),
        _ => unimplemented!(),
    };

    println!("Size: {}", readable_size);
}
