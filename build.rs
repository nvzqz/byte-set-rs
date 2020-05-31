use std::env;

fn main() {
    let use64 = env::var_os("CARGO_CFG_TARGET_POINTER_WIDTH")
        .map(|width| width == "64")
        .unwrap_or(false);

    // TODO: Use 64-bit slot on 32-bit targets with 64-bit instructions.
    //
    // Consider (and benchmark!):
    // - arm with `neon` target feature
    // - x86 with `sse2` target feature
    if use64 {
        println!("cargo:rustc-cfg=byte_set_slot_64");
    }
}
