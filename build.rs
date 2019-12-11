#[cfg(unix)]
fn is_defined(m: &'static str) -> bool {
    let file = format!("{}/test_{}.c", std::env::var("OUT_DIR").unwrap(), m);
    std::fs::write(
        &file,
        format!(
            r#"
            #include <time.h>

            #ifdef {}
            MACRO_IS_DEFINED
            #endif
            "#,
            m
        ),
    )
    .unwrap();
    let check = cc::Build::new().file(&file).expand();
    let check = String::from_utf8(check).unwrap();
    check.contains("MACRO_IS_DEFINED")
}

#[cfg(not(unix))]
fn is_defined(_m: &'static str) -> bool {
    false
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn gen_darwin_binding() {
    use std::path::PathBuf;

    println!("cargo:rerun-if-changed=src/clock/darwin_wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("src/clock/darwin_wrapper.h")
        .whitelist_function("mach_absolute_time")
        .whitelist_function("mach_timebase_info")
        .whitelist_function("pthread_mach_thread_np")
        .whitelist_function("pthread_self")
        .whitelist_function("thread_info")
        .whitelist_type("mach_timebase_info_data_t")
        .whitelist_type("thread_basic_info_data_t")
        .whitelist_var("THREAD_BASIC_INFO")
        .whitelist_var("__THREAD_BASIC_INFO_COUNT")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("darwin_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    let have_steady_clock = if cfg!(any(target_os = "macos", target_os = "ios", windows)) {
        true
    } else {
        is_defined("CLOCK_MONOTONIC")
    };
    if have_steady_clock {
        println!("cargo:rustc-cfg=have_steady_clock");
    }
    if cfg!(unix) && is_defined("CLOCK_THREAD_CPUTIME_ID") {
        println!("cargo:rustc-cfg=have_clock_thread_cputime_id");
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    gen_darwin_binding();
}
