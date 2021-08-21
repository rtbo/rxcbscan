use std::collections::HashSet;
use std::io::{self, Cursor, Write};

use crate::ast::{Doc, EnumItem, Event, StructField, Struct};
use crate::output::Output;

#[derive(Debug)]
pub struct CodeGen {
    xcb_mod: String,
    xcb_mod_prefix: String,
    ffi: Output,
    rs: Output,
    ffi_typ_reg: HashSet<String>, // types registered in the FFI module
    rs_typ_reg: HashSet<String>,  // types registered in the Rust module

    // additional output buffers that make a second group of declaration/functions
    // they are appended to the output at the end
    ffi_buf: Cursor<Vec<u8>>,
    rs_buf: Cursor<Vec<u8>>,
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
            ffi_typ_reg: HashSet::new(),
            rs_typ_reg: HashSet::new(),
            ffi,
            rs,
            ffi_buf: Cursor::new(Vec::new()),
            rs_buf: Cursor::new(Vec::new()),
        }
    }

    // pub fn xcb_mod(&self) -> &str {
    //     &self.xcb_mod
    // }

    fn reg_ffi_type(&mut self, typ: String) {
        self.ffi_typ_reg.insert(typ);
    }

    fn reg_rs_type(&mut self, typ: String) {
        self.rs_typ_reg.insert(typ);
    }

    // pub fn xcb_mod_prefix(&self) -> &str {
    //     &self.xcb_mod_prefix
    // }

    fn ffi_enum_type_name(&mut self, typ: &str) -> String {
        let typ = tit_split(typ).to_ascii_lowercase();
        let try1 = format!("xcb_{}{}_t", self.xcb_mod_prefix, typ);
        if self.ffi_typ_reg.contains(&try1) {
            format!("xcb_{}{}_enum_t", self.xcb_mod_prefix, typ)
        } else {
            try1
        }
    }

    fn rs_enum_type_name(&mut self, typ: &str) -> String {
        let try1 = rust_type_name(&typ);
        if self.rs_typ_reg.contains(&try1) {
            format!("{}Enum", &try1)
        } else {
            try1
        }
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

        out.write_all(self.ffi_buf.get_ref())?;

        writeln!(out).unwrap();
        writeln!(out, "}} // extern")?;

        let out = &mut self.rs;
        out.write_all(self.rs_buf.get_ref())?;
        Ok(())
    }

    pub fn event(&mut self, ev: &Event) -> io::Result<()> {
        match ev {
            Event::Typedef { oldname, newname } => {
                let ffi_old_typ = ffi_type_name(&self.xcb_mod_prefix, oldname);
                let ffi_new_typ = ffi_type_name(&self.xcb_mod_prefix, newname);

                emit_type_alias(&mut self.ffi, &ffi_new_typ, &ffi_old_typ)?;
                self.emit_ffi_iterator(newname, &ffi_new_typ)?;

                let rs_new_typ = rust_type_name(newname);
                emit_type_alias(&mut self.rs, &rs_new_typ, &ffi_new_typ)?;

                self.reg_ffi_type(ffi_new_typ);
                self.reg_rs_type(rs_new_typ);
            }
            Event::XidType(name) => {
                let ffi_typ = ffi_type_name(&self.xcb_mod_prefix, name);
                emit_type_alias(&mut self.ffi, &ffi_typ, "u32")?;
                self.emit_ffi_iterator(name, &ffi_typ)?;

                let rs_typ = rust_type_name(name);
                emit_type_alias(&mut self.rs, &rs_typ, &ffi_typ)?;

                self.reg_ffi_type(ffi_typ);
                self.reg_rs_type(rs_typ);
            }
            Event::Enum (en) => {
                // make owned string to pass into the closure
                // otherwise borrow checker complains
                let xcb_mod_prefix = self.xcb_mod_prefix.to_string();

                let typ = self.ffi_enum_type_name(&en.name);
                emit_enum(
                    &mut self.ffi,
                    &typ,
                    en.items.iter().map(|it| EnumItem {
                        id: it.id.clone(),
                        name: ffi_enum_item_name(&xcb_mod_prefix, &en.name, &it.name),
                        value: it.value,
                    }),
                    &en.doc,
                )?;
                self.reg_ffi_type(typ);

                let typ = self.rs_enum_type_name(&en.name);
                emit_enum(
                    &mut self.rs,
                    &typ,
                    en.items.iter().map(|it| EnumItem {
                        id: it.id.clone(),
                        name: rust_enum_item_name(&en.name, &it.name),
                        value: it.value,
                    }),
                    &en.doc,
                )?;
                self.reg_rs_type(typ);
            }
            Event::Struct (stru) => {
                let ffi_typ = self.emit_ffi_struct(&stru)?;
                let ffi_it_typ = self.emit_ffi_iterator(&stru.name, &ffi_typ)?;

                let rs_typ = self.emit_rs_struct(&ffi_typ, &stru)?;
                self.emit_rs_iterator(&stru.name, &rs_typ, &ffi_it_typ)?;

                self.reg_ffi_type(ffi_typ);
                self.reg_rs_type(rs_typ);
            }
            _ => {}
        }
        Ok(())
    }

    fn emit_ffi_iterator(&mut self, name: &str, typ: &str) -> io::Result<String> {
        let it_typ = ffi_iterator_name(&self.xcb_mod_prefix, &name);
        let it_next = ffi_iterator_next_fn_name(&self.xcb_mod_prefix, &name);
        let it_end = ffi_iterator_end_fn_name(&self.xcb_mod_prefix, &name);

        let out = &mut self.ffi;

        writeln!(out)?;
        writeln!(out, "#[repr(C)]")?;
        writeln!(out, "#[derive(Debug)]")?;
        writeln!(out, "pub struct {} {{", &it_typ)?;
        writeln!(out, "    pub data:  *mut {},", &typ)?;
        writeln!(out, "    pub rem:   c_int,")?;
        writeln!(out, "    pub index: c_int,")?;
        writeln!(out, "}}")?;

        let out = &mut self.ffi_buf;
        writeln!(out).unwrap();
        writeln!(out, "pub fn {}(i: *mut {});", &it_next, &it_typ).unwrap();
        writeln!(
            out,
            "pub fn {}(i: *mut {}) -> xcb_generic_iterator_t;",
            &it_end, &it_typ
        )
        .unwrap();
        Ok(it_typ)
    }

    fn emit_rs_iterator(&mut self, name: &str, typ: &str, ffi_it_typ: &str) -> io::Result<()> {
        let it_typ = format!("{}Iterator", &typ);
        let ffi_it_next = ffi_iterator_next_fn_name(&self.xcb_mod_prefix, &name);

        let out = &mut self.rs_buf;

        writeln!(out)?;
        writeln!(out, "pub type {} = {};", &it_typ, &ffi_it_typ)?;
        writeln!(out)?;
        writeln!(out, "impl Iterator for {} {{", &it_typ)?;
        writeln!(out, "    type Item = {};", &typ)?;
        writeln!(out, "    fn next(&mut self) -> std::option::Option<{}> {{", &typ)?;
        writeln!(out, "        if self.rem == 0 {{")?;
        writeln!(out, "            None")?;
        writeln!(out, "        }} else {{")?;
        writeln!(out, "            unsafe {{")?;
        writeln!(out, "                let iter = self as *mut {};", &ffi_it_typ)?;
        writeln!(out, "                let data = (*iter).data;")?;
        writeln!(out, "                {}(iter);", &ffi_it_next)?;
        writeln!(out, "                Some(std::mem::transmute(*data))")?;
        writeln!(out, "            }}")?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }

    fn emit_ffi_struct(
        &mut self,
        stru: &Struct,
    ) -> io::Result<String> {
        let Struct {name, fields, doc} = &stru;

        let typ = ffi_type_name(&self.xcb_mod_prefix, &name);

        let out = &mut self.ffi;
        writeln!(out)?;
        emit_doc_text(out, &doc)?;
        writeln!(out, "#[repr(C)]")?;
        writeln!(out, "pub struct {} {{", &typ)?;

        let mut padnum = 0;

        for f in fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    emit_doc_field(out, &doc, &name)?;
                    writeln!(
                        out,
                        "    pub {}: {},",
                        symbol(&name),
                        ffi_field_type_name(&self.xcb_mod_prefix, &typ)
                    )?;
                }
                StructField::Pad(sz) => {
                    let padtyp = match sz {
                        1 => "u8".into(),
                        x => format!("[u8; {}]", x),
                    };
                    writeln!(out, "    pub pad{}: {},", padnum, padtyp)?;
                    padnum += 1;
                }
                _ => {}
            }
        }
        writeln!(out, "}}")?;

        Ok(typ)
    }

    fn emit_rs_struct(
        &mut self,
        ffi_typ: &str,
        stru: &Struct,
    ) -> io::Result<String> {
        let Struct {name, fields, doc} = &stru;

        let typ = rust_type_name(&name);

        let out = &mut self.rs_buf;
        // emitting struct
        writeln!(out)?;
        emit_doc_text(out, &doc)?;
        writeln!(out, "#[derive(Copy, Clone)]")?;
        writeln!(out, "pub struct {} {{", &typ)?;
        writeln!(out, "    pub base: {},", &ffi_typ)?;
        writeln!(out, "}}")?;

        // emitting struct impl
        writeln!(out)?;
        writeln!(out, "impl {} {{", &typ)?;
        writeln!(out, "    #[allow(unused_unsafe)]")?;

        // emitting ctor
        write!(out, "    pub fn new(")?;
        for f in fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    write!(out, "{}: {},", symbol(&name), rust_field_type_name(&typ))?;
                }
                _ => {}
            }
        }
        writeln!(out, ") -> {} {{", &typ)?;
        writeln!(out, "        unsafe {{")?;
        writeln!(out, "            {} {{", &typ)?;
        writeln!(out, "                base: {} {{", &ffi_typ)?;
        let mut padnum = 0;
        for f in fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    let name = symbol(name);
                    if typ == "BOOL" {
                        writeln!(out, "                    {}: {} != 0,", &name, &name)?;
                    } else {
                        writeln!(out, "                    {}: {},", &name, &name)?;
                    }
                }
                StructField::Pad(sz) => {
                    let val = if *sz == 1 {
                        "0".into()
                    } else {
                        format!("[0; {}]", sz)
                    };
                    writeln!(out, "                pad{}: {},", padnum, val)?;
                    padnum += 1;
                }
                _ => {}
            }
        }
        writeln!(out, "                }}")?;
        writeln!(out, "            }}")?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;

        // emitting accessors
        for f in fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    emit_doc_field(out, &doc, &name)?;
                    let name = symbol(name);
                    writeln!(
                        out,
                        "    pub fn {}(&self) -> {} {{",
                        &name,
                        rust_field_type_name(&typ)
                    )?;
                    if typ == "BOOL" {
                        writeln!(out, "        self.base.{} != 0", &name)?;
                    } else {
                        writeln!(out, "        unsafe {{ self.base.{} }}", &name)?;
                    }
                    writeln!(out, "    }}")?;
                }
                _ => {}
            }
        }

        writeln!(out, "}}")?;

        Ok(typ)
    }
}

