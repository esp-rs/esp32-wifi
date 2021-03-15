fn main() {
    std::env::set_var("CC_xtensa_esp32_none_elf", "xtensa-esp32-elf-cc");
    println!("cargo:rerun-if-changed=src/compatibility/printf.c");
    cc::Build::new()
        .file("src/compatibility/printf.c")
        .compile("libprintf.a");

    // Add linker search path
    println!("cargo:rustc-link-search=esp32-wifi-lib/esp32");

    //println!("cargo:rustc-link-lib=espnow");
    //    println!("cargo:rustc-link-lib=mesh");
    println!("cargo:rustc-link-lib=net80211");
    println!("cargo:rustc-link-lib=pp");
    println!("cargo:rustc-link-lib=rtc");
    //    println!("cargo:rustc-link-lib=smartconfig");
    println!("cargo:rustc-link-lib=core");
    println!("cargo:rustc-link-lib=phy");
    println!("cargo:rustc-link-lib=coexist");
}
