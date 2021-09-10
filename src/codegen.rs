use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, Cursor, Write};

use crate::ast::{
    Doc, EnumItem, Event, Expr, ExtInfo, OpCopy, OpCopyMap, Reply, Request, Struct, StructField,
    SwitchCase,
};
use crate::output::Output;

#[derive(Clone, Debug)]
pub struct DepInfo {
    pub xcb_mod: String,
    pub xcb_mod_prefix: String,
    pub imports: Vec<String>,
    pub typ_with_lifetime: HashSet<String>,
    pub typ_unions: HashSet<String>,
    pub typ_simple: HashSet<String>,
    pub typ_pod: HashSet<String>,
    pub ffi_type_sizes: HashMap<String, Option<usize>>,
    pub ffi_typ_reg: HashSet<String>,
    pub rs_typ_reg: HashSet<String>,
}

impl DepInfo {
    fn has_type(&self, typ: &str) -> bool {
        self.ffi_type_sizes.contains_key(typ)
    }
}

#[derive(Debug)]
pub struct CodeGen {
    xcb_mod: String,
    xcb_mod_prefix: String,

    // output files
    ffi: Output,
    rs: Output,

    // additional output buffers that make a second group of declaration/functions
    // they are appended to the output at the end
    ffi_buf: Cursor<Vec<u8>>,
    rs_buf: Cursor<Vec<u8>>,

    typ_with_lifetime: HashSet<String>,
    typ_unions: HashSet<String>,
    typ_simple: HashSet<String>, // integer, chars, xids, enums
    typ_pod: HashSet<String>,    // simple and plain old data

    imports: Vec<String>,
    // registered types sizes (is None if size is not fixed - eg. lists with dynamic length)
    ffi_type_sizes: HashMap<String, Option<usize>>,
    // types registered in the FFI module
    ffi_typ_reg: HashSet<String>,
    // types registered in the Rust module
    rs_typ_reg: HashSet<String>,

    dep_info: Vec<DepInfo>,

    evcopies: OpCopyMap,

    ptr_width: usize,
}

#[cfg(target_pointer_width = "64")]
const PTR_WIDTH: usize = 64;
#[cfg(target_pointer_width = "32")]
const PTR_WIDTH: usize = 32;

