mod ffi;
mod naming;
mod output;
mod rust;
mod xcbgen;

use std::cmp;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use output::Output;
use xcbgen::XcbGen;

fn xcb_mod_map(name: &str) -> &str {
    match name {
        "bigreq" => "big_requests",
        "ge" => "genericevent",
        _ => name,
    }
}

fn is_always(name: &str) -> bool {
    match name {
        "xproto" | "big_requests" | "xc_misc" => true,
        _ => false,
    }
}

fn has_feature(name: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{}", name.to_ascii_uppercase())).is_ok()
}

fn main() {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string());
    let xml_dir = Path::new(&root).join("xml");
    // let src_dir = Path::new(&root).join("src");
    let out_dir = env::var("OUT_DIR").unwrap_or("./gen/current".to_string());
    let out_dir = Path::new(&out_dir);

    let ref_mtime = ["main.rs", "xcbgen.rs", "ffi.rs", "rust.rs", "naming.rs", "output.rs"]
        .iter()
        .map(|f| Path::new(&root).join("src").join(f))
        .map(|p| mtime(&p).expect(&format!("cannot get modification time of {}", p.display())))
        .fold(std::i64::MIN, |a, b| a.max(b));

    let rustfmt = env::var("XCB_RUSTFMT").ok().and_then(|var| {
        if var == "1" || var == "y" || var == "Y" {
            find_exe("rustfmt")
        } else {
            None
        }
    });

    for xml_file in iter_xml(&xml_dir) {
        let xcb_mod = xml_file.file_stem().unwrap();
        let xcb_mod = xcb_mod.to_str().unwrap();
        let xcb_mod = xcb_mod_map(xcb_mod);

        if !is_always(&xcb_mod) && !has_feature(&xcb_mod) {
            // continue;
        }

        let ref_mtime = cmp::max(ref_mtime, mtime(&xml_file).unwrap());
        let ffi_file = out_dir.join("ffi").join(&xcb_mod).with_extension("rs");
        let rs_file = out_dir.join(&xcb_mod).with_extension("rs");

        if ref_mtime > optional_mtime(&ffi_file, 0) || ref_mtime > optional_mtime(&rs_file, 0) {
            let ffi = Output::new(&rustfmt, &ffi_file).expect("cannot create FFI output");
            let rs = Output::new(&rustfmt, &rs_file).expect("cannot create Rust output");

            let gen = XcbGen::new(xcb_mod, ffi, rs);
            gen.xcb_gen(&xml_file).expect("could not generate XCB code");
        }
    }

    #[cfg(target_os = "freebsd")]
    println!("cargo:rustc-link-search=/usr/local/lib");
}

#[cfg(target_family = "unix")]
fn mtime<P: AsRef<Path>>(path: P) -> io::Result<i64> {
    use std::os::unix::fs::MetadataExt;
    fs::metadata(path).map(|m| m.mtime())
}

#[cfg(target_family = "windows")]
fn mtime<P: AsRef<Path>>(path: P) -> io::Result<i64> {
    use std::os::windows::fs::MetadataExt;
    fs::metadata(path).map(|m| m.last_write_time() as i64)
}

fn iter_xml(xml_dir: &Path) -> impl Iterator<Item = PathBuf> {
    fs::read_dir(xml_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| match p.extension() {
            Some(e) => e == "xml",
            _ => false,
        })
}

fn optional_mtime(path: &Path, default: i64) -> i64 {
    mtime(path).unwrap_or(default)
}

fn find_exe<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}
