use std::io::{self, Write};

use super::ffi;

use super::{
    emit_doc_field, emit_doc_field_indent, emit_doc_text, expr_fixed_length, extract_module,
    qualified_name, request_has_template, symbol, tit_cap, tit_split, CodeGen, ListField,
};
use crate::ast::{Doc, OpCopy, Reply, Struct, StructField};

impl CodeGen {
    fn has_rs_type(&self, typ: &str) -> bool {
        self.rs_typ_reg.contains(typ)
    }

    pub fn rs_enum_type_name(&mut self, typ: &str) -> String {
        let try1 = type_name(&typ);
        if self.has_rs_type(&try1) {
            format!("{}Enum", &try1)
        } else {
            try1
        }
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
                            field_type_name(&self.xcb_mod, &typ)
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
                            field_type_name(&self.xcb_mod, &typ)
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
                        let typ = field_type_name(&self.xcb_mod, &typ);
                        writeln!(out, "    pub fn {}(&self) -> &[{}] {{", &name, &typ)?;
                        writeln!(out, "        unsafe {{ &(*self.ptr).{} }}", &name)?;
                        writeln!(out, "    }}")?;
                    } else if is_simple {
                        let len_fn = ffi::field_list_iterator_len_fn_name(
                            &self.xcb_mod_prefix,
                            &stru.name,
                            &name,
                        );
                        let acc_fn = ffi::field_list_iterator_acc_fn_name(
                            &self.xcb_mod_prefix,
                            &stru.name,
                            &name,
                        );
                        let is_template = typ == "void";
                        let template = if is_template { "<T>" } else { "" };
                        let ret = match typ.as_str() {
                            "char" => "&str".to_string(),
                            "void" => "&[T]".to_string(),
                            typ => format!("&[{}]", field_type_name(&self.xcb_mod, &typ)),
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
                        let it_typ = iterator_type_name(&typ);
                        let ffi_it_fn_name = ffi::field_list_iterator_it_fn_name(
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
                            field_type_name(&self.xcb_mod, &typ)
                        )?;
                    }
                    _ => {}
                }
            }
            writeln!(out, ") -> {} {{", &rs_typ)?;
            writeln!(out, "        unsafe {{")?;
            writeln!(out, "            {} {{", &rs_typ)?;
            writeln!(out, "                base: {} {{", &ffi_typ)?;
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
                    StructField::Pad(name, sz) => {
                        let val = if *sz == 1 {
                            "0".into()
                        } else {
                            format!("[0; {}]", sz)
                        };
                        writeln!(out, "                {}: {},", name, val)?;
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

    pub fn emit_rs_iterator(
        &mut self,
        name: &str,
        typ: &str,
        ffi_it_typ: &str,
        has_lifetime: bool,
        is_union: bool,
    ) -> io::Result<()> {
        let it_typ = format!("{}Iterator", &typ);
        let ffi_it_next = ffi::iterator_next_fn_name(&self.xcb_mod_prefix, &name);

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

    pub fn emit_rs_struct(
        &mut self,
        ffi_typ: &str,
        stru: &Struct,
        has_lifetime: bool,
    ) -> io::Result<String> {
        let Struct { name, doc, .. } = &stru;

        let typ = type_name(&name);

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

    pub fn emit_rs_union_impl(
        &mut self,
        rs_typ: &str,
        ffi_sz: usize,
        stru: &Struct,
    ) -> io::Result<()> {
        let out = &mut self.rs_buf;

        let Struct { fields, doc, .. } = &stru;

        writeln!(out)?;
        writeln!(out, "impl {} {{", &rs_typ)?;

        // emitting accessors
        for f in fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    let name = symbol(name);
                    let f_rs_typ = field_type_name(&self.xcb_mod, &typ);
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
                    let f_rs_typ = field_type_name(&self.xcb_mod, &typ);
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

    pub fn emit_rs_event(
        &mut self,
        name: &str,
        number: i32,
        stru: &Struct,
        ffi_typ: &str,
        opcopies: &[OpCopy],
        xge: bool,
    ) -> io::Result<String> {
        let rs_typ = type_name(&stru.name);

        let opn = opname(&name);
        let num_typ = if number < 0 { "i8" } else { "u8" };

        {
            let out = &mut self.rs_buf;
            writeln!(out)?;
            writeln!(out, "pub const {}: {} = {};", &opn, &num_typ, number)?;

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
                    &opn
                )?;
            } else {
                writeln!(out, "    /// `response_type` must be set to one of:")?;
                writeln!(out, "    ///     - `{}`", &opn)?;
                for op in opcopies.iter() {
                    let opn = opname(&op.name);
                    writeln!(out, "    ///     - `{}`", &opn)?;
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
                            &field_type_name(&self.xcb_mod, &typ)
                        )?;
                    }
                    StructField::Pad(_, _) => {}
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
                                let typ = field_type_name(&self.xcb_mod, &typ);
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
                let copies = opcopies.iter().map(|opc| opname(&opc.name));
                let opcodes = Some(opn.clone()).into_iter().chain(copies);
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
                writeln!(out, "            (*raw).response_type = {};", &opn)?;
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
            let opn = opname(&opc.name);
            let name = opc.name.clone() + "Event";
            let num_typ = if number < 0 { "i8" } else { "u8" };
            let rs_typ = type_name(&name);
            let ffi_typ = self.ffi_decl_type_name(&name);

            let out = &mut self.rs_buf;
            writeln!(out)?;
            writeln!(out, "pub const {}: {} = {};", &opn, &num_typ, &opc.number)?;

            writeln!(out)?;
            emit_doc_text(out, &stru.doc)?;
            writeln!(out, "pub type {} = base::Event<{}>;", &rs_typ, &ffi_typ)?;
        }

        Ok(rs_typ)
    }

    pub fn emit_rs_req_fn(
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
            ("Send", "xevie") => true,
            ("SendUnchecked", "xevie") => true,
            (_, _) => false,
        };
        let event_is_list = if send_event {
            let mut is_list = false;
            for f in params.iter() {
                if let StructField::List{name, ..} = f {
                    if name == "event" {
                        is_list = true;
                        break;
                    }
                }
            }
            is_list
        } else {
            false
        };

        let has_template = request_has_template(&params) || event_is_list;
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
                    if let Some(name) = field_doc_name(&f) {
                        writeln!(out, "///")?;
                        writeln!(out, "///   - __{}__:", &name)?;
                        emit_doc_field_indent(out, &doc, name, "       ")?;
                    }
                }
            }
            let fn_name = request_fn_name(&req_name);
            let ffi_fn_name = ffi::request_fn_name(&self.xcb_mod_prefix, &req_name);
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
                        let typ = field_type_name(&self.xcb_mod, &typ);
                        writeln!(out, "    {}: {},", name, typ)?;
                    }
                    StructField::List { name, typ, .. } | StructField::ListNoLen { name, typ } => {
                        let name = symbol(&name);
                        let typ = match (send_event, name, typ.as_str()) {
                            (true, "event", _) => "&base::Event<T>".to_string(),
                            (_, _, "char") => "&str".to_string(),
                            (_, _, "void") => "&[T]".to_string(),
                            (_, _, typ) => {
                                format!("&[{}]", field_type_name(&self.xcb_mod, &typ))
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
                        let typ = field_type_name(&self.xcb_mod, &mask_typ);
                        writeln!(out, "    {}: &[({}, u32)],", &name, &typ)?;
                    }
                    _ => {}
                }
            }
            writeln!(out, ") -> {}<'a> {{", cookie_name)?;
            writeln!(out, "    unsafe {{")?;

            // local variables
            if event_is_list {
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
                    if send_event && name == "event" {
                        let out = &mut self.rs_buf;
                        writeln!(out, "            {}.base,", &name)?;
                    } else {
                        let ffi_typ = self.ffi_use_type_name(&typ);

                        if let Some(lf) = list_fields.iter().find(|lf| lf.lenfield == name) {
                            name = lf.name.clone() + "_len";
                        }
                        let out = &mut self.rs_buf;
                        writeln!(out, "            {} as {},", &name, &ffi_typ)?;
                    }
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
                    let typ = field_type_name(&self.xcb_mod, &mask_typ);
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

    pub fn emit_rs_reply(
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
}

pub fn type_name(typ: &str) -> String {
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

fn iterator_type_name(typ: &str) -> String {
    format!("{}Iterator", tit_cap(&typ))
}

/// same as rust_type_name but can also have a namespace before (with a single colon)
fn field_type_name(xcb_mod: &str, typ: &str) -> String {
    let (module, typ) = extract_module(&typ);

    let typ = type_name(&typ);

    qualified_name(&xcb_mod, &module, &typ)
}

fn field_doc_name(f: &StructField) -> Option<&str> {
    match f {
        StructField::Field { name, .. } => Some(symbol(name)),
        StructField::List { name, .. } => Some(symbol(name)),
        StructField::ListNoLen { name, .. } => Some(symbol(name)),
        StructField::Fd(name) => Some(symbol(name)),
        StructField::ValueParam { list_name, .. } => Some(symbol(list_name)),
        _ => None,
    }
}

pub fn enum_item_name(name: &str, item: &str) -> String {
    format!("{}_{}", tit_split(name), tit_split(item)).to_ascii_uppercase()
}

pub fn opname(name: &str) -> String {
    tit_split(&name).to_ascii_uppercase()
}

fn request_fn_name(name: &str) -> String {
    let fn_name = tit_split(&name).to_ascii_lowercase();
    match fn_name.as_str() {
        "await" => "await_".into(),
        _ => fn_name,
    }
}

pub fn emit_opcode<Out: Write>(out: &mut Out, name: &str, opcode: i32) -> io::Result<()> {
    let opname = opname(&name);
    let num_typ = if opcode < 0 { "i8" } else { "u8" };

    writeln!(out)?;
    writeln!(out, "pub const {}: {} = {};", &opname, &num_typ, opcode)?;

    Ok(())
}

pub fn emit_error<Out: Write>(out: &mut Out, ffi_typ: &str, rs_typ: &str) -> io::Result<()> {
    writeln!(out)?;
    writeln!(out, "pub struct {} {{", &rs_typ)?;
    writeln!(out, "    pub base: base::Error<{}>,", &ffi_typ)?;
    writeln!(out, "}}")?;

    Ok(())
}