impl CodeGen {
    pub fn new(
        xcb_mod: &str,
        ffi: Output,
        rs: Output,
        dep_info: Vec<DepInfo>,
        evcopies: OpCopyMap,
    ) -> CodeGen {
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
            ffi_buf: Cursor::new(Vec::new()),
            rs_buf: Cursor::new(Vec::new()),
            imports: Vec::new(),
            typ_with_lifetime: HashSet::new(),
            typ_unions: HashSet::new(),
            typ_simple: HashSet::new(),
            typ_pod: HashSet::new(),
            ffi_type_sizes: HashMap::new(),
            ffi_typ_reg: HashSet::new(),
            rs_typ_reg: HashSet::new(),
            dep_info,
            evcopies,
            ptr_width,
        }
    }

    pub fn into_depinfo(self) -> DepInfo {
        DepInfo {
            xcb_mod: self.xcb_mod,
            xcb_mod_prefix: self.xcb_mod_prefix,
            imports: self.imports,
            typ_with_lifetime: self.typ_with_lifetime,
            typ_unions: self.typ_unions,
            typ_simple: self.typ_simple,
            typ_pod: self.typ_pod,
            ffi_type_sizes: self.ffi_type_sizes,
            ffi_typ_reg: self.ffi_typ_reg,
            rs_typ_reg: self.rs_typ_reg,
        }
    }

    pub fn prologue(&mut self, imports: Vec<String>, ext_info: &Option<ExtInfo>) -> io::Result<()> {
        self.imports = imports;
        let mut imports = HashSet::<String>::new();
        for imp in self.imports.iter() {
            imports.insert(imp.clone());
        }
        for di in &self.dep_info {
            for imp in &di.imports {
                imports.insert(imp.clone());
            }
        }

        let out = &mut self.ffi;
        // Adding a comment only to fit the python generated code and pass initial tests
        writeln!(
            out,
            "// Generated automatically from {}.xml by rs_client.py version 0.9.0.",
            &self.xcb_mod
        )?;
        writeln!(out, "// Do not edit!")?;
        writeln!(out)?;
        writeln!(out, "use ffi::base::*;")?;
        for imp in imports.iter() {
            writeln!(out, "use ffi::{}::*;", imp)?;
        }
        writeln!(out, "use libc::{{c_char, c_int, c_uint, c_void}};")?;
        writeln!(out, "use std;")?;

        if let Some(ext_info) = ext_info {
            let maj_name = ffi_opcode_name(&self.xcb_mod_prefix, "MajorVersion");
            let min_name = ffi_opcode_name(&self.xcb_mod_prefix, "MinorVersion");

            writeln!(out)?;
            writeln!(
                out,
                "pub const {}: u32 = {};",
                &maj_name, ext_info.major_version
            )?;
            writeln!(
                out,
                "pub const {}: u32 = {};",
                &min_name, ext_info.minor_version
            )?;

            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(
                out,
                "pub static mut xcb_{}_id: xcb_extension_t;",
                &self.xcb_mod
            )?;
        }

        let out = &mut self.rs;
        writeln!(
            out,
            "// Generated automatically from {}.xml by rs_client.py version 0.9.0.",
            &self.xcb_mod
        )?;
        writeln!(out, "// Do not edit!")?;
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
        writeln!(out, "use libc::{{self, c_char, c_int, c_uint, c_void}};")?;
        writeln!(out, "use std;")?;
        writeln!(out, "use std::iter::Iterator;")?;
        writeln!(out, "")?;

        if self.xcb_mod != "xproto" {
            let out = &mut self.rs;
            writeln!(out)?;
            writeln!(out, "pub fn id() -> &'static mut base::Extension {{")?;
            writeln!(out, "    unsafe {{")?;
            writeln!(out, "        &mut xcb_{}_id", &self.xcb_mod)?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
        }

        Ok(())
    }

    pub fn epilogue(&mut self) -> io::Result<()> {
        let linklib = match self.xcb_mod.as_str() {
            "xproto" | "big_requests" | "xc_misc" => "xcb".to_owned(),
            "genericevent" => "xcb-ge".to_owned(),
            "x_print" => "xcb-xprint".to_owned(),
            "test" => "xcb-xtest".to_owned(),
            m => {
                let mut l = "xcb-".to_owned();
                l.push_str(m);
                l
            }
        };

        let out = &mut self.ffi;
        // write out all the external functions
        writeln!(out)?;
        writeln!(out, "#[link(name = \"{}\")]", linklib)?;
        writeln!(out, "extern {{")?;

        out.write_all(self.ffi_buf.get_ref())?;

        writeln!(out)?;
        writeln!(out, "}} // extern")?;

        let out = &mut self.rs;
        out.write_all(self.rs_buf.get_ref())?;
        Ok(())
    }

    pub fn event(&mut self, ev: Event) -> io::Result<()> {
        match ev {
            Event::Typedef { oldname, newname } => {
                self.notify_typ(newname.clone());

                let ffi_old_typ = self.ffi_use_type_name(&oldname);
                let ffi_new_typ = self.ffi_decl_type_name(&newname);

                emit_type_alias(&mut self.ffi, &ffi_new_typ, &ffi_old_typ)?;
                self.emit_ffi_iterator(&newname, &ffi_new_typ, false)?;

                let rs_new_typ = rust_type_name(&newname);
                emit_type_alias(&mut self.rs, &rs_new_typ, &ffi_new_typ)?;

                let is_simple = self.typ_is_simple(&oldname);
                let is_pod = self.typ_is_pod(&oldname);

                self.reg_type(
                    newname,
                    ffi_new_typ,
                    rs_new_typ,
                    self.ffi_type_sizeof(&oldname),
                    false,
                    is_simple,
                    is_pod,
                )
            }
            Event::XidType(name) => self.emit_xid(name)?,
            Event::XidUnion(xidun) => self.emit_xid(xidun.name)?,
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
                self.reg_type(en.name, ffi_typ, rs_typ, Some(4), false, true, true);
            }
            Event::Struct(stru) => self.emit_struct(stru)?,
            Event::Union(stru) => self.emit_union(stru)?,
            Event::Error(number, stru) => self.emit_error(number, stru)?,
            Event::ErrorCopy { name, number, ref_ } => {
                self.emit_error_copy(&name, number, &ref_)?
            }
            Event::Event {
                number,
                stru,
                no_seq_number,
                xge,
                ..
            } => self.emit_event(number, stru, no_seq_number, xge)?,
            Event::Request(req) => self.emit_request(req)?,
            _ => {}
        }
        Ok(())
    }

    // pub fn xcb_mod(&self) -> &str {
    //     &self.xcb_mod
    // }

    fn reg_type(
        &mut self,
        typ: String,
        ffi_typ: String,
        rs_typ: String,
        ffi_sz: Option<usize>,
        is_union: bool,
        is_simple: bool,
        is_pod: bool,
    ) {
        self.ffi_typ_reg.insert(ffi_typ);
        self.rs_typ_reg.insert(rs_typ);
        if is_union {
            self.typ_unions.insert(typ.clone());
        }
        if is_simple {
            self.typ_simple.insert(typ.clone());
        }
        if is_pod {
            self.typ_pod.insert(typ.clone());
        }
        self.ffi_type_sizes.insert(typ, ffi_sz);
    }

    /// notifies that a XCB type was declared in this module
    /// alternative to reg_type, with less info
    /// this is useful for replies, cookies, etc. and other derived types
    fn notify_typ(&mut self, typ: String) {
        self.ffi_type_sizes.insert(typ, None);
    }

    fn has_ffi_type(&self, typ: &str) -> bool {
        self.ffi_typ_reg.contains(typ)
    }

    fn has_rs_type(&self, typ: &str) -> bool {
        self.rs_typ_reg.contains(typ)
    }

    fn has_type(&self, typ: &str) -> bool {
        self.ffi_type_sizes.contains_key(typ)
    }

    fn typ_is_simple(&self, typ: &str) -> bool {
        match typ {
            "CARD8" => true,
            "CARD16" => true,
            "CARD32" => true,
            "CARD64" => true,
            "INT8" => true,
            "INT16" => true,
            "INT32" => true,
            "INT64" => true,
            "BYTE" => true,
            "BOOL" => true,
            "char" => true,
            "float" => true,
            "double" => true,
            "void" => true,
            typ => {
                self.typ_simple.contains(typ)
                    || self.dep_info.iter().any(|di| di.typ_simple.contains(typ))
            }
        }
    }

    fn typ_is_pod(&self, typ: &str) -> bool {
        self.typ_is_simple(&typ)
            || self.typ_pod.contains(typ)
            || self.dep_info.iter().any(|di| di.typ_pod.contains(typ))
    }

    fn fields_are_pod(&self, fields: &[StructField]) -> bool {
        for f in fields.iter() {
            match f {
                StructField::Field { typ, .. } => {
                    if !self.typ_is_pod(&typ) {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
        true
    }

    fn type_has_lifetime(&self, typ: &str) -> bool {
        if self.typ_with_lifetime.contains(typ) {
            true
        } else {
            for di in &self.dep_info {
                if di.typ_with_lifetime.contains(typ) {
                    return true;
                }
            }
            false
        }
    }

    /// FFI type name
    fn ffi_decl_type_name(&self, typ: &str) -> String {
        let typ = tit_split(typ).to_ascii_lowercase();
        format!("xcb_{}{}_t", &self.xcb_mod_prefix, typ)
    }

    // fn ffi_typ_base<'a>(&self, typ: &'a str) -> (Option<&'a str>, String) {
    //     let (module, typ) = extract_module(&typ);

    //     if let Some(module) = module {
    //         let mod_prefix = if module == "xproto" { "" } else { module };
    //         let typ = format!("xcb_{}{}_t", &mod_prefix, typ);

    //         if module == "xproto" || module == self.xcb_mod {
    //             (None, typ)
    //         } else {
    //             (Some(module), typ)
    //         }
    //     } else {
    //         let mod_prefix = if self.has_type(typ) {
    //             &self.xcb_mod_prefix
    //         } else {
    //             let mut pref = "";

    //             for di in self.dep_info.iter() {
    //                 if di.has_type(typ) {
    //                     pref = &di.xcb_mod_prefix;
    //                     break;
    //                 }
    //             }

    //             pref
    //         };
    //         let typ = tit_split(typ).to_ascii_lowercase();
    //         (None, format!("xcb_{}{}_t", mod_prefix, typ))
    //     }
    // }

    /// same as ffi_decl_type_name but can also have a namespace before (with a single colon)
    fn ffi_use_type_name(&self, typ: &str) -> String {
        match typ {
            "CARD8" => "u8".into(),
            "CARD16" => "u16".into(),
            "CARD32" => "u32".into(),
            "CARD64" => "u64".into(),
            "INT8" => "i8".into(),
            "INT16" => "i16".into(),
            "INT32" => "i32".into(),
            "BYTE" => "u8".into(),
            "BOOL" => "u8".into(),
            "char" => "c_char".into(),
            "float" => "f32".into(),
            "double" => "f64".into(),
            "void" => "c_void".into(),
            typ => {
                let (module, typ) = extract_module(&typ);

                if let Some(module) = module {
                    let typ = tit_split(typ).to_ascii_lowercase();
                    let mod_prefix = if module == "xproto" {
                        String::new()
                    } else {
                        format!("{}_", module)
                    };
                    format!("xcb_{}{}_t", &mod_prefix, typ)
                } else {
                    let mod_prefix = if self.has_type(typ) {
                        &self.xcb_mod_prefix
                    } else {
                        let mut pref = "";

                        for di in self.dep_info.iter() {
                            if di.has_type(typ) {
                                pref = &di.xcb_mod_prefix;
                                break;
                            }
                        }

                        pref
                    };
                    let typ = tit_split(typ).to_ascii_lowercase();
                    format!("xcb_{}{}_t", mod_prefix, typ)
                }
            }
        }
    }

    fn ffi_iterator_name(&self, typ: &str) -> String {
        let mod_prefix = if self.has_type(typ) {
            &self.xcb_mod_prefix
        } else {
            let mut pref = "";

            for di in self.dep_info.iter() {
                if di.has_type(typ) {
                    pref = &di.xcb_mod_prefix;
                    break;
                }
            }

            pref
        };
        format!(
            "xcb_{}{}_iterator_t",
            &mod_prefix,
            tit_split(typ).to_ascii_lowercase()
        )
    }

    fn ffi_type_sizeof(&self, typ: &str) -> Option<usize> {
        // TODO: emit codegen result if typ is not registered instead of panic
        match typ {
            "CARD8" => Some(1),
            "CARD16" => Some(2),
            "CARD32" => Some(4),
            "CARD64" => Some(8),
            "INT8" => Some(1),
            "INT16" => Some(2),
            "INT32" => Some(4),
            "INT64" => Some(8),
            "BYTE" => Some(1),
            "BOOL" => Some(1),
            "char" => Some(1),
            "float" => Some(4),
            "double" => Some(8),
            "void" => Some(0),
            typ => {
                if let Some(sz) = self.ffi_type_sizes.get(typ) {
                    *sz
                } else {
                    // checking in the imported dependencies
                    for di in self.dep_info.iter() {
                        if let Some(sz) = di.ffi_type_sizes.get(typ) {
                            return *sz;
                        }
                    }
                    None
                }
            }
        }
    }

    fn eligible_to_copy(&self, stru: &Struct) -> bool {
        for f in stru.fields.iter() {
            match f {
                StructField::List { len_expr, .. } => {
                    if expr_fixed_length(len_expr).is_none() {
                        return false;
                    }
                }
                StructField::ValueParam { .. } => {
                    return false;
                }
                StructField::Switch(..) => {
                    return false;
                }
                StructField::ListNoLen { .. } => {
                    return false;
                }
                _ => {}
            }
        }
        true
    }

    fn ffi_enum_type_name(&mut self, typ: &str) -> String {
        let typ = tit_split(typ).to_ascii_lowercase();
        let try1 = format!("xcb_{}{}_t", self.xcb_mod_prefix, typ);
        // hardcoded exceptions: enum are defined before the homonym type
        match try1.as_str() {
            "xcb_render_picture_t" => {
                return "xcb_render_picture_enum_t".into();
            }
            "xcb_present_event_t" => {
                return "xcb_present_event_enum_t".into();
            }
            _ => {}
        }
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

    fn compute_ffi_struct_field_sizeof(&self, field: &StructField) -> Option<usize> {
        match field {
            StructField::Field { typ, .. } => self.ffi_type_sizeof(&typ),
            StructField::Pad(pad_sz) => Some(*pad_sz),
            StructField::List { typ, len_expr, .. } => {
                match (self.ffi_type_sizeof(typ), expr_fixed_length(len_expr)) {
                    (Some(sz), Some(len)) => Some(sz * len),
                    _ => None,
                }
            }
            StructField::ListNoLen { .. } => None,
            f => unimplemented!("{:?}", f),
        }
    }

    fn compute_ffi_struct_size(&self, stru: &Struct) -> Option<usize> {
        let mut stru_sz = Some(0usize);

        for f in stru.fields.iter() {
            match f {
                StructField::AlignPad(alignment) => match stru_sz.as_mut() {
                    Some(sz) => {
                        let curr = *sz % alignment;
                        if curr != 0 {
                            *sz += alignment - curr;
                        }
                    }
                    _ => {
                        return None;
                    }
                },
                field => {
                    match (
                        stru_sz.as_mut(),
                        self.compute_ffi_struct_field_sizeof(field),
                    ) {
                        (Some(stru_sz), Some(f_sz)) => {
                            *stru_sz += f_sz;
                        }
                        _ => {
                            return None;
                        }
                    }
                }
            }
        }

        stru_sz
    }

    fn compute_ffi_union_size(&self, stru: &Struct) -> usize {
        let mut biggest = 1;
        let mut alignment = 1;

        for f in stru.fields.iter() {
            let mut fs = self.ptr_width;
            let mut fa = self.ptr_width;
            match f {
                StructField::AlignPad(_) => panic!("Unexpected align pad in union"),
                StructField::Pad(_) => panic!("Unexpected pad in union"),
                StructField::Field { typ, .. } => {
                    if let Some(sz) = self.ffi_type_sizeof(typ) {
                        fs = sz;
                        fa = sz;
                    }
                }
                StructField::List { typ, len_expr, .. } => {
                    if let Some(sz) = self.ffi_type_sizeof(typ) {
                        fs = sz;
                        fa = sz;
                    }
                    if let Some(len) = expr_fixed_length(len_expr) {
                        fs = len * fa;
                    }
                }
                StructField::ListNoLen { .. } => panic!("Unexpected list without length in union"),

                f => unimplemented!("{:?}", f),
            }

            biggest = biggest.max(fs);
            alignment = alignment.max(fa);
        }

        let mut num_aligned = biggest / alignment;
        if biggest % alignment > 0 {
            num_aligned += 1;
        }
        num_aligned * alignment
    }

    fn emit_ffi_iterator(
        &mut self,
        name: &str,
        typ: &str,
        has_lifetime: bool,
    ) -> io::Result<String> {
        let it_typ = self.ffi_iterator_name(&name);
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

    fn emit_ffi_field_list_accessor(
        &mut self,
        ffi_typ: &str,
        xcb_name: &str,
        fname: &str,
        ftyp: &str,
        fixed_size: bool,
    ) -> io::Result<()> {
        let is_simple = self.typ_is_simple(&ftyp);

        let accessor_needed = fixed_size;
        let length_needed = true;
        let end_needed = is_simple;
        let iterator_needed = !is_simple;

        let has_lifetime = self.type_has_lifetime(ftyp);

        if accessor_needed {
            let ftyp = self.ffi_use_type_name(ftyp);
            let acc_fn =
                ffi_field_list_iterator_acc_fn_name(&self.xcb_mod_prefix, &xcb_name, &fname);
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(
                out,
                "pub fn {}(R: *const {}) -> *mut {};",
                &acc_fn, &ffi_typ, &ftyp
            )?;
        }

        if length_needed {
            let len_fn =
                ffi_field_list_iterator_len_fn_name(&self.xcb_mod_prefix, &xcb_name, &fname);
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(out, "pub fn {}(R: *const {}) -> c_int;", &len_fn, &ffi_typ)?;
        }

        if end_needed {
            let end_fn =
                ffi_field_list_iterator_end_fn_name(&self.xcb_mod_prefix, &xcb_name, &fname);
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(
                out,
                "pub fn {}(R: *const {}) -> xcb_generic_iterator_t;",
                &end_fn, &ffi_typ
            )?;
        }

        if iterator_needed {
            let lifetime = if has_lifetime { "<'a>" } else { "" };
            let it_fn = ffi_field_list_iterator_it_fn_name(&self.xcb_mod_prefix, &xcb_name, &fname);
            let it_typ = self.ffi_iterator_name(&ftyp);
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(
                out,
                "pub fn {}{}(R: *const {}) -> {}{};",
                &it_fn, &lifetime, &ffi_typ, &it_typ, &lifetime
            )?;
        }
        Ok(())
    }

    fn emit_ffi_field_list_accessors(
        &mut self,
        ffi_typ: &str,
        xcb_name: &str,
        fields: &[StructField],
    ) -> io::Result<()> {
        for f in fields {
            match f {
                StructField::List {
                    name,
                    typ,
                    len_expr,
                } => {
                    let fixed_size = self.ffi_type_sizeof(&typ).is_some();
                    let fixed_len = expr_fixed_length(&len_expr).is_some();

                    if fixed_size && fixed_len {
                        continue;
                    }

                    self.emit_ffi_field_list_accessor(
                        &ffi_typ, &xcb_name, &name, &typ, fixed_size,
                    )?;
                }
                StructField::ValueParam { list_name, .. } => {
                    self.emit_ffi_field_list_accessor(
                        &ffi_typ, &xcb_name, &list_name, "CARD32", true,
                    )?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn emit_ffi_struct(&mut self, stru: &Struct, must_pack: bool, no_copy: bool) -> io::Result<String> {
        let Struct { name, fields, doc } = &stru;

        let ffi_typ = self.ffi_decl_type_name(&name);
        let impl_copy_clone = self.eligible_to_copy(&stru) && !no_copy;

        {
            let must_pack = if must_pack { ", packed" } else { "" };

            let out = &mut self.ffi;
            writeln!(out)?;
            emit_doc_text(out, &doc)?;
            writeln!(out, "#[repr(C{})]", must_pack)?;
            writeln!(out, "pub struct {} {{", &ffi_typ)?;
        }

        let mut padnum = 0;

        // cases of ValueParam were the mask is declared as a field in the struct
        // before the actual ValueParam field. We keep track of all fields written
        // to avoid doubles
        let mut written_fields = Vec::new();

        for f in fields.iter() {
            match f {
                StructField::Field { name, typ, .. } | StructField::Expr { name, typ, .. } => {
                    let typ = self.ffi_use_type_name(&typ);
                    let out = &mut self.ffi;
                    emit_doc_field(out, &doc, &name)?;
                    writeln!(out, "    pub {}: {},", symbol(&name), &typ,)?;
                    written_fields.push(name.as_str());
                }
                StructField::Pad(sz) => {
                    let out = &mut self.ffi;
                    let padtyp = match sz {
                        1 => "u8".into(),
                        x => format!("[u8; {}]", x),
                    };
                    writeln!(out, "    pub pad{}: {},", padnum, padtyp)?;
                    padnum += 1;
                }
                StructField::List {
                    name,
                    typ,
                    len_expr,
                } => {
                    if let Some(sz) = expr_fixed_length(&len_expr) {
                        let typ = self.ffi_use_type_name(&typ);
                        let out = &mut self.ffi;

                        emit_doc_field(out, &doc, &name)?;
                        writeln!(out, "    pub {}: [{}; {}],", symbol(&name), &typ, sz)?;
                    }
                }
                StructField::ValueParam {
                    mask_name,
                    mask_typ,
                    ..
                } => {
                    if written_fields.contains(&mask_name.as_str()) {
                        continue;
                    }
                    let mask_typ = self.ffi_use_type_name(&mask_typ);
                    let out = &mut self.ffi;
                    emit_doc_field(out, &doc, &mask_name)?;
                    writeln!(out, "    pub {}: {},", symbol(&mask_name), mask_typ,)?;
                }
                _ => {}
            }
        }

        let out = &mut self.ffi;
        writeln!(out, "}}")?;

        if impl_copy_clone {
            emit_copy_clone(out, &ffi_typ)?;
        }

        writeln!(out, "impl ::std::fmt::Debug for {} {{", &ffi_typ)?;
        writeln!(
            out,
            "    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {{"
        )?;
        writeln!(out, "        fmt.debug_struct(\"{}\")", &ffi_typ)?;
        let mut padnum = 0;
        for f in fields.iter() {
            match f {
                StructField::Field { name, .. } | StructField::Expr { name, .. } => {
                    let name = symbol(name);
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
                StructField::List { name, len_expr, .. } => {
                    if expr_fixed_length(&len_expr).is_some() {
                        let name = symbol(name);
                        writeln!(out, "            .field(\"{}\", &&self.{}[..])", name, name)?;
                    }
                }
                StructField::ValueParam { mask_name, .. } => {
                    if written_fields.contains(&mask_name.as_str()) {
                        continue;
                    }
                    writeln!(
                        out,
                        "            .field(\"{}\", &self.{})",
                        mask_name, mask_name,
                    )?;
                }
                _ => {}
            }
        }
        writeln!(out, "            .finish()")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(ffi_typ)
    }

    fn emit_rs_struct_field_accessors(
        &mut self,
        stru: &Struct,
        accessor: &str,
        skip_fields: &[&str],
        has_lifetime: bool,
    ) -> io::Result<()> {
        for f in stru.fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    if skip_fields.iter().find(|skip| skip == &name).is_some() {
                        continue;
                    }
                    let is_simple = self.typ_is_simple(&typ);
                    let is_pod = self.typ_is_pod(&typ);

                    let out = &mut self.rs_buf;
                    emit_doc_field(out, &stru.doc, &name)?;
                    let name = symbol(name);
                    if !self.typ_unions.contains(typ) {
                        writeln!(
                            out,
                            "    pub fn {}(&self) -> {} {{",
                            &name,
                            rust_field_type_name(&self.xcb_mod, &typ)
                        )?;
                        if typ == "BOOL" {
                            writeln!(out, "        unsafe {{ {}.{} != 0 }}", &accessor, &name)?;
                        } else if is_pod && !is_simple {
                            writeln!(
                                out,
                                "        unsafe {{ std::mem::transmute({}.{}) }}",
                                &accessor, &name
                            )?;
                        } else {
                            writeln!(out, "        unsafe {{ {}.{} }}", &accessor, &name)?;
                        }
                        writeln!(out, "    }}")?;
                    } else {
                        // unions are returned by ref and need lifetime (TODO: check if really needed)
                        // adding lifetime declaration if not already declared in the impl opening
                        let lifetime_decl = if has_lifetime { "" } else { "<'a>" };
                        writeln!(
                            out,
                            "    pub fn {}{}(&'a self) -> &'a {} {{",
                            &name,
                            &lifetime_decl,
                            rust_field_type_name(&self.xcb_mod, &typ)
                        )?;
                        writeln!(out, "        unsafe {{ &{}.{} }}", &accessor, &name)?;
                        writeln!(out, "    }}")?;
                    }
                }
                StructField::List {
                    name,
                    typ,
                    len_expr,
                } => {
                    if skip_fields.iter().find(|skip| skip == &name).is_some() {
                        continue;
                    }
                    let name = symbol(name);
                    let is_simple = self.typ_is_simple(&typ);
                    let is_string = typ == "char";
                    let fixed_len = expr_fixed_length(len_expr);

                    let out = &mut self.rs_buf;

                    if fixed_len.is_some() {
                        let typ = rust_field_type_name(&self.xcb_mod, &typ);
                        writeln!(out, "    pub fn {}(&self) -> &[{}] {{", &name, &typ)?;
                        writeln!(out, "        unsafe {{ &(*self.ptr).{} }}", &name)?;
                        writeln!(out, "    }}")?;
                    } else if is_simple {
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
                        let is_template = typ == "void";
                        let template = if is_template { "<T>" } else { "" };
                        let ret = match typ.as_str() {
                            "char" => "&str".to_string(),
                            "void" => "&[T]".to_string(),
                            typ => format!("&[{}]", rust_field_type_name(&self.xcb_mod, &typ)),
                        };
                        writeln!(
                            out,
                            "    pub fn {}{}(&self) -> {} {{",
                            &name, &template, &ret
                        )?;
                        writeln!(out, "        unsafe {{")?;
                        writeln!(out, "            let field = self.ptr;")?;
                        writeln!(out, "            let len = {}(field) as usize;", &len_fn)?;
                        writeln!(out, "            let data = {}(field);", &acc_fn)?;
                        if is_string {
                            writeln!(out, "            let slice = std::slice::from_raw_parts(data as *const u8, len);")?;
                            writeln!(out, "            // should we check what comes from X?")?;
                            writeln!(out, "            std::str::from_utf8_unchecked(&slice)")?;
                        } else if is_template {
                            writeln!(
                                out,
                                "            debug_assert_eq!(len % std::mem::size_of::<T>(), 0);"
                            )?;
                            writeln!(out, "            std::slice::from_raw_parts(data as *const T, len / std::mem::size_of::<T>())")?;
                        } else {
                            writeln!(out, "            std::slice::from_raw_parts(data, len)")?;
                        }
                        writeln!(out, "        }}")?;
                        writeln!(out, "    }}")?;
                    } else {
                        let has_lifetime = has_lifetime && self.typ_with_lifetime.contains(typ);
                        let lifetime = if has_lifetime { "<'a>" } else { "" };
                        let it_typ = rust_iterator_type_name(&typ);
                        let ffi_it_fn_name = ffi_field_list_iterator_it_fn_name(
                            &self.xcb_mod_prefix,
                            &stru.name,
                            &name,
                        );
                        writeln!(
                            out,
                            "    pub fn {}(&self) -> {}{} {{",
                            &name, &it_typ, &lifetime
                        )?;
                        writeln!(out, "        unsafe {{ {}(self.ptr) }}", &ffi_it_fn_name)?;
                        writeln!(out, "    }}")?;
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn emit_rs_struct_impl(
        &mut self,
        rs_typ: &str,
        ffi_typ: &str,
        stru: &Struct,
        has_lifetime: bool,
        is_pod: bool,
    ) -> io::Result<()> {
        let (accessor, lifetime) = if has_lifetime {
            ("(*self.ptr)", "<'a>")
        } else {
            ("self.base", "")
        };
        // emitting struct impl
        let out = &mut self.rs_buf;

        writeln!(out)?;
        writeln!(out, "impl{} {}{} {{", &lifetime, &rs_typ, &lifetime)?;

        // emitting ctor
        if is_pod {
            writeln!(out, "    #[allow(unused_unsafe)]")?;
            write!(out, "    pub fn new(")?;
            for f in stru.fields.iter() {
                match f {
                    StructField::Field { name, typ, .. } => {
                        write!(
                            out,
                            "{}: {},",
                            symbol(&name),
                            rust_field_type_name(&self.xcb_mod, &typ)
                        )?;
                    }
                    _ => {}
                }
            }
            writeln!(out, ") -> {} {{", &rs_typ)?;
            writeln!(out, "        unsafe {{")?;
            writeln!(out, "            {} {{", &rs_typ)?;
            writeln!(out, "                base: {} {{", &ffi_typ)?;
            let mut padnum = 0;
            for f in stru.fields.iter() {
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
        self.emit_rs_struct_field_accessors(&stru, accessor, &[], has_lifetime)?;

        writeln!(&mut self.rs_buf, "}}")?;

        Ok(())
    }

    fn emit_rs_iterator(
        &mut self,
        name: &str,
        typ: &str,
        ffi_it_typ: &str,
        has_lifetime: bool,
        is_union: bool,
    ) -> io::Result<()> {
        let it_typ = format!("{}Iterator", &typ);
        let ffi_it_next = ffi_iterator_next_fn_name(&self.xcb_mod_prefix, &name);

        let lifetime = if has_lifetime { "<'a>" } else { "" };

        let return_expr = match (has_lifetime, is_union) {
            (true, true) => unimplemented!(),
            (true, false) => "std::mem::transmute(data)",
            (false, true) => "*data",
            (false, false) => "std::mem::transmute(*data)",
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
        writeln!(out, "                Some({})", &return_expr)?;
        writeln!(out, "            }}")?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }

    fn emit_rs_struct(
        &mut self,
        ffi_typ: &str,
        stru: &Struct,
        has_lifetime: bool,
    ) -> io::Result<String> {
        let Struct { name, doc, .. } = &stru;

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

        self.emit_rs_struct_impl(&typ, &ffi_typ, &stru, has_lifetime, !has_lifetime)?;

        Ok(typ)
    }

    fn emit_rs_union_impl(&mut self, rs_typ: &str, ffi_sz: usize, stru: &Struct) -> io::Result<()> {
        let out = &mut self.rs_buf;

        let Struct { fields, doc, .. } = &stru;

        writeln!(out)?;
        writeln!(out, "impl {} {{", &rs_typ)?;

        // emitting accessors
        for f in fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    let name = symbol(name);
                    let f_rs_typ = rust_field_type_name(&self.xcb_mod, &typ);
                    emit_doc_field(out, &doc, &name)?;

                    writeln!(out)?;
                    writeln!(out, "    pub fn {}(&self) -> {} {{", &name, &f_rs_typ)?;
                    writeln!(out, "        unsafe {{")?;
                    writeln!(
                        out,
                        "            let _ptr = self.data.as_ptr() as *const {};",
                        &f_rs_typ
                    )?;
                    writeln!(out, "            *_ptr")?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                    writeln!(
                        out,
                        "    pub fn from_{}({}: {}) -> {} {{",
                        &name, &name, &f_rs_typ, &rs_typ
                    )?;
                    writeln!(out, "        unsafe {{")?;
                    writeln!(
                        out,
                        "            let mut res = {} {{ data: [0; {}] }};",
                        &rs_typ, ffi_sz
                    )?;
                    writeln!(
                        out,
                        "            let res_ptr = self.data.as_mut_ptr() as *mut {};",
                        &f_rs_typ
                    )?;
                    writeln!(out, "            *res_ptr = {};", &name)?;
                    writeln!(out, "            res")?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                }
                StructField::List {
                    name,
                    typ,
                    len_expr,
                } => {
                    emit_doc_field(out, &doc, &name)?;
                    let name = symbol(name);
                    let f_rs_typ = rust_field_type_name(&self.xcb_mod, &typ);
                    let len =
                        expr_fixed_length(len_expr).expect("union list field with variable length");
                    // accessor
                    writeln!(out, "    pub fn {}(&self) -> &[{}] {{", &name, &f_rs_typ)?;
                    writeln!(out, "        unsafe {{")?;
                    writeln!(
                        out,
                        "            let ptr = self.data.as_ptr() as *const {};",
                        &f_rs_typ
                    )?;
                    writeln!(out, "            std::slice::from_raw_parts(ptr, {})", len)?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                    // constructor
                    writeln!(
                        out,
                        "    pub fn from_{}({}: [{}; {}]) -> {} {{",
                        &name, &name, &f_rs_typ, len, &rs_typ
                    )?;
                    writeln!(out, "        unsafe {{")?;
                    writeln!(out, "            {} {{", &rs_typ)?;
                    writeln!(out, "                data: std::mem::transmute({})", &name)?;
                    writeln!(out, "            }}")?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                }
                _ => {
                    unimplemented!();
                }
            }
        }
        writeln!(out, "}}")?;

        Ok(())
    }

    fn emit_rs_event(
        &mut self,
        name: &str,
        number: i32,
        stru: &Struct,
        ffi_typ: &str,
        opcopies: &[OpCopy],
        xge: bool,
    ) -> io::Result<String> {
        let rs_typ = rust_type_name(&stru.name);

        let opname = rust_opname(&name);
        let num_typ = if number < 0 { "i8" } else { "u8" };

        {
            let out = &mut self.rs_buf;
            writeln!(out)?;
            writeln!(out, "pub const {}: {} = {};", &opname, &num_typ, number)?;

            writeln!(out)?;
            emit_doc_text(out, &stru.doc)?;
            writeln!(out, "pub type {} = base::Event<{}>;", &rs_typ, &ffi_typ)?;

            writeln!(out)?;
            writeln!(out, "impl {} {{", &rs_typ)?;
        }

        let field_skip = {
            let mut fs = if xge {
                vec![
                    "sequence",
                    "extension",
                    "length",
                    "event_type",
                    "full_sequence",
                ]
            } else {
                vec!["sequence"]
            };
            if opcopies.is_empty() {
                fs.push("response_type");
            }
            fs
        };

        {
            let mut fs = field_skip.clone();
            fs.push("response_type");
            self.emit_rs_struct_field_accessors(&stru, "(*self.ptr)", &fs, false)?;
        }

        // emitting ctor

        {
            let out = &mut self.rs_buf;

            writeln!(out, "    /// Constructs a new {}", &rs_typ)?;
            if opcopies.is_empty() {
                writeln!(
                    out,
                    "    /// `response_type` will be set automatically to {}",
                    &opname
                )?;
            } else {
                writeln!(out, "    /// `response_type` must be set to one of:")?;
                writeln!(out, "    ///     - `{}`", &opname)?;
                for op in opcopies.iter() {
                    let opname = rust_opname(&op.name);
                    writeln!(out, "    ///     - `{}`", &opname)?;
                }
            }

            writeln!(out, "    pub fn new (")?;

            for f in stru.fields.iter() {
                match f {
                    StructField::Field { name, typ, .. } => {
                        if field_skip.iter().find(|skip| skip == &name).is_some() {
                            continue;
                        }
                        writeln!(
                            out,
                            "        {}: {},",
                            symbol(name),
                            &rust_field_type_name(&self.xcb_mod, &typ)
                        )?;
                    }
                    StructField::Pad(_) => {}
                    StructField::List {
                        name,
                        typ,
                        len_expr,
                    } => {
                        if field_skip.iter().find(|skip| skip == &name).is_some() {
                            continue;
                        }
                        let typ = match typ.as_str() {
                            "char" => "&str".into(),
                            typ => {
                                let typ = rust_field_type_name(&self.xcb_mod, &typ);
                                if let Some(len) = expr_fixed_length(&len_expr) {
                                    format!("[{}; {}]", &typ, len)
                                } else {
                                    format!("&[{}]", &typ)
                                }
                            }
                        };
                        writeln!(out, "        {}: {},", symbol(name), &typ)?;
                    }
                    StructField::ListNoLen { .. } => {}
                    _ => unimplemented!("{}::{}::{:?}", self.xcb_mod, &rs_typ, f),
                }
            }

            writeln!(out, "    ) -> {} {{", &rs_typ)?;
            writeln!(out, "        unsafe {{")?;
            writeln!(
                out,
                "            let raw = libc::malloc(32 as usize) as *mut {};",
                &ffi_typ
            )?;
            if !opcopies.is_empty() {
                let copies = opcopies.iter().map(|opc| rust_opname(&opc.name));
                let opcodes = Some(opname.clone()).into_iter().chain(copies);
                let assert_expr = opcodes
                    .map(|opc| format!("response_type == {}", &opc))
                    .collect::<Vec<String>>()
                    .join(" || ");
                writeln!(
                    out,
                    "            assert!({}, \"wrong response_type supplied to {}::new\");",
                    &assert_expr, &rs_typ
                )?;
            } else {
                writeln!(out, "            (*raw).response_type = {};", &opname)?;
            }

            for f in stru.fields.iter() {
                match f {
                    StructField::Field { name, typ, .. } => {
                        if field_skip.iter().find(|skip| skip == &name).is_some() {
                            continue;
                        }
                        let name = symbol(name);
                        let expr = if typ == "BOOL" {
                            format!("if {} {{ 1 }} else {{ 0 }}", name)
                        } else {
                            name.into()
                        };
                        writeln!(out, "            (*raw).{} = {};", name, expr)?;
                    }
                    StructField::List { name, .. } => {
                        let name = symbol(name);
                        writeln!(out, "            (*raw).{} = {};", name, name)?;
                    }
                    _ => {}
                }
            }
            writeln!(out, "            {} {{ ptr: raw }}", &rs_typ)?;
            writeln!(out, "        }}")?; // closing unsafe
            writeln!(out, "    }}")?; // closing new
            writeln!(out, "}}")?; // closing impl
        }

        for opc in opcopies.iter() {
            let opname = rust_opname(&opc.name);
            let name = opc.name.clone() + "Event";
            let num_typ = if number < 0 { "i8" } else { "u8" };
            let rs_typ = rust_type_name(&name);
            let ffi_typ = self.ffi_decl_type_name(&name);

            let out = &mut self.rs_buf;
            writeln!(out)?;
            writeln!(
                out,
                "pub const {}: {} = {};",
                &opname, &num_typ, &opc.number
            )?;

            writeln!(out)?;
            emit_doc_text(out, &stru.doc)?;
            writeln!(out, "pub type {} = base::Event<{}>;", &rs_typ, &ffi_typ)?;
        }

        Ok(rs_typ)
    }

    fn emit_ffi_switch_struct(
        &mut self,
        typ_name: &str,
        switch_name: &str,
        _expr: &Expr<usize>,
        cases: &[SwitchCase],
    ) -> io::Result<String> {
        let fields = {
            let mut fields = Vec::new();
            for c in cases.iter() {
                fields.append(&mut c.fields.clone());
            }
            fields
        };

        let stru = Struct {
            name: typ_name.to_string() + &capitalize(switch_name),
            fields,
            doc: None,
        };
        let typ = self.emit_ffi_struct(&stru, false, true)?;
        Ok(typ)
    }

    fn emit_ffi_req_fn(
        &mut self,
        req_name: &str,
        fn_name: &str,
        cookie_name: &str,
        fields: &[StructField],
        doc: &Option<Doc>,
    ) -> io::Result<()> {
        let cookie_typ = self.ffi_use_type_name(&cookie_name);
        {
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            emit_doc_text(out, doc)?;
            writeln!(out, "    pub fn {} (", &fn_name)?;
            writeln!(out, "        c: *mut xcb_connection_t,")?;
        }
        let mut written_fields = Vec::new();
        for f in fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    written_fields.push(name);
                    let name = symbol(&name);
                    let typ = self.ffi_use_type_name(&typ);
                    writeln!(&mut self.ffi_buf, "        {}: {},", &name, &typ)?;
                }
                StructField::Fd(name) => {
                    let name = symbol(&name);
                    writeln!(&mut self.ffi_buf, "        {}: i32,", &name)?;
                }
                StructField::ValueParam {
                    mask_typ,
                    mask_name,
                    list_name,
                } => {
                    let mask_typ = self.ffi_use_type_name(&mask_typ);
                    let list_name = symbol(list_name);

                    let out = &mut self.ffi_buf;
                    if !written_fields.contains(&mask_name) {
                        let mask_name = symbol(mask_name);
                        writeln!(out, "        {}: {},", &mask_name, &mask_typ)?;
                    }
                    writeln!(out, "        {}: *const u32,", &list_name)?;
                }
                StructField::List { name, typ, .. } => {
                    let name = symbol(&name);
                    let typ = self.ffi_use_type_name(&typ);
                    let out = &mut self.ffi_buf;
                    writeln!(out, "        {}: *const {},", &name, &typ)?;
                }
                StructField::ListNoLen { name, typ } => {
                    let len_name = name.clone() + "_len";
                    let name = symbol(&name);
                    let typ = self.ffi_use_type_name(&typ);
                    let out = &mut self.ffi_buf;
                    writeln!(out, "        {}: u32,", &len_name)?;
                    writeln!(out, "        {}: *const {},", &name, &typ)?;
                }
                StructField::Switch(name, ..) => {
                    let name = symbol(&name);
                    let typ = ffi_switch_struct_name(&self.xcb_mod_prefix, &req_name, &name);
                    writeln!(&mut self.ffi_buf, "        {}: *const {},", &name, &typ)?;
                }
                _ => {}
            }
        }

        let out = &mut self.ffi_buf;
        writeln!(out, "    ) -> {};", &cookie_typ)?;

        Ok(())
    }

    fn emit_ffi_reply(
        &mut self,
        req_name: &str,
        mut reply: Reply,
    ) -> io::Result<(String, String, String)> {
        // writting cookie struct
        let cookie_name = req_name.to_string() + "Cookie";
        let cookie_ffi_typ = self.ffi_decl_type_name(&cookie_name);
        let reply_fields = {
            let mut fields = vec![
                make_field("response_type".into(), "CARD8".into()),
                if reply.fields.is_empty() {
                    StructField::Pad(1usize)
                } else {
                    reply.fields.remove(0)
                },
                make_field("sequence".into(), "CARD16".into()),
                make_field("length".into(), "CARD32".into()),
            ];
            fields.append(&mut reply.fields);
            fields
        };
        let reply = Struct {
            name: req_name.to_string() + "Reply",
            fields: reply_fields,
            doc: reply.doc,
        };

        {
            let out = &mut self.ffi;
            writeln!(out)?;
            writeln!(out, "#[derive(Copy, Clone, Debug)]")?;
            writeln!(out, "#[repr(C)]")?;
            writeln!(out, "pub struct {} {{", &cookie_ffi_typ)?;
            writeln!(out, "    pub(crate) sequence: c_uint,")?;
            writeln!(out, "}}")?;
        }

        let ffi_reply_typ = self.emit_ffi_struct(&reply, false, false)?;

        self.emit_ffi_field_list_accessors(&ffi_reply_typ, &req_name, &reply.fields)?;

        let ffi_reply_fn = ffi_reply_fn_name(&self.xcb_mod_prefix, &req_name);
        {
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(
                out,
                "    /// the returned value must be freed by the caller using libc::free()."
            )?;
            writeln!(out, "    pub fn {} (", &ffi_reply_fn)?;
            writeln!(out, "        c: *mut xcb_connection_t,")?;
            writeln!(out, "        cookie: {},", &cookie_ffi_typ)?;
            writeln!(out, "        error: *mut *mut xcb_generic_error_t,")?;
            writeln!(out, "    ) -> *mut {};", &ffi_reply_typ)?;
        }

        if has_fd(&reply.fields) {
            let fds_fn = ffi_reply_fds_fn_name(&self.xcb_mod_prefix, &req_name);
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(out, "    pub fn {}(", &fds_fn)?;
            writeln!(out, "        c: *mut xcb_connection_t,")?;
            writeln!(out, "        reply: *mut {},", &ffi_reply_typ)?;
            writeln!(out, "    ) -> *mut c_int;")?;
        }

        Ok((cookie_ffi_typ, ffi_reply_fn, ffi_reply_typ))
    }

    fn emit_rs_req_fn(
        &mut self,
        req_name: &str,
        cookie_name: &str,
        params: &[StructField],
        doc: &Option<Doc>,
        checked: bool,
    ) -> io::Result<()> {
        // special case for the send_event request
        let send_event = match (req_name, self.xcb_mod.as_str()) {
            ("SendEvent", "xproto") => true,
            ("SendEventChecked", "xproto") => true,
            (_, _) => false,
        };
        let has_template = request_has_template(&params) || send_event;
        let list_fields = ListField::fetch_from(&params);
        let skip_fields = {
            let mut sf = Vec::new();

            for f in params.iter() {
                if let StructField::ValueParam { mask_name, .. } = f {
                    sf.push(mask_name.clone());
                }
            }

            sf
        };

        {
            let out = &mut self.rs_buf;

            writeln!(out)?;
            emit_doc_text(out, &doc)?;
            if let Some(_) = &doc {
                writeln!(out, "///")?;
                writeln!(out, "/// parameters:")?;
                writeln!(out, "///")?;
                writeln!(out, "///   - __c__:")?;
                writeln!(out, "///       The connection object to the server")?;
                for f in params.iter() {
                    if let Some(name) = rust_field_doc_name(&f) {
                        writeln!(out, "///")?;
                        writeln!(out, "///   - __{}__:", &name)?;
                        emit_doc_field_indent(out, &doc, name, "       ")?;
                    }
                }
            }
            let fn_name = rust_request_fn_name(&req_name);
            let ffi_fn_name = ffi_request_fn_name(&self.xcb_mod_prefix, &req_name);
            let template = if has_template { ", T" } else { "" };
            writeln!(out, "pub fn {}<'a{}>(", &fn_name, &template)?;

            // function parameters
            writeln!(out, "    c: &'a base::Connection,")?;
            for f in params.iter() {
                match f {
                    StructField::Field { name, typ, .. } => {
                        if skip_fields.contains(&name) {
                            continue;
                        }
                        if list_fields.iter().any(|lf| &lf.lenfield == name) {
                            continue;
                        }
                        let name = symbol(&name);
                        let typ = rust_field_type_name(&self.xcb_mod, &typ);
                        writeln!(out, "    {}: {},", name, typ)?;
                    }
                    StructField::List { name, typ, .. } | StructField::ListNoLen { name, typ } => {
                        let name = symbol(&name);
                        let typ = match (send_event, name, typ.as_str()) {
                            (true, "event", _) => "&base::Event<T>".to_string(),
                            (_, _, "char") => "&str".to_string(),
                            (_, _, "void") => "&[T]".to_string(),
                            (_, _, typ) => {
                                format!("&[{}]", rust_field_type_name(&self.xcb_mod, &typ))
                            }
                        };
                        writeln!(out, "    {}: {},", name, typ)?;
                    }
                    StructField::ValueParam {
                        list_name,
                        mask_typ,
                        ..
                    } => {
                        let name = symbol(&list_name);
                        let typ = rust_field_type_name(&self.xcb_mod, &mask_typ);
                        writeln!(out, "    {}: &[({}, u32)],", &name, &typ)?;
                    }
                    _ => {}
                }
            }
            writeln!(out, ") -> {}<'a> {{", cookie_name)?;
            writeln!(out, "    unsafe {{")?;

            // local variables
            if send_event {
                writeln!(
                    out,
                    "        let event_ptr = std::mem::transmute(event.ptr);"
                )?;
            }
            for ListField { name, typ, .. } in list_fields.iter() {
                let name = symbol(&name);
                if typ == "char" {
                    writeln!(out, "        let {} = {}.as_bytes();", &name, &name)?;
                }
                writeln!(out, "        let {}_len = {}.len();", &name, &name)?;
                writeln!(out, "        let {}_ptr = {}.as_ptr();", &name, &name)?;
            }
            for f in params.iter() {
                if let StructField::ValueParam { list_name, .. } = f {
                    let list_sym = symbol(&list_name);
                    writeln!(
                        out,
                        "        let mut {}_copy = {}.to_vec();",
                        &list_name, &list_sym
                    )?;
                    writeln!(
                        out,
                        "        let ({}_mask, {}_vec) = base::pack_bitfield(&mut {}_copy);",
                        &list_name, &list_name, &list_name
                    )?;
                    writeln!(
                        out,
                        "        let {}_ptr = {}_vec.as_ptr();",
                        &list_name, &list_name
                    )?;
                }
            }
            writeln!(out, "        let cookie = {}(", &ffi_fn_name)?;
            writeln!(out, "            c.get_raw_conn(),")?;
        }

        // FFI request arguments
        for f in params.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    if skip_fields.contains(&name) {
                        continue;
                    }
                    let mut name = symbol(&name).to_string();
                    let ffi_typ = self.ffi_use_type_name(&typ);

                    if let Some(lf) = list_fields.iter().find(|lf| lf.lenfield == name) {
                        name = lf.name.clone() + "_len";
                    }
                    let out = &mut self.rs_buf;
                    writeln!(out, "            {} as {},", &name, &ffi_typ)?;
                }
                StructField::List { name, typ, .. } => {
                    if send_event && name == "event" {
                        let out = &mut self.rs_buf;
                        writeln!(out, "            event_ptr")?;
                    } else {
                        let name = symbol(&name);
                        let ffi_typ = self.ffi_use_type_name(&typ);
                        let out = &mut self.rs_buf;
                        writeln!(out, "            {}_ptr as *const {},", &name, &ffi_typ)?;
                    }
                }
                StructField::ListNoLen { name, typ } => {
                    let name = symbol(&name);
                    let ffi_typ = self.ffi_use_type_name(&typ);
                    let out = &mut self.rs_buf;
                    writeln!(out, "            {}_len as u32,", &name)?;
                    writeln!(out, "            {}_ptr as *const {},", &name, &ffi_typ)?;
                }
                StructField::ValueParam {
                    list_name,
                    mask_typ,
                    ..
                } => {
                    let typ = rust_field_type_name(&self.xcb_mod, &mask_typ);
                    let out = &mut self.rs_buf;
                    writeln!(out, "            {}_mask as {},", &list_name, &typ)?;
                    writeln!(out, "             {}_ptr as *const u32,", &list_name)?;
                }
                _ => {}
            }
        }

        let out = &mut self.rs_buf;
        writeln!(out, "        );")?;
        writeln!(out, "        {} {{", cookie_name)?;
        writeln!(out, "            cookie: cookie,")?;
        writeln!(out, "            conn: c,")?;
        writeln!(
            out,
            "            checked: {},",
            if checked { "true" } else { "false" }
        )?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }

    fn emit_rs_reply(
        &mut self,
        req_name: &str,
        ffi_cookie: &str,
        ffi_reply_fn: &str,
        ffi_reply_typ: &str,
        rs_cookie: &str,
        reply: Reply,
    ) -> io::Result<()> {
        let rs_reply = req_name.to_string() + "Reply";

        {
            let out = &mut self.rs_buf;
            writeln!(out)?;
            writeln!(out, "impl base::CookieSeq for {} {{", &ffi_cookie)?;
            writeln!(out, "    fn sequence(&self) -> c_uint {{")?;
            writeln!(out, "        self.sequence")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;

            writeln!(out)?;
            writeln!(
                out,
                "pub type {}<'a> = base::Cookie<'a, {}>;",
                &rs_cookie, &ffi_cookie
            )?;
            writeln!(out)?;

            writeln!(out, "impl<'a> {}<'a> {{", &rs_cookie)?;
            writeln!(
                out,
                "    pub fn get_reply(self) -> Result<{}, base::ReplyError> {{",
                &rs_reply
            )?;
            writeln!(
                out,
                "        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();"
            )?;
            writeln!(
            out,
            "        let err_ptr = if self.checked {{ &mut err }} else {{ std::ptr::null_mut() }};"
        )?;
            writeln!(out, "        let reply = unsafe {{")?;
            writeln!(out, "            {} {{", &rs_reply)?;
            writeln!(out, "                ptr: {}(", &ffi_reply_fn)?;
            writeln!(out, "                    self.conn.get_raw_conn(),")?;
            writeln!(out, "                    self.cookie,")?;
            writeln!(out, "                    err_ptr,")?;
            writeln!(out, "                ),")?;
            writeln!(out, "            }}")?;
            writeln!(out, "        }};")?;
            writeln!(out, "        let checked = self.checked;")?;
            writeln!(out, "        std::mem::forget(self);")?;
            writeln!(out)?;
            writeln!(
                out,
                "        match (reply.ptr.is_null(), err.is_null(), checked) {{"
            )?;
            writeln!(out, "            (false, _, false) => Ok(reply),")?;
            writeln!(out, "            (false, true, true) => Ok(reply),")?;
            writeln!(out, "            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError {{ ptr: err }})),")?;
            writeln!(
                out,
                "            (true, true, _) => Err(base::ReplyError::NullResponse),"
            )?;
            writeln!(
                out,
                "            (r, e, c) => unreachable!(\"{{:?}}\", (r, e, c)),"
            )?;
            writeln!(out, "        }}")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;

            writeln!(out)?;
            writeln!(
                out,
                "pub type {} = base::Reply<{}>;",
                &rs_reply, &ffi_reply_typ
            )?;
            writeln!(out)?;
            writeln!(out, "impl {} {{", &rs_reply)?;
        }

        let stru = Struct {
            name: req_name.into(),
            fields: reply.fields,
            doc: reply.doc,
        };

        self.emit_rs_struct_field_accessors(&stru, "(*self.ptr)", &[], false)?;

        {
            let out = &mut self.rs_buf;
            writeln!(out, "}}")?;
        }

        Ok(())
    }

    fn emit_xid(&mut self, name: String) -> io::Result<()> {
        self.notify_typ(name.clone());
        let ffi_typ = self.ffi_decl_type_name(&name);
        emit_type_alias(&mut self.ffi, &ffi_typ, "u32")?;
        self.emit_ffi_iterator(&name, &ffi_typ, false)?;

        let rs_typ = rust_type_name(&name);
        emit_type_alias(&mut self.rs, &rs_typ, &ffi_typ)?;

        self.reg_type(name, ffi_typ, rs_typ, Some(4), false, true, true);
        Ok(())
    }

    fn emit_struct(&mut self, stru: Struct) -> io::Result<()> {
        self.notify_typ(stru.name.clone());

        let has_lifetime = fields_need_lifetime(&stru.fields);
        if has_lifetime {
            self.typ_with_lifetime.insert(stru.name.clone());
        }
        let ffi_typ = self.emit_ffi_struct(&stru, false, false)?;
        self.emit_ffi_field_list_accessors(&ffi_typ, &stru.name, &stru.fields)?;
        let ffi_it_typ = self.emit_ffi_iterator(&stru.name, &ffi_typ, has_lifetime)?;

        let rs_typ = self.emit_rs_struct(&ffi_typ, &stru, has_lifetime)?;
        self.emit_rs_iterator(&stru.name, &rs_typ, &ffi_it_typ, has_lifetime, false)?;

        let stru_sz = self.compute_ffi_struct_size(&stru);
        self.reg_type(
            stru.name,
            ffi_typ,
            rs_typ,
            stru_sz,
            false,
            false,
            self.fields_are_pod(&stru.fields),
        );

        Ok(())
    }

    fn emit_union(&mut self, stru: Struct) -> io::Result<()> {
        self.notify_typ(stru.name.clone());

        let ffi_sz = self.compute_ffi_union_size(&stru);
        let ffi_typ = self.ffi_decl_type_name(&stru.name);

        {
            let out = &mut self.ffi;
            writeln!(out)?;
            emit_doc_text(out, &stru.doc)?;
            writeln!(out, "// union")?;
            writeln!(out, "#[repr(C)]")?;
            writeln!(out, "#[derive(Debug)]")?;
            writeln!(out, "pub struct {} {{", &ffi_typ)?;
            writeln!(out, "    pub data: [u8; {}],", ffi_sz)?;
            writeln!(out, "}}")?;
            emit_copy_clone(out, &ffi_typ)?;
        }

        self.emit_ffi_iterator(&stru.name, &ffi_typ, false)?;

        let rs_typ = rust_type_name(&stru.name);

        {
            let out = &mut self.rs_buf;

            writeln!(out)?;
            emit_doc_text(out, &stru.doc)?;
            emit_type_alias(out, &rs_typ, &ffi_typ)?;
        }

        self.emit_rs_union_impl(&rs_typ, ffi_sz, &stru)?;

        let ffi_it_typ = self.ffi_iterator_name(&stru.name);

        self.emit_rs_iterator(&stru.name, &rs_typ, &ffi_it_typ, false, true)?;

        self.reg_type(stru.name, ffi_typ, rs_typ, Some(ffi_sz), true, false, false);

        Ok(())
    }

    fn emit_error(&mut self, number: i32, stru: Struct) -> io::Result<()> {
        self.notify_typ(stru.name.clone() + "Error");

        emit_ffi_opcode(&mut self.ffi, &self.xcb_mod_prefix, &stru.name, number)?;

        emit_rs_opcode(&mut self.rs_buf, &stru.name, number)?;

        let fields = {
            let mut fields = vec![
                StructField::Field {
                    name: "response_type".into(),
                    typ: "CARD8".into(),
                    enu: None,
                },
                StructField::Field {
                    name: "error_code".into(),
                    typ: "CARD8".into(),
                    enu: None,
                },
                StructField::Field {
                    name: "sequence".into(),
                    typ: "CARD16".into(),
                    enu: None,
                },
            ];
            for f in stru.fields.into_iter() {
                fields.push(f);
            }
            fields
        };
        let stru = Struct {
            name: stru.name + "Error",
            fields,
            doc: stru.doc,
        };

        let ffi_typ = self.emit_ffi_struct(&stru, false, false)?;

        let rs_typ = rust_type_name(&stru.name);

        emit_rs_error(&mut self.rs, &ffi_typ, &rs_typ)?;

        self.reg_type(
            stru.name,
            ffi_typ,
            rs_typ,
            None,
            false,
            false,
            self.fields_are_pod(&stru.fields),
        );

        Ok(())
    }

    fn emit_error_copy(&mut self, name: &str, number: i32, error_ref: &str) -> io::Result<()> {
        emit_ffi_opcode(&mut self.ffi, &self.xcb_mod_prefix, &name, number)?;
        let new_name = name.to_owned() + "Error";
        let old_name = error_ref.to_owned() + "Error";

        let new_ffi_typ = self.ffi_decl_type_name(&new_name);
        let old_ffi_typ = self.ffi_use_type_name(&old_name);

        emit_type_alias(&mut self.ffi, &new_ffi_typ, &old_ffi_typ)?;

        let rs_typ = rust_type_name(&new_name);

        emit_rs_error(&mut self.rs, &new_ffi_typ, &rs_typ)?;
        emit_rs_opcode(&mut self.rs_buf, &name, number)?;

        self.notify_typ(new_name);

        Ok(())
    }

    fn emit_event(
        &mut self,
        number: i32,
        stru: Struct,
        no_seq_number: bool,
        xge: bool,
    ) -> io::Result<()> {
        let Struct {
            name: orig_name,
            fields: mut orig_fields,
            doc,
        } = stru;
        let event_typ = orig_name.clone() + "Event";
        self.notify_typ(event_typ.clone());

        emit_ffi_opcode(&mut self.ffi, &self.xcb_mod_prefix, &orig_name, number)?;

        let opcopies = self
            .evcopies
            .remove(&orig_name)
            .expect("missing event copies");

        let (fields, must_pack) = {
            let mut fields = vec![StructField::Field {
                name: "response_type".into(),
                typ: "CARD8".into(),
                enu: None,
            }];
            let mut must_pack = false;

            let mut sz = 1; // response_type size

            if xge {
                fields.push(StructField::Field {
                    name: "extension".into(),
                    typ: "CARD8".into(),
                    enu: None,
                });
                fields.push(StructField::Field {
                    name: "sequence".into(),
                    typ: "CARD16".into(),
                    enu: None,
                });
                fields.push(StructField::Field {
                    name: "length".into(),
                    typ: "CARD32".into(),
                    enu: None,
                });
                fields.push(StructField::Field {
                    name: "event_type".into(),
                    typ: "CARD16".into(),
                    enu: None,
                });
                sz += 9;
            } else if !no_seq_number {
                fields.push(orig_fields.remove(0));
                fields.push(StructField::Field {
                    name: "sequence".into(),
                    typ: "CARD16".into(),
                    enu: None,
                });
            }

            for f in orig_fields.into_iter() {
                if xge {
                    let fsz = self.compute_ffi_struct_field_sizeof(&f);
                    fields.push(f);
                    if sz < 32 {
                        sz += fsz.expect("can't compute ffi full_sequence position");
                        if sz == 32 {
                            fields.push(StructField::Field {
                                name: "full_sequence".into(),
                                typ: "CARD32".into(),
                                enu: None,
                            });
                        }
                    } else if let Some(fsz) = fsz {
                        if fsz == 8 {
                            must_pack = true;
                        }
                    }
                } else {
                    fields.push(f);
                }
            }
            (fields, must_pack)
        };
        let stru = Struct {
            name: event_typ,
            fields,
            doc,
        };

        let ffi_typ = self.emit_ffi_struct(&stru, must_pack, false)?;
        let ffi_sz = self.compute_ffi_struct_size(&stru);

        for c in opcopies.iter() {
            emit_ffi_opcode(&mut self.ffi, &self.xcb_mod_prefix, &c.name, c.number)?;
            let new_name = c.name.to_owned() + "Event";

            let new_ffi_typ = self.ffi_decl_type_name(&new_name);
            let old_ffi_typ = self.ffi_use_type_name(&stru.name);

            emit_type_alias(&mut self.ffi, &new_ffi_typ, &old_ffi_typ)?;
        }

        let rs_typ = self.emit_rs_event(&orig_name, number, &stru, &ffi_typ, &opcopies, xge)?;

        self.reg_type(
            stru.name,
            ffi_typ,
            rs_typ,
            ffi_sz,
            false,
            false,
            self.fields_are_pod(&stru.fields),
        );

        Ok(())
    }

    fn emit_request(&mut self, mut req: Request) -> io::Result<()> {
        let request_typ = req.name.clone() + "Request";
        self.notify_typ(request_typ.clone());

        let fields = {
            let mut params = req.params.clone();
            let mut fields = vec![
                StructField::Field {
                    name: "major_opcode".into(),
                    typ: "CARD8".into(),
                    enu: None,
                },
                if self.xcb_mod == "xproto" {
                    if params.is_empty() {
                        StructField::Pad(1)
                    } else {
                        let p = params.remove(0);
                        // assert!(self.compute_ffi_struct_field_sizeof(&p) == Some(1usize));
                        p
                    }
                } else {
                    StructField::Field {
                        name: "minor_opcode".into(),
                        typ: "CARD8".into(),
                        enu: None,
                    }
                },
                StructField::Field {
                    name: "length".into(),
                    typ: "CARD16".into(),
                    enu: None,
                },
            ];
            fields.append(&mut params);
            fields
        };

        for f in &fields {
            if let StructField::Switch(name, expr, cases) = f {
                self.emit_ffi_switch_struct(&req.name, &name, &expr, &cases)?;
            }
        }

        emit_ffi_opcode(&mut self.ffi, &self.xcb_mod_prefix, &req.name, req.opcode)?;

        let stru = Struct {
            name: request_typ,
            fields,
            doc: req.doc.clone(),
        };
        self.emit_ffi_struct(&stru, false, false)?;

        let void = req.reply.is_none();
        let (ffi_cookie, check_name, checked) = if void {
            ("VoidCookie".to_string(), req.name.clone() + "Checked", true)
        } else {
            (
                req.name.clone() + "Cookie",
                req.name.clone() + "Unchecked",
                false,
            )
        };

        let rs_cookie = if ffi_cookie == "VoidCookie" {
            "base::VoidCookie"
        } else {
            &ffi_cookie
        };
        emit_rs_opcode(&mut self.rs_buf, &req.name, req.opcode)?;

        if let Some(reply) = req.reply.take() {
            let (ffi_cookie, ffi_reply_fn, ffi_reply_typ) =
                self.emit_ffi_reply(&req.name, reply.clone())?;
            self.emit_rs_reply(
                &req.name,
                &ffi_cookie,
                &ffi_reply_fn,
                &ffi_reply_typ,
                &rs_cookie,
                reply,
            )?;
            self.notify_typ(req.name.clone() + "Reply");
            self.notify_typ(req.name.clone() + "Cookie");
        }

        let ffi_fn_name = ffi_request_fn_name(&self.xcb_mod_prefix, &req.name);
        let ffi_check_fn_name = ffi_request_fn_name(&self.xcb_mod_prefix, &check_name);

        self.emit_ffi_req_fn(&req.name, &ffi_fn_name, &ffi_cookie, &req.params, &stru.doc)?;
        self.emit_ffi_req_fn(&req.name, &ffi_check_fn_name, &ffi_cookie, &req.params, &stru.doc)?;
        self.emit_rs_req_fn(&req.name, &rs_cookie, &req.params, &stru.doc, !checked)?;
        self.emit_rs_req_fn(&check_name, &rs_cookie, &req.params, &stru.doc, checked)?;

        Ok(())
    }
}

fn fields_need_lifetime(fields: &[StructField]) -> bool {
    for f in fields.iter() {
        match f {
            StructField::List { .. } => {
                return true;
            }
            _ => {}
        }
    }
    false
}

fn has_fd(fields: &[StructField]) -> bool {
    for f in fields.iter() {
        if let StructField::Fd(_) = f {
            return true;
        }
    }
    false
}

fn make_field(name: String, typ: String) -> StructField {
    StructField::Field {
        name,
        typ,
        enu: None,
    }
}

fn expr_fixed_length(expr: &Expr<usize>) -> Option<usize> {
    match expr {
        Expr::EnumRef { .. } => None, // FIXME: get the value of the enum item
        Expr::Value(val) => Some(*val),
        Expr::Popcount(ex) => expr_fixed_length(&ex).map(|sz| sz.count_ones() as _),
        Expr::Op(op, lhs, rhs) => match (expr_fixed_length(lhs), expr_fixed_length(rhs)) {
            (Some(lhs), Some(rhs)) => match op.as_str() {
                "+" => Some(lhs + rhs),
                "-" => Some(lhs - rhs),
                "*" => Some(lhs * rhs),
                "/" => Some(lhs / rhs),
                _ => panic!("Unexpected binary operator in Expr: {}", op),
            },
            _ => None,
        },
        Expr::Unop(op, val) => expr_fixed_length(val).map(|val| match op.as_str() {
            "~" => !val,
            _ => panic!("Unexpected unary operator in Expr: {}", op),
        }),
        _ => None,
    }
}

fn capitalize(s: &str) -> String {
    let mut ch = s.chars();
    match ch.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(ch).collect()
    }
}

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
            if prev != '_' {
                res.push('_');
            }
        }
        res.push(c);
        prev = c;
        c = next;
    }
    if is_low(prev) && is_high(c) && prev != '_' {
        res.push('_');
    }
    res.push(c);

    res
}

/// capitalize each substring beginning by uppercase
/// said otherwise: every upper preceded by another upper and followed by a upper is turned to lower
/// assert!(tit_cap("SomeString") == "SomeString")
/// assert!(tit_cap("WINDOW") == "Window")
/// assert!(tit_cap("GContext") == "GContext")
/// assert!(tit_cap("IDChoice") == "IdChoice")
fn tit_cap(name: &str) -> String {
    if name.len() <= 1 {
        return name.into();
    }

    let is_high = |c: char| c.is_ascii_uppercase() | c.is_ascii_digit();

    let mut res = String::new();
    let mut ch = name.chars();
    let mut prev = ch.next().unwrap();
    res.push(prev);
    let mut c = ch.next().unwrap();

    for next in ch {
        if is_high(c) && is_high(prev) && is_high(next) {
            res.push(c.to_ascii_lowercase())
        } else {
            res.push(c)
        }
        prev = c;
        c = next;
    }
    if is_high(c) && is_high(prev) {
        res.push(c.to_ascii_lowercase());
    } else {
        res.push(c);
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

fn qualified_name(xcb_mod: &str, module: &Option<&str>, name: &str) -> String {
    if let Some(module) = module {
        if module != &xcb_mod {
            format!("{}::{}", module, &name)
        } else {
            name.into()
        }
    } else {
        name.into()
    }
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
        tit_split(field)
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
        tit_split(field)
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
        tit_split(field)
    )
}

fn ffi_field_list_iterator_it_fn_name(xcb_mod_prefix: &str, typ_name: &str, field: &str) -> String {
    format!(
        "xcb_{}{}_{}_iterator",
        &xcb_mod_prefix,
        tit_split(typ_name).to_ascii_lowercase(),
        tit_split(field)
    )
}

fn ffi_switch_struct_name(xcb_mod_prefix: &str, req_name: &str, switch_name: &str) -> String {
    format!(
        "xcb_{}{}_{}_t",
        &xcb_mod_prefix,
        tit_split(req_name).to_ascii_lowercase(),
        tit_split(switch_name).to_ascii_lowercase()
    )
}

fn ffi_request_fn_name(xcb_mod_prefix: &str, req_name: &str) -> String {
    format!(
        "xcb_{}{}",
        &xcb_mod_prefix,
        tit_split(req_name).to_ascii_lowercase(),
    )
}

fn ffi_reply_fn_name(xcb_mod_prefix: &str, req_name: &str) -> String {
    format!(
        "xcb_{}{}_reply",
        &xcb_mod_prefix,
        tit_split(req_name).to_ascii_lowercase(),
    )
}

fn ffi_reply_fds_fn_name(xcb_mod_prefix: &str, req_name: &str) -> String {
    format!(
        "xcb_{}{}_reply_fds",
        &xcb_mod_prefix,
        tit_split(req_name).to_ascii_lowercase(),
    )
}

fn ffi_opcode_name(xcb_mod_prefix: &str, name: &str) -> String {
    format!(
        "XCB_{}{}",
        xcb_mod_prefix.to_ascii_uppercase(),
        tit_split(&name).to_ascii_uppercase()
    )
}

fn rust_type_name(typ: &str) -> String {
    match typ {
        "CARD8" => "u8".into(),
        "CARD16" => "u16".into(),
        "CARD32" => "u32".into(),
        "CARD64" => "u64".into(),
        "INT8" => "i8".into(),
        "INT16" => "i16".into(),
        "INT32" => "i32".into(),
        "BYTE" => "u8".into(),
        "BOOL" => "bool".into(),
        "float" => "f32".into(),
        "double" => "f64".into(),
        typ => tit_cap(typ),
    }
}

fn rust_iterator_type_name(typ: &str) -> String {
    format!("{}Iterator", tit_cap(&typ))
}

/// same as rust_type_name but can also have a namespace before (with a single colon)
fn rust_field_type_name(xcb_mod: &str, typ: &str) -> String {
    let (module, typ) = extract_module(&typ);

    let typ = rust_type_name(&typ);

    qualified_name(&xcb_mod, &module, &typ)
}

fn rust_field_doc_name(f: &StructField) -> Option<&str> {
    match f {
        StructField::Field { name, .. } => Some(symbol(name)),
        StructField::List { name, .. } => Some(symbol(name)),
        StructField::ListNoLen { name, .. } => Some(symbol(name)),
        StructField::Fd(name) => Some(symbol(name)),
        StructField::ValueParam { list_name, .. } => Some(symbol(list_name)),
        _ => None,
    }
}

fn rust_enum_item_name(name: &str, item: &str) -> String {
    format!("{}_{}", tit_split(name), tit_split(item)).to_ascii_uppercase()
}

fn rust_opname(name: &str) -> String {
    tit_split(&name).to_ascii_uppercase()
}

fn rust_request_fn_name(name: &str) -> String {
    let fn_name = tit_split(&name).to_ascii_lowercase();
    match fn_name.as_str() {
        "await" => "await_".into(),
        _ => fn_name,
    }
}

fn emit_type_alias<Out: Write>(out: &mut Out, new_typ: &str, old_typ: &str) -> io::Result<()> {
    writeln!(out)?;
    writeln!(out, "pub type {} = {};", new_typ, old_typ)?;
    Ok(())
}

fn emit_doc_str<Out: Write>(out: &mut Out, docstr: &str, indent: &str) -> io::Result<()> {
    let mut wrote = false;
    if !docstr.is_empty() {
        for s in docstr.split('\n') {
            let s = s.trim_end();
            if !s.is_empty() {
                writeln!(out, "///{}{}", indent, s.trim_end())?;
            } else {
                writeln!(out, "///")?;
            }
            wrote = true;
        }
    }
    if !wrote {
        writeln!(out, "///")
    } else {
        Ok(())
    }
}

fn emit_doc_text<Out: Write>(out: &mut Out, doc: &Option<Doc>) -> io::Result<()> {
    if let Some(doc) = doc {
        if !doc.brief.is_empty() {
            emit_doc_str(out, &doc.brief, " ")?;
        }
        if !doc.brief.is_empty() && !doc.text.is_empty() {
            writeln!(out, "///")?;
        }
        if !doc.text.is_empty() {
            emit_doc_str(out, &doc.text, " ")?;
        }
    }
    Ok(())
}

fn emit_doc_field<Out: Write>(out: &mut Out, doc: &Option<Doc>, field: &str) -> io::Result<()> {
    if let Some(doc) = doc {
        if let Some(f) = doc.fields.iter().find(|f| f.name == field) {
            emit_doc_str(out, &f.text, " ")
        } else {
            //writeln!(out, "///")
            Ok(())
        }
    } else {
        Ok(())
    }
}

fn emit_doc_field_indent<Out: Write>(
    out: &mut Out,
    doc: &Option<Doc>,
    field: &str,
    indent: &str,
) -> io::Result<()> {
    if let Some(doc) = doc {
        if let Some(f) = doc.fields.iter().find(|f| f.name == field) {
            emit_doc_str(out, &f.text, indent)?;
        }
    }
    Ok(())
}

fn emit_copy_clone<Out: Write>(out: &mut Out, typ: &str) -> io::Result<()> {
    writeln!(out)?;
    writeln!(out, "impl Copy for {} {{}}", &typ)?;
    writeln!(out, "impl Clone for {} {{", &typ)?;
    writeln!(out, "    fn clone(&self) -> {} {{ *self }}", &typ)?;
    writeln!(out, "}}")?;

    Ok(())
}

fn emit_ffi_opcode<Out: Write>(
    out: &mut Out,
    xcb_mod_prefix: &str,
    name: &str,
    num: i32,
) -> io::Result<()> {
    let op_name = ffi_opcode_name(&xcb_mod_prefix, &name);
    let num_typ = if num < 0 { "i8" } else { "u8" };
    writeln!(out)?;
    writeln!(out, "pub const {}: {} = {};", &op_name, &num_typ, num)?;
    Ok(())
}

fn emit_rs_opcode<Out: Write>(out: &mut Out, name: &str, opcode: i32) -> io::Result<()> {
    let opname = rust_opname(&name);
    let num_typ = if opcode < 0 { "i8" } else { "u8" };

    writeln!(out)?;
    writeln!(out, "pub const {}: {} = {};", &opname, &num_typ, opcode)?;

    Ok(())
}

fn emit_rs_error<Out: Write>(out: &mut Out, ffi_typ: &str, rs_typ: &str) -> io::Result<()> {
    writeln!(out)?;
    writeln!(out, "pub struct {} {{", &rs_typ)?;
    writeln!(out, "    pub base: base::Error<{}>,", &ffi_typ)?;
    writeln!(out, "}}")?;

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

struct ListField {
    name: String,
    typ: String,
    lenfield: String,
    // lenfield_typ: String,
}

impl ListField {
    fn fetch_from(fields: &[StructField]) -> Vec<ListField> {
        let mut res = Vec::new();
        for f in fields {
            match f {
                StructField::List {
                    name,
                    typ,
                    len_expr,
                } => {
                    let lenfield = expr_lenfield(&len_expr);
                    if let Some(lenfield) = lenfield {
                        let name = name.clone();
                        let typ = typ.clone();
                        let lenfield = lenfield.to_string();
                        // let lenfield_typ = fields
                        //     .iter()
                        //     .find_map(|f| match f {
                        //         StructField::Field { name, typ, .. } => {
                        //             if name == &lenfield {
                        //                 Some(typ.clone())
                        //             } else {
                        //                 None
                        //             }
                        //         }
                        //         _ => None,
                        //     })
                        //     .expect("can't find lenfield type");
                        res.push(ListField {
                            name,
                            typ,
                            lenfield,
                            // lenfield_typ,
                        });
                    }
                }
                StructField::ListNoLen { name, typ } => {
                    let name = name.clone();
                    let typ = typ.clone();
                    let lenfield = name.clone() + "_len";
                    res.push(ListField {
                        name,
                        typ,
                        lenfield,
                        // lenfield_typ: "CARD32".to_string(),
                    });
                }
                _ => {}
            }
        }
        res
    }
}

fn expr_lenfield(expr: &Expr<usize>) -> Option<&str> {
    match expr {
        Expr::FieldRef(name) => Some(name),
        Expr::Op(_, lhs, rhs) => expr_lenfield(&lhs).or_else(|| expr_lenfield(&rhs)),
        Expr::Unop(_, rhs) => expr_lenfield(&rhs),
        _ => None,
    }
}

fn request_has_template(params: &[StructField]) -> bool {
    for f in params.iter() {
        if let StructField::List { typ, .. } = f {
            if typ == "void" {
                return true;
            }
        }
    }
    false
}
