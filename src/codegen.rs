use crate::output::Output;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct CodeGen {
    xcb_mod: String,
    xcb_mod_prefix: String,
}

impl CodeGen {
    pub fn new(xcb_mod: &str) -> CodeGen {
        let mp = if xcb_mod == "xproto" {
            String::new()
        } else {
            format!("{}_", &xcb_mod)
        };

        CodeGen {
            xcb_mod: xcb_mod.to_owned(),
            xcb_mod_prefix: mp,
        }
    }

    pub fn xcb_mod(&self) -> &str {
        &self.xcb_mod
    }

    // pub fn xcb_mod_prefix(&self) -> &str {
    //     &self.xcb_mod_prefix
    // }

    pub fn ffi_type_name(&self, typ: &str) -> String {
        format!(
            "xcb_{}{}_t",
            self.xcb_mod_prefix,
            typ.to_ascii_lowercase()
        )
    }

    pub fn ffi_iterator_name(&self, typ: &str) -> String {
        format!(
            "xcb_{}{}_iterator_t",
            self.xcb_mod_prefix,
            typ.to_ascii_lowercase()
        )
    }

    pub fn ffi_iterator_next_fn_name(&self, typ: &str) -> String {
        format!(
            "xcb_{}{}_next",
            self.xcb_mod_prefix,
            typ.to_ascii_lowercase()
        )
    }

    pub fn ffi_iterator_end_fn_name(&self, typ: &str) -> String {
        format!(
            "xcb_{}{}_end",
            self.xcb_mod_prefix,
            typ.to_ascii_lowercase()
        )
    }

    pub fn rust_type_name(&self, typ: &str) -> String {
        capitalize(typ)
    }

    pub fn emit_type_alias(&self, out: &mut Output, new_typ: &str, old_typ: &str)-> io::Result<()> {
        writeln!(out)?;
        writeln!(out, "pub type {} = {};", new_typ, old_typ)?;
        Ok(())
    }

}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
    }
}

// fn upper_first(s: &str) -> String {
//     let mut c = s.chars();
//     match c.next() {
//         None => String::new(),
//         Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
//     }
// }
