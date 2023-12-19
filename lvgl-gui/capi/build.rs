/*
 * Copyright (C) 2015-2023 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Redpesk interface code/config use MIT License and can be freely copy/modified even within proprietary code
 * License: $RP_BEGIN_LICENSE$ SPDX:MIT https://opensource.org/licenses/MIT $RP_END_LICENSE$
 *
*/
extern crate bindgen;
use std::env;

fn main() {
    // invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=capi/capi-map.c");
    println!("cargo:rustc-link-search=/usr/local/lib64");
    println!("cargo:rustc-link-arg=-llvgl");
    println!("cargo:rustc-link-arg=-llv_drivers");

    if let Ok(value) = env::var("CARGO_TARGET_DIR") {
        if let Ok(profile) = env::var("PROFILE") {
            println!("cargo:rustc-link-search=crate={}{}", value, profile);
        }
    }

    let header = "
    // -----------------------------------------------------------------------
    //         <- private '_capi_map.rs' Rust/C unsafe binding ->
    // -----------------------------------------------------------------------
    //   Do not exit this file it will be regenerated automatically by cargo.
    //   Check:
    //     - build.rs for C/Rust glue options
    //     - src/capi/capi-map.c for C prototype inputs
    // -----------------------------------------------------------------------
    "
    .to_string();
    let prj_dir = format!(
        "\npub const PRJ_DIR:&str=\"{}\";",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    let header = header + prj_dir.as_str();

    let gtk_selected = match env::var("USE_GTK") {
        Ok(_value) => {
            println!("cargo:warning=GTK driver backend selected");
            println!("cargo:rustc-cfg=use_gtk");
            1
        }
        Err(_) => 0,
    };

    let _capi_map = bindgen::Builder::default()
        .header("capi/capi-map.c")
        .clang_arg("-I/usr/local/include/lvgl")
        .clang_arg("-I/usr/include/lvgl")
        .raw_line(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_debug(false)
        .layout_tests(false)
        .allowlist_item("img_.*")
        .blocklist_type("lv_img_dsc_t") // defined in lvgl-rclib
        .generate()
        .expect("Unable to generate _capi-map.rs");

    _capi_map
        .write_to_file("capi/_capi-map.rs")
        .expect("Couldn't write _capi-map.rs!");

    let defined = gtk_selected.to_string();
    cc::Build::new()
        .file("capi/capi-map.c")
        .define("USE_GTK", defined.as_str())
        .include("/usr/local/include/lvgl")
        .include("/usr/local/include")
        .include("/usr/include/lvgl")
        .include("/usr/include")
        .compile("lvgl-asset");
}
