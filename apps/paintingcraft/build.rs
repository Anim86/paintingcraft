//! Windows only: embed the app icon and version info (VERSIONINFO) into `photocraft.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PHOTOCRAFT_REQUIRE_WINRES=1` (set by the
//! release workflow) turns it into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/paintingcraft.ico");
    println!("cargo:rerun-if-env-changed=PAINTINGCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/paintingcraft.ico")
        .set("ProductName", "PaintingCraft")
        .set("FileDescription", "PaintingCraft digital painting studio")
        .set("CompanyName", "Storytold")
        .set("LegalCopyright", "Copyright (c) the PaintingCraft authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "paintingcraft.exe")
        .set("InternalName", "paintingcraft");
    if let Err(e) = res.compile() {
        if std::env::var_os("PAINTINGCRAFT_REQUIRE_WINRES").is_some() || std::env::var_os("PHOTOCRAFT_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=paintingcraft.exe built without icon/version resources: {e}");
    }
}
