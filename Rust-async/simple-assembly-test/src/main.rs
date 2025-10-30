use std::arch::asm;

fn main() {

    let t1: usize = 500000;

    let t1_ptr: *const usize = &t1; 

    let t2: usize = check_assembly_reg(t1_ptr);

    println!("After assembly reg: {}", t2);

    println!("Hello, world!");
}

fn check_assembly_reg(t1_ptr: *const usize) -> usize {
    let mut res: usize;
    //[] --> this will make the cpu handle the data in the register as addr
    unsafe {
        asm!("mov {0}, [{1}]", out(reg) res, in(reg) t1_ptr)
    };

    res
}