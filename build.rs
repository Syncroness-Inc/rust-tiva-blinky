extern crate gcc;
use std::env;

// Build the application for the target.
fn build_for_target () {
    
    // Set the target C compiler to the ARM cross compiler.
    env::set_var("TARGET_CC", "arm-none-eabi-gcc");
    env::set_var("TARGET_CFLAGS", "-O0 -mthumb -mcpu=cortex-m4 -mfpu=fpv4-sp-d16 -mfloat-abi=softfp --specs=nano.specs");
    
    // Compile some C libraries so we can use some C code.
    // Note that we new to use the "advanced" Config syntax so that we set the "pic" option to
    // false. The prevents compilation with from using -fPIC option for position independent code
    // (PIC). We do not want PIC, that's for dynamic libraries. Without this option we end up with
    // .got and .got.ld sections in the binary, which are the sign that something is wrong.
    // See https://github.com/alexcrichton/gcc-rs/pull/67 for details.
    
    // Compile my syscalls for the hardware, so we can use newlib as the C standard library.
    gcc::Config::new()
        .file("src/syscalls.c")
        .pic(false)
        .compile("libsyscalls.a");
    
    gcc::Config::new()
        .file("src/startup.c")
        .pic(false)
        .compile("libstartup.a");
    
    // Link to the pre-compiled TivaWare library.
    println!("cargo:rustc-link-search=native=lib/TivaWare/driverlib/gcc");
    println!("cargo:rustc-link-lib=static=driver");
    
    // Link to the C standard library.
    println!("cargo:rustc-link-lib=static=c");
}

// Build for running the tests on the host.
fn build_tests_for_host() {
    println!("*********************************************");
}

fn main() {
    if let Ok(target) = env::var("TARGET") {
        if let Ok(host) = env::var("HOST"){
            if target == host {
                build_tests_for_host();
                return;
            }
        }
    }

    build_for_target(); 
}