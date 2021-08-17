#[derive(Debug, Clone)]
pub struct Naming {
    xcb_mod: String,
}

impl Naming {
    pub fn new(xcb_mod: &str) -> Naming {
        Naming {
            xcb_mod: xcb_mod.to_owned(),
        }
    }

    pub fn xcb_mod(&self) -> &str {
        &self.xcb_mod
    }

    pub fn ffi_type(&self, typ: &str) -> String {
        let m = if self.xcb_mod == "xproto" {
            String::new()
        } else {
            format!("{}_", &self.xcb_mod)
        };

        format!("xcb_{}{}_t", m.to_lowercase(), typ.to_lowercase())
    }

    pub fn rust_type(&self, typ: &str) -> String {
        capitalize(typ)
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
