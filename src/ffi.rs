use crate::codegen::CodeGen;
use crate::output::Output;
use crate::xcbgen::Event;
use std::io::{self, Cursor, Write};
use std::collections::HashSet;

#[derive(Debug)]
pub struct FfiXcbEmit {
    cg: CodeGen,
    out: Output,                 // direct output
    ext_fn_out: Cursor<Vec<u8>>, // all the extern functions are grouped
    typ_reg: HashSet<String>,        // types registered
}

impl FfiXcbEmit {
    pub fn new(cg: CodeGen, out: Output) -> FfiXcbEmit {
        FfiXcbEmit {
            cg,
            out,
            ext_fn_out: Cursor::new(Vec::new()),
            typ_reg: HashSet::new(),
        }
    }

    pub fn prologue(&mut self, imports: &Vec<String>) -> io::Result<()> {
        let out = &mut self.out;
        writeln!(out, "use libc::{{c_char, c_int, c_uint, c_void}};")?;
        writeln!(out, "use std;")?;
        writeln!(out)?;
        writeln!(out, "use ffi::base::*;")?;
        for imp in imports.iter() {
            writeln!(out, "use ffi::{}::*;", imp)?;
        }
        writeln!(out)?;
        Ok(())
    }

    pub fn epilogue(&mut self) -> io::Result<()> {
        let out = &mut self.out;
        // write out all the external functions
        writeln!(out).unwrap();
        writeln!(out, "extern {{")?;

        out.write(self.ext_fn_out.get_ref())?;

        writeln!(out).unwrap();
        writeln!(out, "}} // extern")?;
        Ok(())
    }

    pub fn event(&mut self, ev: &Event) -> io::Result<()> {
        match ev {
            Event::XidType(name) => {
                let typ = self.cg.ffi_type_name(name);
                self.cg
                    .emit_type_alias(&mut self.out, &typ, "u32")?;

                self.emit_iterator(name)?;
                self.typ_reg.insert(typ);
            }
            Event::Enum {
                name,
                bitset,
                items,
                ..
            } => {
                let out = &mut self.out;
                writeln!(out)?;
                let typ = self.cg.ffi_type_name(name);
                let typ = if self.typ_reg.contains(&typ) {
                    self.cg.ffi_type_name(&(name.to_owned() + "_enum"))
                }
                else {
                    typ
                };

                writeln!(out, "pub type {} = u32;", typ)?;
                for item in items {
                    let val = if *bitset {
                        format!("0x{:02X}", item.value)
                    } else {
                        format!("{}", item.value)
                    };
                    writeln!(
                        out,
                        "const {}: {} = {};",
                        self.cg.ffi_enum_item_name(name, &item.name),
                        typ,
                        val
                    )?;
                }
                self.typ_reg.insert(typ);
            }
            _ => {}
        }
        Ok(())
    }

    fn emit_iterator(&mut self, typ: &str) -> io::Result<()> {
        let out = &mut self.out;
        let typ = typ.to_ascii_lowercase();
        let it_typ = self.cg.ffi_iterator_name(&typ);
        let it_next = self.cg.ffi_iterator_next_fn_name(&typ);
        let it_end = self.cg.ffi_iterator_end_fn_name(&typ);

        writeln!(out)?;
        writeln!(out, "#[repr(C)]")?;
        writeln!(out, "#[derive(Debug)]")?;
        writeln!(out, "pub struct {} {{", &it_typ)?;
        writeln!(out, "    pub data:  *mut xcb_{}_t,", &typ)?;
        writeln!(out, "    pub rem:   c_int,")?;
        writeln!(out, "    pub index: c_int,")?;
        writeln!(out, "}}")?;

        let out = &mut self.ext_fn_out;
        writeln!(out).unwrap();
        writeln!(out, "fn {}(i: *mut {});", &it_next, &it_typ).unwrap();
        writeln!(
            out,
            "fn {}(i: *mut {}) -> xcb_generic_iterator_t;",
            &it_end, &it_typ
        )
        .unwrap();
        Ok(())
    }
}
