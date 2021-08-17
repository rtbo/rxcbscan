use crate::ffi::FfiXcbEmit;
use crate::naming::Naming;
use crate::output::Output;
use crate::rust::RustXcbEmit;
use quick_xml::events::Event as XmlEv;
use quick_xml::Reader as XmlReader;
use std::io::{self, BufRead};
use std::path::Path;
use std::str;
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

#[derive(Debug)]
pub struct XcbGen {
    ffi: FfiXcbEmit,
    rust: RustXcbEmit,
    xcb_mod: String,
}

impl XcbGen {
    pub fn new(xcb_mod: &str, ffi: Output, rust: Output) -> XcbGen {
        let naming = Naming::new(xcb_mod);

        let ffi = FfiXcbEmit::new(naming.clone(), ffi);
        let rust = RustXcbEmit::new(naming, rust);
        XcbGen {
            ffi,
            rust,
            xcb_mod: xcb_mod.into(),
        }
    }

    pub fn xcb_gen(mut self, xml_file: &Path) -> io::Result<()> {
        let xml = XmlReader::from_file(xml_file).unwrap();
        let parser = XcbParser {
            xml,
            buf: Vec::with_capacity(8 * 1024),
        };

        self.ffi.emit_proloque()?;
        self.rust.emit_proloque()?;

        for ev in parser {
            self.ffi.event(&ev)?;
            self.rust.event(&ev)?;
        }

        Ok(())
    }
}

// fn find_attr<'a>(attrs: &'a Vec<OwnedAttribute>, name: &str) -> Option<&'a str> {
//     return attrs
//         .iter()
//         .filter_map(|a| {
//             if a.name.local_name == name {
//                 Some(a.value.as_str())
//             } else {
//                 None
//             }
//         })
//         .next();
// }
#[derive(Debug)]
pub enum Event {
    Import(String),
    XidType(String),
    Ignore,
}

struct XcbParser<B: BufRead> {
    xml: XmlReader<B>,
    buf: Vec<u8>,
}

impl<B: BufRead> XcbParser<B> {
    fn parse_import(&mut self) -> Option<String> {
        let mut s = None;
        loop {
            match self.xml.read_event(&mut self.buf) {
                Ok(XmlEv::Text(e)) => {
                    s = Some(e.unescape_and_decode(&self.xml).unwrap());
                }
                Ok(XmlEv::End(ref e)) => match e.name() {
                    b"import" => break,
                    _ => {
                        s = None;
                        break;
                    }
                },
                Err(_) => {
                    s = None;
                    break;
                },
                _ => {},
            }
        }
        s
    }
}

impl<B: BufRead> Iterator for XcbParser<B> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.clear();
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::Empty(ref e) | XmlEv::Start(ref e)) => match e.name() {
                b"xidtype" => {
                    let name = e
                        .attributes()
                        .find(|a| a.as_ref().unwrap().key == b"name")
                        .expect("XID type without name")
                        .unwrap();
                    Some(Event::XidType(
                        name.unescape_and_decode_value(&self.xml)
                            .expect("can't escape argument"),
                    ))
                }
                b"import" => self.parse_import().map(|s| Event::Import(s)),
                _ => Some(Event::Ignore),
            },

            Ok(XmlEv::Eof) => None,

            _ => Some(Event::Ignore),
        }
    }
}
