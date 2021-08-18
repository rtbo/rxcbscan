use crate::output::Output;
use std::collections::HashSet;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct CodeGen {
    xcb_mod: String,
    xcb_mod_prefix: String,
    typ_reg: HashSet<String>, // types registered
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
            typ_reg: HashSet::new(),
        }
    }

    pub fn xcb_mod(&self) -> &str {
        &self.xcb_mod
    }

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

    pub fn emit_type_alias(
        &self,
        out: &mut Output,
        new_typ: &str,
        old_typ: &str,
    ) -> io::Result<()> {
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

// assert!(tit_split("SomeString") == "Some_String")
// assert!(tit_split("WINDOW") == "WINDOW")
fn tit_split(name: &str) -> String {
    if name.len() == 0 {
        return String::new();
    }
    let all_upper = !name.chars().any(|c| c.is_ascii_lowercase());
    if all_upper {
        return name.into();
    }
    let mut res = String::new();
    let mut ch = name.chars();
    res.push(ch.next().unwrap());
    for c in ch {
        if c.is_ascii_uppercase() || c.is_ascii_digit() {
            res.push('_');
        }
        res.push(c);
    }
    res
}
