#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
//#![allow(overflowing_literals)]

use std::io::Read;
use std::fs::File;

fn main() {
    let init_address = 0;
    let mut c = 0;

    let mut emu = create_emu(init_address,0);
    
    let (v, s)  = read_binary("helloworld.bin");

    for i in 0..v.len() {
        emu.memory[i] = v[i]
    }

    while emu.memory[(emu.eip - init_address) as usize] != 0 {
        let value = emu.memory[(emu.eip - init_address) as usize];

        c += 1;
        if c > 2 {
            break
        }
        
        //println!("v: {}", value);
        // op branch
        if (0xb8 <= value) && (value < 0xb8 + 8) {
            println!("MOV, EIP: {}", emu.eip);
            emu_mov_r32_imm(&mut emu);
        } else if value == 0xeb {
            println!("JMP EIP: {}", emu.eip);
            emu_short_jmp(&mut emu);
        }
    }

    display(&mut emu);
}

struct Emulator {
    eip: i32,
    eflags: i32,
    esp: i32,
    registers: [i32; 16],
    memory: [u8; 128],    
}

fn create_emu(eip_init: i32, esp_init: i32) -> Emulator {
    let emu = Emulator{eip: eip_init, esp: esp_init, eflags: 0, registers: [0; 16], memory: [0; 128],};
    return emu
}

fn read_binary(target: &str) -> (Vec<u8>, usize) {

    let mut file = match File::open(target) {
        Err(e) => panic!("error: {}", e),
        Ok(file) => file,
    };

    let mut buffer = Vec::new();
    let size = file.read_to_end(&mut buffer).unwrap();
    return (buffer, size)
}

fn emu_mov_r32_imm(emu: &mut Emulator) {
    let data = get_data_u8(emu);
    let register_n = data - 0xb8;
    let imm = get_data32(emu, 1);
    emu.registers[register_n as usize] = imm;
    emu.eip += 5;
}

fn emu_short_jmp(emu: &mut Emulator) {
    let jmp_size = get_data_i8(emu, 1) as i32;
    println!("jmp_s: {}", jmp_size);
    emu.eip += jmp_size;
    emu.eip += 2;
}

fn get_data_u8(emu: &mut Emulator) -> u8 {
    let data = emu.memory[emu.eip as usize];
    return data
}

fn get_data_i8(emu: &mut Emulator, i: usize) -> i8 {
    let data = emu.memory[emu.eip as usize + i];
    return data as i8;
}

fn get_data_16(emu: &mut Emulator) -> (u8, u8) {
    let data1 = emu.memory[emu.eip as usize];
    let data2 = emu.memory[(emu.eip + 1) as usize];
    return (data1, data2)
}

fn get_data32(emu: &mut Emulator, i: usize) -> i32 {
    let pointer = emu.eip as usize + i;

    let data1 = emu.memory[pointer as usize] as i32;
    let data2 = emu.memory[(pointer + 1) as usize] as i32;
    let data3 = emu.memory[(pointer + 2) as usize] as i32;
    let data4 = emu.memory[(pointer + 3) as usize] as i32;
    
    return data4 * (256 * 256 * 256) + data3 * (256 * 256) + data2 * 256 + data1
}

fn display(emu: &mut Emulator) {
    println!("EIP: {}", emu.eip);
    println!("ESP: {}", emu.esp);
    println!("EFLAGS: {}", emu.eflags);
    for i in 0..emu.registers.len() {
        println!("Register {}: 0x{:x}", i, emu.registers[i as usize]);
    }
    println!("Memory :");
    for i in 0..emu.memory.len() {
        print!("{:x} ", emu.memory[i as usize]);
    }
    println!("");
    
}