// fn capitalize(s: &str) -> String {
//     let mut c = s.chars();
//     match c.next() {
//         None => String::new(),
//         Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
//     }
// }

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

/// capitalize each substring beginning by uppercase
/// said otherwise: every upper preceded by another upper is turned to lower
/// assert!(tit_cap("SomeString") == "SomeString")
/// assert!(tit_cap("WINDOW") == "Window")
fn tit_cap(name: &str) -> String {
    if name.len() <= 1 {
        return name.into();
    }

    let is_high = |c: char| c.is_ascii_uppercase() | c.is_ascii_digit();

    let mut res = String::new();
    let mut ch = name.chars();
    let mut prev = ch.next().unwrap();
    res.push(prev);

    for c in ch {
        if is_high(c) && is_high(prev) {
            res.push(c.to_ascii_lowercase())
        } else {
            res.push(c)
        }
        prev = c;
    }

    res
}

fn symbol(name: &str) -> &str {
    match name {
        "type" => "type_",
        "str" => "str_",
        "match" => "match_",
        "new" => "new_",
        s => s,
    }
}

fn extract_module(typ: &str) -> (Option<&str>, &str) {
    let len = typ.len();
    let colon = typ.as_bytes().iter().position(|b| *b == b':');
    if let Some(colon) = colon {
        (Some(&typ[0..colon]), &typ[colon + 1..len])
    } else {
        (None, typ)
    }
}

