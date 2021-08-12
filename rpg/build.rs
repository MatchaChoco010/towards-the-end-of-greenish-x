use embed_resource;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    embed_resource::compile("./resources.rc");
}
