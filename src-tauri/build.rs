fn main() {
    tauri_build::build();

    // Nur crunch_wrapper.cpp + unitycrunch.cpp – kein bcn.cpp, kein stdint-Problem
    cc::Build::new()
        .cpp(true)
        .file("cpp/crunch_wrapper.cpp")
        .file("cpp/unitycrunch.cpp")
        .include("cpp")
        .flag_if_supported("/MT")  // VC++ Runtime statisch linken
        .flag_if_supported("/std:c++17")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-w")
        .compile("crunch_native");

    println!("cargo:rerun-if-changed=cpp/crunch_wrapper.cpp");
    println!("cargo:rerun-if-changed=cpp/unitycrunch.cpp");
}
