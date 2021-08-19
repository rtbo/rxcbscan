use crate::output::Output;
use crate::parse::{self, EnumItem, Event};
use std::collections::HashSet;
use std::io::{self, Cursor, Write};

#[derive(Debug)]
pub struct CodeGen {
    xcb_mod: String,
    xcb_mod_prefix: String,
    typ_reg: HashSet<String>, // types registered
    ffi: Output,
    rs: Output,
    ffi_ext_fn: Cursor<Vec<u8>>,
}

impl CodeGen {
    pub fn new(xcb_mod: &str, ffi: Output, rs: Output) -> CodeGen {
        let mp = if xcb_mod == "xproto" {
            String::new()
        } else {
            format!("{}_", &xcb_mod)
        };

        CodeGen {
            xcb_mod: xcb_mod.to_owned(),
            xcb_mod_prefix: mp,
            typ_reg: HashSet::new(),
            ffi,
            rs,
            ffi_ext_fn: Cursor::new(Vec::new()),
        }
    }

    // pub fn xcb_mod(&self) -> &str {
    //     &self.xcb_mod
    // }

    pub fn reg_type(&mut self, typ: String) {
        self.typ_reg.insert(typ);
    }

    // pub fn xcb_mod_prefix(&self) -> &str {
    //     &self.xcb_mod_prefix
    // }

    pub fn ffi_type_name(&mut self, typ: &str) -> String {
        let typ = tit_split(typ).to_ascii_lowercase();
        format!("xcb_{}{}_t", self.xcb_mod_prefix, typ)
    }

    pub fn ffi_enum_type_name(&mut self, typ: &str) -> String {
        let typ = tit_split(typ).to_ascii_lowercase();
        let try1 = format!("xcb_{}{}_t", self.xcb_mod_prefix, typ);
        if self.typ_reg.contains(&try1) {
            format!("xcb_{}{}_enum_t", self.xcb_mod_prefix, typ)
        } else {
            try1
        }
    }

    pub fn ffi_enum_item_name(&self, name: &str, item: &str) -> String {
        format!(
            "XCB_{}{}_{}",
            &self.xcb_mod_prefix,
            tit_split(name),
            tit_split(item)
        )
        .to_ascii_uppercase()
    }

    pub fn ffi_iterator_name(&self, typ: &str) -> String {
        format!(
            "xcb_{}{}_iterator_t",
            self.xcb_mod_prefix,
            tit_split(typ).to_ascii_lowercase()
        )
    }

    pub fn ffi_iterator_next_fn_name(&self, typ: &str) -> String {
        format!(
            "xcb_{}{}_next",
            self.xcb_mod_prefix,
            tit_split(typ).to_ascii_lowercase()
        )
    }

    pub fn ffi_iterator_end_fn_name(&self, typ: &str) -> String {
        format!(
            "xcb_{}{}_end",
            self.xcb_mod_prefix,
            tit_split(typ).to_ascii_lowercase()
        )
    }

    pub fn rust_type_name(&self, typ: &str) -> String {
        capitalize(typ)
    }

    pub fn prologue(&mut self, imports: &Vec<String>) -> io::Result<()> {
        let out = &mut self.ffi;
        writeln!(out, "use libc::{{c_char, c_int, c_uint, c_void}};")?;
        writeln!(out, "use std;")?;
        writeln!(out)?;
        writeln!(out, "use ffi::base::*;")?;
        for imp in imports.iter() {
            writeln!(out, "use ffi::{}::*;", imp)?;
        }
        writeln!(out)?;

        let out = &mut self.rs;
        writeln!(out, "use libc::{{self, c_char, c_int, c_uint, c_void}};")?;
        writeln!(out, "use std;")?;
        writeln!(out, "use std::iter::Iterator;")?;
        writeln!(out, "")?;

        writeln!(out, "use base;")?;
        for imp in imports.iter() {
            writeln!(out, "use {};", imp)?;
        }
        writeln!(out, "use ffi::base::*;")?;
        writeln!(out, "use ffi::{}::*;", self.xcb_mod)?;
        for imp in imports.iter() {
            writeln!(out, "use ffi::{}::*;", imp)?;
        }
        writeln!(out, "")?;

        Ok(())
    }

