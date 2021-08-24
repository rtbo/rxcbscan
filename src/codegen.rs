use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, Cursor, Write};

use crate::ast::{Doc, EnumItem, Event, Struct, StructField};
use crate::output::Output;

#[derive(Debug)]
pub struct CodeGen {
    xcb_mod: String,
    xcb_mod_prefix: String,
    ffi: Output,
    rs: Output,
    typ_with_lifetime: HashSet<String>,

    // registered types
    ffi_type_sizes: HashMap<String, Option<usize>>,
    // types registered in the FFI module
    ffi_typ_reg: HashSet<String>,
    // types registered in the Rust module
    rs_typ_reg: HashSet<String>,

    // additional output buffers that make a second group of declaration/functions
    // they are appended to the output at the end
    ffi_buf: Cursor<Vec<u8>>,
    rs_buf: Cursor<Vec<u8>>,

    ptr_width: usize,
}

#[cfg(target_pointer_width = "64")]
const PTR_WIDTH: usize = 64;
#[cfg(target_pointer_width = "32")]
const PTR_WIDTH: usize = 32;

impl CodeGen {
    pub fn new(xcb_mod: &str, ffi: Output, rs: Output) -> CodeGen {
        let mp = if xcb_mod == "xproto" {
            String::new()
        } else {
            format!("{}_", &xcb_mod)
        };

        let ptr_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH")
            .map(|s| {
                str::parse::<usize>(&s).expect(&format!(
                    "can't parse CARGO_CFG_TARGET_POINTER_WIDTH {} as usize",
                    s
                ))
            })
            .unwrap_or(PTR_WIDTH);

        CodeGen {
            xcb_mod: xcb_mod.to_owned(),
            xcb_mod_prefix: mp,
            ffi,
            rs,
            typ_with_lifetime: HashSet::new(),
            ffi_type_sizes: HashMap::new(),
            ffi_typ_reg: HashSet::new(),
            rs_typ_reg: HashSet::new(),
            ffi_buf: Cursor::new(Vec::new()),
            rs_buf: Cursor::new(Vec::new()),
            ptr_width,
        }
    }

    // pub fn xcb_mod(&self) -> &str {
    //     &self.xcb_mod
    // }

    fn reg_type(&mut self, typ: String, ffi_typ: String, rs_typ: String, ffi_sz: Option<usize>) {
        self.ffi_typ_reg.insert(ffi_typ);
        self.rs_typ_reg.insert(rs_typ);
        self.ffi_type_sizes.insert(typ, ffi_sz);
    }

    fn has_ffi_type(&mut self, typ: &str) -> bool {
        self.ffi_typ_reg.contains(typ)
    }

    fn has_rs_type(&mut self, typ: &str) -> bool {
        self.rs_typ_reg.contains(typ)
    }

    fn ffi_type_sizeof(&self, typ: &str) -> usize {
        // TODO: emit codegen result if typ is not registered instead of panic

        match typ {
            "CARD8" => 1,
            "CARD16" => 2,
            "CARD32" => 4,
            "INT8" => 1,
            "INT16" => 2,
            "INT32" => 4,
            "BYTE" => 1,
            "BOOL" => 1,
            "char" => 1,
            typ => self
                .ffi_type_sizes
                .get(typ)
                .unwrap_or_else(|| panic!("no sizeof entry for {}", typ))
                .unwrap_or_else(|| panic!("type {} has variable size", typ)),
        }
    }

    fn eligible_to_copy(&self, stru: &Struct) -> bool {
        !has_list(&stru.fields)
    }

    fn type_has_lifetime(&self, stru: &Struct) -> bool {
        has_list(&stru.fields)
    }

    fn ffi_enum_type_name(&mut self, typ: &str) -> String {
        let typ = tit_split(typ).to_ascii_lowercase();
        let try1 = format!("xcb_{}{}_t", self.xcb_mod_prefix, typ);
        if self.has_ffi_type(&try1) {
            format!("xcb_{}{}_enum_t", self.xcb_mod_prefix, typ)
        } else {
            try1
        }
    }

