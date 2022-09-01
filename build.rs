use std::collections::HashSet;

extern crate bindgen;
extern crate pkg_config;

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn get_ignored_macros() -> IgnoreMacros {
    let mut ignored = HashSet::new();
    ignored.insert("FE_INVALID".to_string());
    ignored.insert("FE_DIVBYZERO".to_string());
    ignored.insert("FE_OVERFLOW".to_string());
    ignored.insert("FE_UNDERFLOW".to_string());
    ignored.insert("FE_INEXACT".to_string());
    ignored.insert("FE_TONEAREST".to_string());
    ignored.insert("FE_DOWNWARD".to_string());
    ignored.insert("FE_UPWARD".to_string());
    ignored.insert("FE_TOWARDZERO".to_string());
    IgnoreMacros(ignored)
}

fn main() {
    let _ = pkg_config::probe_library("libobs").unwrap();

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .blacklist_type("_bindgen_ty_2")
        // This is a workaround for a redefine error from bindgen.
        .parse_callbacks(Box::new(get_ignored_macros()))
        .generate()
        .expect("Unable to generate libOBS bindings. Do you have OBS >= 23.0.0 installed?");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings in the directory");
}
