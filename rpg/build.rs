use embed_resource;
use std::env;
use std::fs;
use std::path;

fn copy_i18n_dir() {
    let src = path::Path::new(&env::current_dir().unwrap()).join("i18n");
    let dest = path::Path::new(&env::var("OUT_DIR").unwrap())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("i18n");
    if dest.exists() {
        fs::remove_dir_all(&dest).unwrap();
    }
    fs::create_dir(&dest).unwrap();
    for entry in fs::read_dir(&src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = path.file_name().unwrap();
        fs::copy(&path, dest.join(filename)).unwrap();
    }
}

fn main() {
    embed_resource::compile("./resources.rc");
    copy_i18n_dir();
}