    fn rs_enum_type_name(&mut self, typ: &str) -> String {
        let try1 = rust_type_name(&typ);
        if self.has_rs_type(&try1) {
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
                self.emit_ffi_iterator(newname, &ffi_new_typ, false)?;

                let rs_new_typ = rust_type_name(newname);
                emit_type_alias(&mut self.rs, &rs_new_typ, &ffi_new_typ)?;

                self.reg_type(newname.clone(), ffi_new_typ, rs_new_typ, Some(self.ffi_type_sizeof(&oldname)))
            }
            Event::XidType(name) => self.emit_xid(name)?,
            Event::XidUnion(xidun) => self.emit_xid(&xidun.name)?,
            Event::Enum(en) => {
                // make owned string to pass into the closure
                // otherwise borrow checker complains
                let xcb_mod_prefix = self.xcb_mod_prefix.to_string();

                let ffi_typ = self.ffi_enum_type_name(&en.name);
                emit_enum(
                    &mut self.ffi,
                    &ffi_typ,
                    en.items.iter().map(|it| EnumItem {
                        id: it.id.clone(),
                        name: ffi_enum_item_name(&xcb_mod_prefix, &en.name, &it.name),
                        value: it.value,
                    }),
                    &en.doc,
                )?;

                let rs_typ = self.rs_enum_type_name(&en.name);
                emit_enum(
                    &mut self.rs,
                    &rs_typ,
                    en.items.iter().map(|it| EnumItem {
                        id: it.id.clone(),
                        name: rust_enum_item_name(&en.name, &it.name),
                        value: it.value,
                    }),
                    &en.doc,
                )?;
                self.reg_type(en.name.clone(), ffi_typ, rs_typ, Some(4));
            }
            Event::Struct(stru) => self.emit_struct(&stru)?,
            _ => {}
        }
        Ok(())
    }

    fn emit_xid(&mut self, name: &str) -> io::Result<()> {
        let ffi_typ = ffi_type_name(&self.xcb_mod_prefix, name);
        emit_type_alias(&mut self.ffi, &ffi_typ, "u32")?;
        self.emit_ffi_iterator(name, &ffi_typ, false)?;

        let rs_typ = rust_type_name(name);
        emit_type_alias(&mut self.rs, &rs_typ, &ffi_typ)?;

        self.reg_type(name.into(), ffi_typ, rs_typ, Some(4));
        Ok(())
    }

    fn emit_struct(&mut self, stru: &Struct) -> io::Result<()> {
        let has_lifetime = self.type_has_lifetime(&stru);
        if has_lifetime {
            self.typ_with_lifetime.insert(stru.name.clone());
        }
        let ffi_typ = self.emit_ffi_struct(&stru)?;
        self.emit_ffi_field_list_accessors(&ffi_typ, &stru)?;
        let ffi_it_typ = self.emit_ffi_iterator(&stru.name, &ffi_typ, has_lifetime)?;

        let rs_typ = self.emit_rs_struct(&ffi_typ, &stru, has_lifetime)?;
        self.emit_rs_iterator(&stru.name, &rs_typ, &ffi_it_typ, has_lifetime)?;

        let mut stru_sz = Some(0usize);

        for f in stru.fields.iter() {
            match f {
                StructField::Field { typ, .. } => {
                    stru_sz = stru_sz.map(|sz| sz + self.ffi_type_sizeof(&typ));
                }
                StructField::Pad(pad_sz) => {
                    stru_sz = stru_sz.map(|sz| sz + pad_sz);
                }
                _ => {
                    stru_sz = None;
                    break;
                }
            }
        }

        self.reg_type(stru.name.clone(), ffi_typ, rs_typ, stru_sz);

        Ok(())
    }

    fn emit_ffi_iterator(
        &mut self,
        name: &str,
        typ: &str,
        has_lifetime: bool,
    ) -> io::Result<String> {
        let it_typ = ffi_iterator_name(&self.xcb_mod_prefix, &name);
        let it_next = ffi_iterator_next_fn_name(&self.xcb_mod_prefix, &name);
        let it_end = ffi_iterator_end_fn_name(&self.xcb_mod_prefix, &name);

        let out = &mut self.ffi;

        let lifetime = if has_lifetime { "<'a>" } else { "" };

        writeln!(out)?;
        writeln!(out, "#[repr(C)]")?;
        writeln!(out, "#[derive(Debug)]")?;
        writeln!(out, "pub struct {}{} {{", &it_typ, lifetime)?;
        writeln!(out, "    pub data:  *mut {},", &typ)?;
        writeln!(out, "    pub rem:   c_int,")?;
        writeln!(out, "    pub index: c_int,")?;
        if has_lifetime {
            writeln!(out, "    _phantom: std::marker::PhantomData<&'a {}>,", &typ)?;
        }
        writeln!(out, "}}")?;

        let out = &mut self.ffi_buf;
        writeln!(out).unwrap();
        writeln!(out, "pub fn {}(i: *mut {});", &it_next, &it_typ).unwrap();
        writeln!(out).unwrap();
        writeln!(
            out,
            "pub fn {}(i: *mut {}) -> xcb_generic_iterator_t;",
            &it_end, &it_typ
        )
        .unwrap();
        Ok(it_typ)
    }

    fn emit_ffi_field_list_accessors(&mut self, typ: &str, stru: &Struct) -> io::Result<()> {
        let stru_typ = typ;
        for f in &stru.fields {
            match f {
                StructField::List { name, typ, .. } => {
                    // 3 possible cases depending on list type to provide the same functions as C bindings:
                    //  1. Strings
                    //      we need accessor (returns *mut c_char)
                    //      we need the length function
                    //      we need the end function
                    //  2. type without lifetime (POD or scalar)
                    //      we need accessor
                    //      we need the length function
                    //      we need the iterator function
                    //  3. type with lifetime
                    //      we need the length function
                    //      we need the iterator function (with lifetime)
                    let is_string = typ == "char";
                    let has_lifetime = self.typ_with_lifetime.contains(typ);
                    let accessor_needed = is_string || !has_lifetime;
                    let length_needed = true;
                    let iterator_needed = !is_string;
                    let end_needed = is_string;

                    if accessor_needed {
                        let f_typ = ffi_type_name(&self.xcb_mod_prefix, typ);
                        let acc_fn = ffi_field_list_iterator_acc_fn_name(
                            &self.xcb_mod_prefix,
                            &stru.name,
                            &name,
                        );
                        let out = &mut self.ffi_buf;
                        writeln!(out)?;
                        writeln!(
                            out,
                            "pub fn {}(R: *const {}) -> *mut {};",
                            &acc_fn, &stru_typ, &f_typ
                        )?;
                    }

                    if length_needed {
                        let len_fn = ffi_field_list_iterator_len_fn_name(
                            &self.xcb_mod_prefix,
                            &stru.name,
                            &name,
                        );
                        let out = &mut self.ffi_buf;
                        writeln!(out)?;
                        writeln!(out, "pub fn {}(R: *const {}) -> c_int;", &len_fn, &stru_typ)?;
                    }

                    if end_needed {
                        let end_fn = ffi_field_list_iterator_end_fn_name(
                            &self.xcb_mod_prefix,
                            &stru.name,
                            &name,
                        );
                        let out = &mut self.ffi_buf;
                        writeln!(out)?;
                        writeln!(
                            out,
                            "pub fn {}(R: *const {}) -> xcb_generic_iterator_t;",
                            &end_fn, &stru_typ
                        )?;
                    }

                    if iterator_needed {
                        let lifetime = if has_lifetime { "<'a>" } else { "" };
                        let it_fn = ffi_field_list_iterator_it_fn_name(
                            &self.xcb_mod_prefix,
                            &stru.name,
                            &name,
                        );
                        let it_typ = ffi_iterator_name(&self.xcb_mod_prefix, &typ);
                        let out = &mut self.ffi_buf;
                        writeln!(out)?;
                        writeln!(
                            out,
                            "pub fn {}{}(R: *const {}) -> {}{};",
                            &it_fn, &lifetime, &stru_typ, &it_typ, &lifetime
                        )?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn emit_rs_iterator(
        &mut self,
        name: &str,
        typ: &str,
        ffi_it_typ: &str,
        has_lifetime: bool,
    ) -> io::Result<()> {
        let it_typ = format!("{}Iterator", &typ);
        let ffi_it_next = ffi_iterator_next_fn_name(&self.xcb_mod_prefix, &name);

        let (indir, lifetime) = if has_lifetime {
            ("", "<'a>")
        } else {
            ("*", "")
        };

        let out = &mut self.rs_buf;

        writeln!(out)?;
        writeln!(
            out,
            "pub type {}{} = {}{};",
            &it_typ, &lifetime, &ffi_it_typ, &lifetime
        )?;
        writeln!(out)?;
        writeln!(
            out,
            "impl{} Iterator for {}{} {{",
            &lifetime, &it_typ, &lifetime
        )?;
        writeln!(out, "    type Item = {}{};", &typ, &lifetime)?;
        writeln!(
            out,
            "    fn next(&mut self) -> std::option::Option<{}{}> {{",
            &typ, &lifetime,
        )?;
        writeln!(out, "        if self.rem == 0 {{")?;
        writeln!(out, "            None")?;
        writeln!(out, "        }} else {{")?;
        writeln!(out, "            unsafe {{")?;
        writeln!(
            out,
            "                let iter = self as *mut {};",
            &ffi_it_typ
        )?;
        writeln!(out, "                let data = (*iter).data;")?;
        writeln!(out, "                {}(iter);", &ffi_it_next)?;
        writeln!(
            out,
            "                Some(std::mem::transmute({}data))",
            &indir
        )?;
        writeln!(out, "            }}")?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }

    fn emit_ffi_struct(&mut self, stru: &Struct) -> io::Result<String> {
        let Struct { name, fields, doc } = &stru;

        let typ = ffi_type_name(&self.xcb_mod_prefix, &name);
        let impl_copy_clone = self.eligible_to_copy(&stru);

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

        if impl_copy_clone {
            writeln!(out)?;
            writeln!(out, "impl Copy for {} {{}}", &typ)?;
            writeln!(out, "impl Clone for {} {{", &typ)?;
            writeln!(out, "    fn clone(&self) -> {} {{", &typ)?;
            writeln!(out, "        *self")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
        }
        writeln!(out, "impl ::std::fmt::Debug for {} {{", &typ)?;
        writeln!(
            out,
            "    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {{"
        )?;
        writeln!(out, "        fmt.debug_struct(\"{}\")", &typ)?;
        let mut padnum = 0;
        for f in fields.iter() {
            match f {
                StructField::Field { name, .. } => {
                    writeln!(out, "            .field(\"{}\", &self.{})", &name, &name)?;
                }
                StructField::Pad(sz) => {
                    let (indir, idx) = if *sz == 1 { ("&", "") } else { ("&&", "[..] ") };
                    writeln!(
                        out,
                        "            .field(\"pad{}\", {}self.pad{}{})",
                        padnum, &indir, padnum, &idx
                    )?;
                    padnum += 1;
                }
                _ => {}
            }
        }
        writeln!(out, "            .finish()")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(typ)
    }

    fn emit_rs_struct(
        &mut self,
        ffi_typ: &str,
        stru: &Struct,
        has_lifetime: bool,
    ) -> io::Result<String> {
        let Struct { name, fields, doc } = &stru;

        let typ = rust_type_name(&name);

        let out = &mut self.rs_buf;
        // emitting struct
        writeln!(out)?;
        emit_doc_text(out, &doc)?;
        if has_lifetime {
            writeln!(
                out,
                "pub type {}<'a> = base::StructPtr<'a, {}>;",
                &typ, &ffi_typ
            )?;
        } else {
            writeln!(out, "#[derive(Copy, Clone)]")?;
            writeln!(out, "pub struct {} {{", &typ)?;
            writeln!(out, "    pub base: {},", &ffi_typ)?;
            writeln!(out, "}}")?;
        }

        let (accessor, lifetime) = if has_lifetime {
            ("(*self.ptr)", "<'a>")
        } else {
            ("self.base", "")
        };
        // emitting struct impl
        writeln!(out)?;
        writeln!(out, "impl{} {}{} {{", &lifetime, &typ, &lifetime)?;

        // emitting ctor
        if !has_lifetime {
            writeln!(out, "    #[allow(unused_unsafe)]")?;
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
        }

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
                        writeln!(out, "        unsafe {{ {}.{} != 0 }}", &accessor, &name)?;
                    } else {
                        writeln!(out, "        unsafe {{ {}.{} }}", &accessor, &name)?;
                    }
                    writeln!(out, "    }}")?;
                }
                StructField::List { name, typ, .. } => {
                    let is_string = typ == "char";

                    if is_string {
                        let len_fn = ffi_field_list_iterator_len_fn_name(
                            &self.xcb_mod_prefix,
                            &stru.name,
                            &name,
                        );
                        let acc_fn = ffi_field_list_iterator_acc_fn_name(
                            &self.xcb_mod_prefix,
                            &stru.name,
                            &name,
                        );
                        writeln!(out, "    pub fn {}(&self) -> &str {{", &name)?;
                        writeln!(out, "        unsafe {{")?;
                        writeln!(out, "            let field = self.ptr;")?;
                        writeln!(out, "            let len = {}(field) as usize;", &len_fn)?;
                        writeln!(out, "            let data = {}(field);", &acc_fn)?;
                        writeln!(out, "            let slice = std::slice::from_raw_parts(data as *const u8, len);")?;
                        writeln!(out, "            // should we check what comes from X?")?;
                        writeln!(out, "            std::str::from_utf8_unchecked(&slice)")?;
                        writeln!(out, "        }}")?;
                        writeln!(out, "    }}")?;
                    } else {
                        let has_lifetime = self.typ_with_lifetime.contains(typ);
                        let lifetime = if has_lifetime { "<'a>" } else { "" };
                        let it_typ = rust_iterator_type_name(&typ);
                        let ffi_it_typ = ffi_field_list_iterator_it_fn_name(
                            &self.xcb_mod_prefix,
                            &stru.name,
                            &name,
                        );
                        writeln!(
                            out,
                            "    pub fn {}(&self) -> {}{} {{",
                            &name, &it_typ, &lifetime
                        )?;
                        writeln!(out, "        unsafe {{ {}(self.ptr) }}", &ffi_it_typ)?;
                        writeln!(out, "    }}")?;
                    }
                }
                _ => {}
            }
        }
        writeln!(out, "}}")?;

        Ok(typ)
    }
}

