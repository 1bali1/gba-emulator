#![allow(non_snake_case)]
use crate::cpu::CPU;

#[path="cpu/cpu.rs"]
mod cpu;
mod bus;

fn main() {
    let cpu = CPU::new();
}