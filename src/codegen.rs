use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, Cursor, Write};

use crate::ast::{
    Doc, EnumItem, Event, Expr, OpCopy, OpCopyMap, Reply, Request, Struct, StructField,
};
use crate::output::Output;

#[derive(Clone, Debug)]
pub struct DepInfo {
    pub xcb_mod: String,
    pub typ_with_lifetime: HashSet<String>,
    pub ffi_type_sizes: HashMap<String, Option<usize>>,
    pub ffi_typ_reg: HashSet<String>,
    pub rs_typ_reg: HashSet<String>,
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
            typ_with_lifetime: HashSet::new(),
            typ_unions: HashSet::new(),
            typ_simple: HashSet::new(),
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
            typ_with_lifetime: self.typ_with_lifetime,
            ffi_type_sizes: self.ffi_type_sizes,
            ffi_typ_reg: self.ffi_typ_reg,
            rs_typ_reg: self.rs_typ_reg,
        }
    }

    pub fn prologue(&mut self, imports: &Vec<String>) -> io::Result<()> {
        let out = &mut self.ffi;
        // Adding a comment only to fit the python generated code and pass initial tests
        writeln!(
            out,
            "// Generated automatically from {}.xml by rs_client.py version 0.9.0.",
            &self.xcb_mod
        )?;
        writeln!(out, "// Do not edit!")?;
        writeln!(out, "")?;
        writeln!(out, "use ffi::base::*;")?;
        for imp in imports.iter() {
            writeln!(out, "use ffi::{}::*;", imp)?;
        }
        writeln!(out, "use libc::{{c_char, c_int, c_uint, c_void}};")?;
        writeln!(out, "use std;")?;
        writeln!(out)?;

        if self.xcb_mod != "xproto" {
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(out, "pub static mut {}: xcb_extension_t;", &self.xcb_mod)?;
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
            writeln!(out, "        &mut {}", &self.xcb_mod)?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
        }

        Ok(())
    }

    pub fn epilogue(&mut self) -> io::Result<()> {
        let linklib = match self.xcb_mod.as_str() {
            "xproto" | "big_requests" | "xc_misc" => "xcb".to_owned(),
            "genericevent" => "xcb-ge".to_owned(),
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
                let ffi_old_typ = ffi_type_name(&self.xcb_mod_prefix, &oldname);
                let ffi_new_typ = ffi_type_name(&self.xcb_mod_prefix, &newname);

                emit_type_alias(&mut self.ffi, &ffi_new_typ, &ffi_old_typ)?;
                self.emit_ffi_iterator(&newname, &ffi_new_typ, false)?;

                let rs_new_typ = rust_type_name(&newname);
                emit_type_alias(&mut self.rs, &rs_new_typ, &ffi_new_typ)?;

                let is_simple = self.typ_is_simple(&oldname);

                self.reg_type(
                    newname,
                    ffi_new_typ,
                    rs_new_typ,
                    self.ffi_type_sizeof(&oldname),
                    false,
                    is_simple,
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
                self.reg_type(en.name, ffi_typ, rs_typ, Some(4), false, true);
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
    ) {
        self.ffi_typ_reg.insert(ffi_typ);
        self.rs_typ_reg.insert(rs_typ);
        if is_union {
            self.typ_unions.insert(typ.clone());
        }
        if is_simple {
            self.typ_simple.insert(typ.clone());
        }
        self.ffi_type_sizes.insert(typ, ffi_sz);
    }

    fn has_ffi_type(&mut self, typ: &str) -> bool {
        self.ffi_typ_reg.contains(typ)
    }

    fn has_rs_type(&mut self, typ: &str) -> bool {
        self.rs_typ_reg.contains(typ)
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
            "void" => true,
            typ => self.typ_simple.contains(typ),
        }
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
                StructField::Switch => {
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

    fn type_has_lifetime(&self, stru: &Struct) -> bool {
        has_variable_list(&stru.fields)
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

                    let is_simple = self.typ_is_simple(&typ);

                    let accessor_needed = fixed_size;
                    let length_needed = true;
                    let end_needed = is_simple;
                    let iterator_needed = !is_simple;

                    let has_lifetime = self.typ_with_lifetime.contains(typ);

                    if accessor_needed {
                        let f_typ = ffi_type_name(&self.xcb_mod_prefix, typ);
                        let acc_fn = ffi_field_list_iterator_acc_fn_name(
                            &self.xcb_mod_prefix,
                            &xcb_name,
                            &name,
                        );
                        let out = &mut self.ffi_buf;
                        writeln!(out)?;
                        writeln!(
                            out,
                            "pub fn {}(R: *const {}) -> *mut {};",
                            &acc_fn, &ffi_typ, &f_typ
                        )?;
                    }

                    if length_needed {
                        let len_fn = ffi_field_list_iterator_len_fn_name(
                            &self.xcb_mod_prefix,
                            &xcb_name,
                            &name,
                        );
                        let out = &mut self.ffi_buf;
                        writeln!(out)?;
                        writeln!(out, "pub fn {}(R: *const {}) -> c_int;", &len_fn, &ffi_typ)?;
                    }

                    if end_needed {
                        let end_fn = ffi_field_list_iterator_end_fn_name(
                            &self.xcb_mod_prefix,
                            &xcb_name,
                            &name,
                        );
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
                        let it_fn = ffi_field_list_iterator_it_fn_name(
                            &self.xcb_mod_prefix,
                            &xcb_name,
                            &name,
                        );
                        let it_typ = ffi_iterator_name(&self.xcb_mod_prefix, &typ);
                        let out = &mut self.ffi_buf;
                        writeln!(out)?;
                        writeln!(
                            out,
                            "pub fn {}{}(R: *const {}) -> {}{};",
                            &it_fn, &lifetime, &ffi_typ, &it_typ, &lifetime
                        )?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn emit_ffi_struct(&mut self, stru: &Struct) -> io::Result<String> {
        let Struct { name, fields, doc } = &stru;

        let ffi_typ = ffi_type_name(&self.xcb_mod_prefix, &name);
        let impl_copy_clone = self.eligible_to_copy(&stru);

        let out = &mut self.ffi;
        writeln!(out)?;
        emit_doc_text(out, &doc)?;
        writeln!(out, "#[repr(C)]")?;
        writeln!(out, "pub struct {} {{", &ffi_typ)?;

        let mut padnum = 0;

        // cases of ValueParam were the mask is declared as a field in the struct
        // before the actual ValueParam field. We keep track of all fields written
        // to avoid doubles
        let mut written_fields = Vec::new();

        for f in fields.iter() {
            match f {
                StructField::Field { name, typ, .. } | StructField::Expr { name, typ, .. } => {
                    emit_doc_field(out, &doc, &name)?;
                    writeln!(
                        out,
                        "    pub {}: {},",
                        symbol(&name),
                        ffi_field_type_name(&self.xcb_mod, &self.xcb_mod_prefix, &typ)
                    )?;
                    written_fields.push(name.as_str());
                }
                StructField::Pad(sz) => {
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
                        emit_doc_field(out, &doc, &name)?;
                        writeln!(
                            out,
                            "    pub {}: [{}; {}],",
                            symbol(&name),
                            ffi_field_type_name(&self.xcb_mod, &self.xcb_mod_prefix, &typ),
                            sz
                        )?;
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
                    emit_doc_field(out, &doc, &mask_name)?;
                    writeln!(
                        out,
                        "    pub {}: {},",
                        symbol(&mask_name),
                        ffi_field_type_name(&self.xcb_mod, &self.xcb_mod_prefix, &mask_typ)
                    )?;
                }
                _ => {}
            }
        }
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
        let out = &mut self.rs_buf;
        for f in stru.fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    if skip_fields.iter().find(|skip| skip == &name).is_some() {
                        continue;
                    }
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
                    let is_string = typ == "char";
                    let is_byte_slice = typ == "BYTE" || typ == "CARD8";
                    let fixed_len = expr_fixed_length(len_expr);

                    if is_byte_slice && fixed_len.is_some() {
                        writeln!(out, "    pub fn {}(&self) -> &[u8] {{", &name)?;
                        writeln!(out, "        unsafe {{ &(*self.ptr).{} }}", &name)?;
                        writeln!(out, "    }}")?;
                    } else if is_string || is_byte_slice {
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
                        let ret = if is_string { "&str" } else { "&[u8]" };
                        writeln!(out, "    pub fn {}(&self) -> {} {{", &name, &ret)?;
                        writeln!(out, "        unsafe {{")?;
                        writeln!(out, "            let field = self.ptr;")?;
                        writeln!(out, "            let len = {}(field) as usize;", &len_fn)?;
                        writeln!(out, "            let data = {}(field);", &acc_fn)?;
                        if is_string {
                            writeln!(out, "            let slice = std::slice::from_raw_parts(data as *const u8, len);")?;
                            writeln!(out, "            // should we check what comes from X?")?;
                            writeln!(out, "            std::str::from_utf8_unchecked(&slice)")?;
                        } else {
                            writeln!(out, "            std::slice::from_raw_parts(data, len)")?;
                        }
                        writeln!(out, "        }}")?;
                        writeln!(out, "    }}")?;
                    } else {
                        let has_lifetime = self.typ_with_lifetime.contains(typ);
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
            let ffi_typ = ffi_type_name(&self.xcb_mod_prefix, &name);

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

    fn emit_ffi_req_fn(
        &mut self,
        req_name: &str,
        cookie_name: &str,
        fields: &[StructField],
        doc: &Option<Doc>,
    ) -> io::Result<()> {
        let fn_name = ffi_request_fn_name(&self.xcb_mod_prefix, &req_name);
        let cookie_typ = ffi_type_name(&self.xcb_mod_prefix, &cookie_name);
        let out = &mut self.ffi_buf;
        writeln!(out)?;
        emit_doc_text(out, doc)?;
        writeln!(out, "    pub fn {} (", &fn_name)?;
        writeln!(out, "        c: *mut xcb_connection_t,")?;
        let mut written_fields = Vec::new();
        for f in fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    written_fields.push(name);
                    let name = symbol(&name);
                    let typ = ffi_field_type_name(&self.xcb_mod, &self.xcb_mod_prefix, &typ);
                    writeln!(out, "        {}: {},", &name, &typ)?;
                }
                StructField::ValueParam {
                    mask_typ,
                    mask_name,
                    list_name,
                } => {
                    let mask_typ =
                        ffi_field_type_name(&self.xcb_mod, &self.xcb_mod_prefix, &mask_typ);
                    let list_name = symbol(list_name);

                    if !written_fields.contains(&mask_name) {
                        let mask_name = symbol(mask_name);
                        writeln!(out, "        {}: {},", &mask_name, &mask_typ)?;
                    }
                    writeln!(out, "        {}: *const u32,", &list_name)?;
                }
                StructField::List { name, typ, .. } => {
                    let name = symbol(&name);
                    let typ = ffi_field_type_name(&self.xcb_mod, &self.xcb_mod_prefix, &typ);
                    writeln!(out, "        {}: *const {},", &name, &typ)?;
                }
                StructField::ListNoLen { name, typ } => {
                    let len_name = name.clone() + "_len";
                    let name = symbol(&name);
                    let typ = ffi_field_type_name(&self.xcb_mod, &self.xcb_mod_prefix, &typ);
                    writeln!(out, "        {}: u32,", &len_name)?;
                    writeln!(out, "        {}: *const {},", &name, &typ)?;
                }
                _ => {}
            }
        }
        writeln!(out, "    ) -> {};", &cookie_typ)?;

        Ok(())
    }

    fn emit_ffi_reply(&mut self, req_name: &str, mut reply: Reply) -> io::Result<String> {
        // writting cookie struct
        let cookie_name = req_name.to_string() + "Cookie";
        let cookie_ffi_typ = ffi_type_name(&self.xcb_mod_prefix, &cookie_name);
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

        let out = &mut self.ffi;
        writeln!(out)?;
        writeln!(out, "#[derive(Copy, Clone, Debug)]")?;
        writeln!(out, "#[repr(C)]")?;
        writeln!(out, "pub struct {} {{", &cookie_ffi_typ)?;
        writeln!(out, "    pub(crate) sequence: c_uint,")?;
        writeln!(out, "}}")?;

        let ffi_reply_typ = self.emit_ffi_struct(&reply)?;

        self.emit_ffi_field_list_accessors(&ffi_reply_typ, &req_name, &reply.fields)?;

        {
            let ffi_reply_fn = ffi_reply_fn_name(&self.xcb_mod_prefix, &req_name);
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

        Ok(cookie_ffi_typ)
    }

    fn emit_rs_req_fn(
        &mut self,
        req_name: &str,
        cookie_name: &str,
        params: &[StructField],
        doc: &Option<Doc>,
        checked: bool,
    ) -> io::Result<()> {
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
        writeln!(out, "pub fn {}<'a>(", &fn_name)?;
        writeln!(out, "    c: &'a base::Connection,")?;
        for f in params.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    let name = symbol(&name);
                    let typ = rust_field_type_name(&self.xcb_mod, &typ);
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
        for f in params.iter() {
            match f {
                StructField::ValueParam { list_name, .. } => {
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
                _ => {}
            }
        }
        writeln!(out, "        let cookie = {}(", &ffi_fn_name)?;
        writeln!(out, "            c.get_raw_conn(),")?;
        // FIXME: ind and pos only to have diff identical comments and not useful otherwise
        // should be removed before merging once tests are passed
        let mut ind = 0usize;
        let mut pos = 0;
        for f in params.iter() {
            let is_last = ind == params.len();
            let pos_str = if is_last {
                format!(" // {}", pos)
            } else {
                String::new()
            };
            match f {
                StructField::Field { name, typ, .. } => {
                    let name = symbol(&name);
                    let ffi_typ = ffi_field_type_name(&self.xcb_mod, &self.xcb_mod_prefix, &typ);
                    writeln!(out, "            {} as {},{}", name, ffi_typ, &pos_str)?;
                    pos += 1;
                }
                StructField::ValueParam {
                    list_name,
                    mask_typ,
                    ..
                } => {
                    let typ = rust_field_type_name(&self.xcb_mod, &mask_typ);
                    writeln!(
                        out,
                        "            {}_mask as {},{}",
                        &list_name, &typ, &pos_str
                    )?;
                    writeln!(out, "             {}_ptr as *const u32,", &list_name)?;
                    pos += 1;
                }
                _ => {}
            }
            ind += 1;
        }
        writeln!(out, "        ); // {}", ind)?; // ind here is actually a bug in the python script
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
        cookie_ffi_typ: &str,
        reply: Reply,
    ) -> io::Result<()> {
        let out = &mut self.rs_buf;
        writeln!(out, "impl base::CookieSeq for {} {{", &cookie_ffi_typ)?;
        writeln!(out, "    fn sequence(&self) -> c_uint {{")?;
        writeln!(out, "        self.sequence")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }

    fn emit_xid(&mut self, name: String) -> io::Result<()> {
        let ffi_typ = ffi_type_name(&self.xcb_mod_prefix, &name);
        emit_type_alias(&mut self.ffi, &ffi_typ, "u32")?;
        self.emit_ffi_iterator(&name, &ffi_typ, false)?;

        let rs_typ = rust_type_name(&name);
        emit_type_alias(&mut self.rs, &rs_typ, &ffi_typ)?;

        self.reg_type(name, ffi_typ, rs_typ, Some(4), false, true);
        Ok(())
    }

    fn emit_struct(&mut self, stru: Struct) -> io::Result<()> {
        let has_lifetime = self.type_has_lifetime(&stru);
        if has_lifetime {
            self.typ_with_lifetime.insert(stru.name.clone());
        }
        let ffi_typ = self.emit_ffi_struct(&stru)?;
        self.emit_ffi_field_list_accessors(&ffi_typ, &stru.name, &stru.fields)?;
        let ffi_it_typ = self.emit_ffi_iterator(&stru.name, &ffi_typ, has_lifetime)?;

        let rs_typ = self.emit_rs_struct(&ffi_typ, &stru, has_lifetime)?;
        self.emit_rs_iterator(&stru.name, &rs_typ, &ffi_it_typ, has_lifetime, false)?;

        let stru_sz = self.compute_ffi_struct_size(&stru);
        self.reg_type(stru.name, ffi_typ, rs_typ, stru_sz, false, false);

        Ok(())
    }

    fn emit_union(&mut self, stru: Struct) -> io::Result<()> {
        let ffi_sz = self.compute_ffi_union_size(&stru);
        let ffi_typ = ffi_type_name(&self.xcb_mod_prefix, &stru.name);

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

        let ffi_it_typ = ffi_iterator_name(&self.xcb_mod_prefix, &stru.name);

        self.emit_rs_iterator(&stru.name, &rs_typ, &ffi_it_typ, false, true)?;

        self.reg_type(stru.name, ffi_typ, rs_typ, Some(ffi_sz), true, false);

        Ok(())
    }

    fn emit_error(&mut self, number: i32, stru: Struct) -> io::Result<()> {
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

        let ffi_typ = self.emit_ffi_struct(&stru)?;

        let rs_typ = rust_type_name(&stru.name);

        emit_rs_error(&mut self.rs, &ffi_typ, &rs_typ)?;

        self.reg_type(stru.name, ffi_typ, rs_typ, None, false, false);

        Ok(())
    }

    fn emit_error_copy(&mut self, name: &str, number: i32, error_ref: &str) -> io::Result<()> {
        emit_ffi_opcode(&mut self.ffi, &self.xcb_mod_prefix, &name, number)?;
        let new_name = name.to_owned() + "Error";
        let old_name = error_ref.to_owned() + "Error";

        let new_ffi_typ = ffi_type_name(&self.xcb_mod_prefix, &new_name);
        let old_ffi_typ = ffi_type_name(&self.xcb_mod_prefix, &old_name);

        emit_type_alias(&mut self.ffi, &new_ffi_typ, &old_ffi_typ)?;

        let rs_typ = rust_type_name(&new_name);

        emit_rs_error(&mut self.rs, &new_ffi_typ, &rs_typ)?;
        emit_rs_opcode(&mut self.rs_buf, &name, number)?;

        Ok(())
    }

    fn emit_event(
        &mut self,
        number: i32,
        stru: Struct,
        no_seq_number: bool,
        xge: bool,
    ) -> io::Result<()> {
        emit_ffi_opcode(&mut self.ffi, &self.xcb_mod_prefix, &stru.name, number)?;

        let opcopies = self
            .evcopies
            .remove(&stru.name)
            .expect("missing event copies");

        let Struct {
            name: orig_name,
            fields: mut orig_fields,
            doc,
        } = stru;

        let fields = {
            let mut fields = vec![StructField::Field {
                name: "response_type".into(),
                typ: "CARD8".into(),
                enu: None,
            }];

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
                if xge && sz < 32 {
                    sz += self
                        .compute_ffi_struct_field_sizeof(&f)
                        .expect(&format!("can't compute ffi full_sequence pos"));
                    fields.push(f);
                    if sz == 32 {
                        fields.push(StructField::Field {
                            name: "full_sequence".into(),
                            typ: "CARD32".into(),
                            enu: None,
                        });
                        sz += 4;
                    }
                } else {
                    fields.push(f);
                }
            }
            fields
        };
        let stru = Struct {
            name: orig_name.clone() + "Event",
            fields,
            doc,
        };

        let ffi_typ = self.emit_ffi_struct(&stru)?;
        let ffi_sz = self.compute_ffi_struct_size(&stru);

        for c in opcopies.iter() {
            emit_ffi_opcode(&mut self.ffi, &self.xcb_mod_prefix, &c.name, c.number)?;
            let new_name = c.name.to_owned() + "Event";

            let new_ffi_typ = ffi_type_name(&self.xcb_mod_prefix, &new_name);
            let old_ffi_typ = ffi_type_name(&self.xcb_mod_prefix, &stru.name);

            emit_type_alias(&mut self.ffi, &new_ffi_typ, &old_ffi_typ)?;
        }

        let rs_typ = self.emit_rs_event(&orig_name, number, &stru, &ffi_typ, &opcopies, xge)?;

        self.reg_type(stru.name, ffi_typ, rs_typ, ffi_sz, false, false);

        Ok(())
    }

    fn emit_request(&mut self, mut req: Request) -> io::Result<()> {
        emit_ffi_opcode(&mut self.ffi, &self.xcb_mod_prefix, &req.name, req.opcode)?;

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

        let stru = Struct {
            name: req.name.clone() + "Request",
            fields,
            doc: req.doc.clone(),
        };
        self.emit_ffi_struct(&stru)?;

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
        self.emit_rs_req_fn(&req.name, &rs_cookie, &req.params, &stru.doc, !checked)?;
        self.emit_rs_req_fn(&check_name, &rs_cookie, &req.params, &stru.doc, checked)?;

        if let Some(reply) = req.reply.take() {
            let cookie_ffi_typ = self.emit_ffi_reply(&req.name, reply.clone())?;
            self.emit_rs_reply(&req.name, &cookie_ffi_typ, reply)?;
        }

        self.emit_ffi_req_fn(&req.name, &ffi_cookie, &req.params, &stru.doc)?;
        self.emit_ffi_req_fn(&check_name, &ffi_cookie, &req.params, &stru.doc)?;

        Ok(())
    }
}

fn has_variable_list(fields: &[StructField]) -> bool {
    for f in fields.iter() {
        match f {
            StructField::List { len_expr, .. } => {
                if expr_fixed_length(&len_expr).is_none() {
                    return true;
                }
            }
            _ => {}
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
        Expr::FieldRef(_) => None,
        Expr::ParamRef(_) => None,
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

fn ffi_type_name(xcb_mod_prefix: &str, typ: &str) -> String {
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
        "void" => "c_void".into(),
        typ => {
            let typ = tit_split(typ).to_ascii_lowercase();
            format!("xcb_{}{}_t", xcb_mod_prefix, typ)
        }
    }
}

/// same as ffi_type_name but can also have a namespace before (with a single colon)
fn ffi_field_type_name(xcb_mod: &str, xcb_mod_prefix: &str, typ: &str) -> String {
    let (module, typ) = extract_module(&typ);

    let typ = ffi_type_name(&xcb_mod_prefix, &typ);

    qualified_name(&xcb_mod, &module, &typ)
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
