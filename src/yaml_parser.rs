extern crate yaml_rust;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use yaml_rust::YamlLoader;

#[derive(Debug, Clone)]
pub struct DnameStruct {
    pub u_items: usize,
    pub items: Vec<HashMap<String, String>>,
}

pub struct RsfSettings {
    pub rsf_path: Option<String>,
    pub rsf_set: HashMap<String, String>, // Placeholder for settings structure
    pub dname: DnameStruct,
}

pub struct UserSettings {
    pub common: RsfSettings,
}

pub struct YamlContext {
    pub done: bool,
    pub error: bool,
    pub string: Option<String>,
    pub dname: Option<DnameStruct>,
}

impl YamlContext {
    fn new(dname: Option<DnameStruct>) -> Self {
        YamlContext {
            done: false,
            error: false,
            string: None,
            dname,
        }
    }

    fn process_yaml_string(&mut self, raw_str: &str) {
        if let Some(ref mut string) = self.string {
            // Process the raw string similarly to the C code
            let mut proc_str_len = 0;
            let mut processed_string = String::new();

            let mut pos = 0;
            while let Some(start) = raw_str[pos..].find("$(") {
                let sub_start = start + pos;
                let sub_end = raw_str[sub_start + 2..].find(")").unwrap_or(0) + sub_start + 2;

                processed_string.push_str(&raw_str[pos..sub_start]); // Push content before the variable

                let sub_name = &raw_str[sub_start + 2..sub_end];
                if let Some(dname) = &self.dname {
                    for item in &dname.items {
                        if let Some(val) = item.get(sub_name) {
                            processed_string.push_str(val);
                            break;
                        }
                    }
                }
                pos = sub_end + 1;
            }
            processed_string.push_str(&raw_str[pos..]); // Append remaining part of the string
            self.string = Some(processed_string);
        }
    }

    fn get_yaml_string(&self) -> Option<&str> {
        self.string.as_deref()
    }
}

fn init_yaml_context(ctx: &mut YamlContext, dname: Option<DnameStruct>) {
    *ctx = YamlContext::new(dname);
}

fn parse_spec_file(
    set: &mut RsfSettings,
    path: &str,
    dname: &DnameStruct,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let docs = YamlLoader::load_from_str(&content)?;
    if let Some(doc) = docs.get(0) {
        // Process the YAML document
        let mut ctx = YamlContext::new(Some(dname.clone()));

        ctx.process_yaml_string(doc.as_str().unwrap_or_default());
        set.rsf_set.insert("parsed".to_string(), "true".to_string()); // Placeholder logic
    }

    Ok(())
}

fn get_rsf_settings(set: &mut UserSettings) -> Result<(), Box<dyn Error>> {
    if let Some(ref path) = set.common.rsf_path {
        // Immutable borrow ends here as we move path into a local variable
        let path = path.clone();

        // Check if the file exists before proceeding
        if !std::path::Path::new(&path).exists() {
            eprintln!("[RSF ERROR] Failed to open {}", path);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            )));
        }

        // Now you can safely pass the mutable reference to `set.common`
        let dname_clone = set.common.dname.clone();
        parse_spec_file(&mut set.common, &path, &dname_clone)?; // Pass the clone of dname
    }
    Ok(())
}