fn qualified_name(module: &Option<&str>, name: &str) -> String {
    if let Some(module) = module {
        format!("{}::{}", module, &name)
    } else {
        name.into()
    }
}

fn ffi_type_name(xcb_mod_prefix: &str, typ: &str) -> String {
    match typ {
        "CARD8" => "u8".into(),
        "CARD16" => "u16".into(),
        "CARD32" => "u32".into(),
        "INT8" => "i8".into(),
        "INT16" => "i16".into(),
        "INT32" => "i32".into(),
        "BYTE" => "u8".into(),
        "BOOL" => "u8".into(),
        typ => {
            let typ = tit_split(typ).to_ascii_lowercase();
            format!("xcb_{}{}_t", xcb_mod_prefix, typ)
        }
    }
}

/// same as ffi_type_name but can also have a namespace before (with a single colon)
fn ffi_field_type_name(xcb_mod_prefix: &str, typ: &str) -> String {
    let (module, typ) = extract_module(&typ);

    let typ = ffi_type_name(&xcb_mod_prefix, &typ);

    qualified_name(&module, &typ)
}

fn ffi_enum_item_name(xcb_mod_prefix: &str, name: &str, item: &str) -> String {
    format!(
        "XCB_{}{}_{}",
        xcb_mod_prefix,
        tit_split(name),
        tit_split(item)
    )
    .to_ascii_uppercase()
}

