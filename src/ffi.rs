use crate::codegen::CodeGen;
use crate::output::Output;
use crate::parse::Event;
use std::io::{self, Cursor, Write};

#[derive(Debug)]
pub struct FfiXcbEmit {
    out: Output,                 // direct output
    ext_fn_out: Cursor<Vec<u8>>, // all the extern functions are grouped at the end and emitted at once
}

impl FfiXcbEmit {
    pub fn new(out: Output) -> FfiXcbEmit {
        FfiXcbEmit {
            out,
            ext_fn_out: Cursor::new(Vec::new()),
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

    pub fn event(&mut self, cg: &mut CodeGen, ev: &Event) -> io::Result<()> {
        match ev {
            Event::XidType(name) => {
                let typ = cg.ffi_type_name(name);
                cg.emit_type_alias(&mut self.out, &typ, "u32")?;

                self.emit_iterator(cg, name)?;
                cg.reg_type(typ);
            }
            Event::Enum {
                name,
                bitset,
                items,
                ..
            } => {
                let out = &mut self.out;
                writeln!(out)?;
                let typ = cg.ffi_enum_type_name(name);

                writeln!(out, "pub type {} = u32;", typ)?;
                for item in items {
                    let val = if *bitset {
                        format!("0x{:02X}", item.value)
                    } else {
                        item.value.to_string()
                    };
                    writeln!(
                        out,
                        "const {}: {} = {};",
                        cg.ffi_enum_item_name(name, &item.name),
                        typ,
                        val
                    )?;
                }
                cg.reg_type(typ)
            }
            _ => {}
        }
        Ok(())
    }

    fn emit_iterator(&mut self, cg: &mut CodeGen, typ: &str) -> io::Result<()> {
        let out = &mut self.out;
        let typ = typ.to_ascii_lowercase();
        let it_typ = cg.ffi_iterator_name(&typ);
        let it_next = cg.ffi_iterator_next_fn_name(&typ);
        let it_end = cg.ffi_iterator_end_fn_name(&typ);

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
