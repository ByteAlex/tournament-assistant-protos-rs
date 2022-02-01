use std::fs;
use std::path::{Path, PathBuf};

fn collect_proto_files(dir: fs::ReadDir) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for file in dir {
        let path = file?.path();
        if path.is_dir() {
            let dir_files = collect_proto_files(path.read_dir()?)?;
            files.extend(dir_files.into_iter());
        }
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension.eq("proto") {
                    files.push(path);
                }
            }
        }
    }
    Ok(files)
}

fn copy_into_src() -> std::io::Result<Vec<String>> {
    let mut modules = Vec::new();
    let src_dir = PathBuf::from("src/");
    let dir = std::env::var_os("OUT_DIR").expect("Out dir not set!");
    let dir = PathBuf::from(dir);
    for entry in dir.read_dir()? {
        let path = entry?.path();
        if path.is_file() {
            let name = path.file_name().expect("File name").to_str().unwrap();
            if name.starts_with("proto.") && name.ends_with(".rs") {
                let raw_name = &name[6..name.len()-3];
                modules.push(raw_name.to_string());
                let mut copy_file = src_dir.clone();
                copy_file.push(&name[6..]);
                fs::rename(path, copy_file)?;
            }
        }
    }
    Ok(modules)
}

pub fn generate_lib_file(mut modules: Vec<String>) -> String {
    modules.sort();
    let mut lib_file = "".to_string();
    for module in modules {
        lib_file.push_str(format!("pub mod {};\n", module).as_str());
    }
    lib_file
}

fn main() -> std::io::Result<()> {
    let path: &Path = "resources/protos/".as_ref();
    let files = collect_proto_files(path.read_dir()?)?;
    println!("Found proto files for compilation:\n{:?}", files);
    prost_build::compile_protos(files.as_slice(), &["resources/protos/"])?;
    let modules = copy_into_src()?;
    let content = generate_lib_file(modules);
    fs::write("src/lib.rs", content)?;
    Ok(())
}
