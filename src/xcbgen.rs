use crate::ffi::FfiXcbEmit;
use crate::rust::RustXcbEmit;
use crate::output::Output;
use crate::naming::Naming;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};
// pub struct TypAnnot {
//     name: String,
//     borrow: bool,
//     mutable: bool,
// }

// pub struct Field {
//     name: String,
//     typ: TypAnnot,
// }

// pub struct EnumItem {
//     name: String,
//     value: u32,
// }

// pub struct Enum {
//     name: String,
//     items: Vec<EnumItem>,
// }

pub trait XcbEmit {
    fn emit_proloque(&mut self) -> io::Result<()>;
    fn emit_xidtype(&mut self, name: &str) -> io::Result<()>;
}

#[derive(Debug)]
pub struct XcbGen {
    ffi: FfiXcbEmit,
    rust: RustXcbEmit,
}

impl XcbGen {
    pub fn new(xcb_mod: &str, ffi: Output, rust: Output) -> XcbGen {
        let naming = Naming::new(xcb_mod);

        let ffi = FfiXcbEmit::new(naming.clone(), ffi);
        let rust = RustXcbEmit::new(naming, rust);
        XcbGen { ffi, rust }
    }

    pub fn xcb_gen(mut self, xml_file: &Path) -> io::Result<()> {
        let xml_file = File::open(&xml_file)?;
        let xml_file = BufReader::new(xml_file);

        let parser = EventReader::new(xml_file);

        self.emit_proloque()?;

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.local_name == "xidtype" {
                        let name = find_attr(&attributes, "name")
                            .expect("xidtype element must have name attribute");
                        self.emit_xidtype(&name)?;
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl XcbEmit for XcbGen {
    fn emit_proloque(&mut self) -> io::Result<()> {
        self.ffi.emit_proloque()?;
        self.rust.emit_proloque()?;
        Ok(())
    }

    fn emit_xidtype(&mut self, name: &str) -> io::Result<()> {
        self.ffi.emit_xidtype(name)?;
        self.rust.emit_xidtype(name)?;
        Ok(())
    }
}

fn find_attr<'a>(attrs: &'a Vec<OwnedAttribute>, name: &str) -> Option<&'a str> {
    return attrs
        .iter()
        .filter_map(|a| {
            if a.name.local_name == name {
                Some(a.value.as_str())
            } else {
                None
            }
        })
        .next();
}
