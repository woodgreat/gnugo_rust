fn main() {
    // 链接原gnugo的C库（临时方案）
    println!("cargo:rustc-link-lib=static=gnugo");
    println!("cargo:rustc-link-search=native=../gnugo-c/engine");
}