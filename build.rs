fn main() {
    if cfg!(all(target_os = "windows", target_env = "gnu")) {
        // libsixel ends up needing these MinGW archives again at the very end
        // of the final link on Windows GNU, or the nanosleep/stat shims stay
        // unresolved in release builds.
        println!("cargo:rustc-link-arg=-lwinpthread");
        println!("cargo:rustc-link-arg=-lmsvcrt");
    }
}
