#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::println;

#[nexus_rt::main]
#[nexus_rt::public_input(x)]
fn main(x: u32, y: u32) -> u32 {
    println!("🔍 Processing inputs:");
    println!("  📖 Public input (x):  {}", x);
    println!("  🔒 Private input (y): {}", y);
    println!("  ⚡ Computing: {} × {} = {}", x, y, x * y);

    x * y
}