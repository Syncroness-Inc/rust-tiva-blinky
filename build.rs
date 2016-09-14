use std::env;

fn main() {
    
    // Set the target C compiler to the ARM cross compiler.
    env::set_var("TARGET_CC", "arm-none-eabi-gcc");
    env::set_var("TARGET_CFLAGS", "-mthumb -mcpu=cortex-m4 -mfpu=fpv4-sp-d16 -mfloat-abi=softfp");
    
    // Link to the pre-compiled TivaWare library.
    println!("cargo:rustc-link-search=native=lib/TivaWare/driverlib/gcc");
    println!("cargo:rustc-link-lib=static=driver");
}