fn ffi_iterator_name(xcb_mod_prefix: &str, typ: &str) -> String {
    format!(
        "xcb_{}{}_iterator_t",
        xcb_mod_prefix,
        tit_split(typ).to_ascii_lowercase()
    )
}

fn ffi_iterator_next_fn_name(xcb_mod_prefix: &str, typ: &str) -> String {
    format!(
        "xcb_{}{}_next",
        xcb_mod_prefix,
        tit_split(typ).to_ascii_lowercase()
    )
}

fn ffi_iterator_end_fn_name(xcb_mod_prefix: &str, typ: &str) -> String {
    format!(
        "xcb_{}{}_end",
        xcb_mod_prefix,
        tit_split(typ).to_ascii_lowercase()
    )
}

fn rust_type_name(typ: &str) -> String {
    match typ {
        "CARD8" => "u8".into(),
        "CARD16" => "u16".into(),
        "CARD32" => "u32".into(),
        "INT8" => "i8".into(),
        "INT16" => "i16".into(),
        "INT32" => "i32".into(),
        "BYTE" => "u8".into(),
        "BOOL" => "bool".into(),
        typ => tit_cap(typ),
    }
}

/// same as rust_type_name but can also have a namespace before (with a single colon)
fn rust_field_type_name(typ: &str) -> String {
    let (module, typ) = extract_module(&typ);

    let typ = rust_type_name(&typ);

    qualified_name(&module, &typ)
}

fn rust_enum_item_name(name: &str, item: &str) -> String {
    format!("{}_{}", tit_split(name), tit_split(item)).to_ascii_uppercase()
}

fn emit_type_alias<Out: Write>(out: &mut Out, new_typ: &str, old_typ: &str) -> io::Result<()> {
    writeln!(out)?;
    writeln!(out, "pub type {} = {};", new_typ, old_typ)?;
    Ok(())
}

fn emit_doc_str<Out: Write>(out: &mut Out, docstr: &str) -> io::Result<()> {
    if !docstr.is_empty() {
        let strvec: Vec<String> = docstr.split('\n').map(|l| "/// ".to_owned() + l).collect();
        let mut docstr = strvec.join("\n");
        docstr.push('\n');
        out.write_all(docstr.as_bytes())
    } else {
        Ok(())
    }
}

fn emit_doc_text<Out: Write>(out: &mut Out, doc: &Option<Doc>) -> io::Result<()> {
    if let Some(doc) = doc {
        if !doc.brief.is_empty() {
            emit_doc_str(out, &doc.brief)?;
        }
        if !doc.text.is_empty() {
            emit_doc_str(out, &doc.text)?;
        }
    }
    Ok(())
}

fn emit_doc_field<Out: Write>(out: &mut Out, doc: &Option<Doc>, field: &str) -> io::Result<()> {
    if let Some(doc) = doc {
        if let Some(f) = doc.fields.iter().find(|f| f.name == field) {
            emit_doc_str(out, &f.text)?;
        }
    }
    Ok(())
}

fn emit_enum<Out, Items>(
    out: &mut Out,
    typ: &str,
    items: Items,
    doc: &Option<Doc>,
) -> io::Result<()>
where
    Out: Write,
    Items: Iterator<Item = EnumItem>,
{
    writeln!(out)?;
    emit_doc_text(out, doc)?;
    writeln!(out, "pub type {} = u32;", typ)?;
    for item in items {
        emit_doc_field(out, doc, &item.id)?;
        let val = format!("0x{:02x}", item.value);
        writeln!(out, "pub const {}: {} = {};", item.name, typ, val)?;
    }
    Ok(())
}