fn has_list(fields: &[StructField]) -> bool {
    for f in fields.iter() {
        match f {
            StructField::List { .. } => return true,
            _ => {}
        }
    }
    false
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
        "char" => "c_char".into(),
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

fn ffi_field_list_iterator_acc_fn_name(
    xcb_mod_prefix: &str,
    typ_name: &str,
    field: &str,
) -> String {
    format!(
        "xcb_{}{}_{}",
        &xcb_mod_prefix,
        tit_split(typ_name).to_ascii_lowercase(),
        &field
    )
}

fn ffi_field_list_iterator_len_fn_name(
    xcb_mod_prefix: &str,
    typ_name: &str,
    field: &str,
) -> String {
    format!(
        "xcb_{}{}_{}_length",
        &xcb_mod_prefix,
        tit_split(typ_name).to_ascii_lowercase(),
        &field
    )
}

fn ffi_field_list_iterator_end_fn_name(
    xcb_mod_prefix: &str,
    typ_name: &str,
    field: &str,
) -> String {
    format!(
        "xcb_{}{}_{}_end",
        &xcb_mod_prefix,
        tit_split(typ_name).to_ascii_lowercase(),
        &field
    )
}

fn ffi_field_list_iterator_it_fn_name(xcb_mod_prefix: &str, typ_name: &str, field: &str) -> String {
    format!(
        "xcb_{}{}_{}_iterator",
        &xcb_mod_prefix,
        tit_split(typ_name).to_ascii_lowercase(),
        &field
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

fn rust_iterator_type_name(typ: &str) -> String {
    format!("{}Iterator", tit_cap(&typ))
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