    pub fn epilogue(&mut self) -> io::Result<()> {
        let out = &mut self.ffi;
        // write out all the external functions
        writeln!(out).unwrap();
        writeln!(out, "extern {{")?;

        out.write(self.ffi_ext_fn.get_ref())?;

        writeln!(out).unwrap();
        writeln!(out, "}} // extern")?;
        Ok(())
    }

    pub fn event(&mut self, ev: &Event) -> parse::Result<()> {
        match ev {
            Event::XidType(name) => {
                let typ = self.ffi_type_name(name);
                emit_type_alias(&mut self.ffi, &typ, "u32")?;

                self.emit_ffi_iterator(name)?;
                self.reg_type(typ);

                let typ = self.rust_type_name(name);
                let ffi_typ = self.ffi_type_name(name);
                emit_type_alias(&mut self.rs, &typ, &ffi_typ)?;
                self.reg_type(typ);
            }
            Event::Enum { name, items, .. } => {
                let typ = self.ffi_enum_type_name(name);

                self.emit_ffi_enum(&typ, &name, &items)?;

                self.reg_type(typ)
            }
            _ => {}
        }
        Ok(())
    }

    fn emit_ffi_iterator(&mut self, typ: &str) -> io::Result<()> {
        let typ = typ.to_ascii_lowercase();
        let it_typ = self.ffi_iterator_name(&typ);
        let it_next = self.ffi_iterator_next_fn_name(&typ);
        let it_end = self.ffi_iterator_end_fn_name(&typ);

        let out = &mut self.ffi;

        writeln!(out)?;
        writeln!(out, "#[repr(C)]")?;
        writeln!(out, "#[derive(Debug)]")?;
        writeln!(out, "pub struct {} {{", &it_typ)?;
        writeln!(out, "    pub data:  *mut xcb_{}_t,", &typ)?;
        writeln!(out, "    pub rem:   c_int,")?;
        writeln!(out, "    pub index: c_int,")?;
        writeln!(out, "}}")?;

        let out = &mut self.ffi_ext_fn;
        writeln!(out).unwrap();
        writeln!(out, "pub fn {}(i: *mut {});", &it_next, &it_typ).unwrap();
        writeln!(
            out,
            "pub fn {}(i: *mut {}) -> xcb_generic_iterator_t;",
            &it_end, &it_typ
        )
        .unwrap();
        Ok(())
    }

    fn emit_ffi_enum(&mut self, typ: &str, name: &str, items: &[EnumItem]) -> io::Result<()> {
        writeln!(&mut self.ffi)?;
        writeln!(&mut self.ffi, "pub type {} = u32;", typ)?;
        for item in items {
            let val = format!("0x{:02x}", item.value);
            let name = self.ffi_enum_item_name(name, &item.name);
            writeln!(&mut self.ffi, "pub const {}: {} = {};", name, typ, val)?;
        }
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

/// insert a underscore before each uppercase/digit preceded or follwed by lowercase
/// do not apply to the first char
/// assert!(tit_split("SomeString") == "Some_String")
/// assert!(tit_split("WINDOW") == "WINDOW")
fn tit_split(name: &str) -> String {
    if name.len() <= 1 {
        return name.into();
    }

    let is_high = |c: char| c.is_ascii_uppercase() || c.is_ascii_digit();
    let is_low = |c: char| c.is_ascii_lowercase();

    let mut res = String::new();
    let mut ch = name.chars();
    let mut prev = ch.next().unwrap();
    res.push(prev);
    let mut c = ch.next().unwrap();

    for next in ch {
        if is_low(prev) && is_high(c) || is_high(c) && is_low(next) {
            res.push('_');
        }
        res.push(c);
        prev = c;
        c = next;
    }
    if is_low(prev) && is_high(c) {
        res.push('_');
    }
    res.push(c);

    res
}

fn emit_type_alias(out: &mut Output, new_typ: &str, old_typ: &str) -> io::Result<()> {
    writeln!(out)?;
    writeln!(out, "pub type {} = {};", new_typ, old_typ)?;
    Ok(())
}
