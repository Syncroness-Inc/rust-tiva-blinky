// Finally, we need to define some "lang items" we are _not_ going to use, but that `rustc` demands
// anyway. As we are not going to use the functionality they provide (panic/unwinding) we can left
// their definitions empty.

// Only define these for a bare metal target. This means we can run tests on the host.
#[cfg(target_os = "none")]
#[lang = "panic_fmt"]
extern fn panic_fmt() {}

#[cfg(target_os = "none")]
#[lang = "eh_personality"]
fn eh_personality() {}
