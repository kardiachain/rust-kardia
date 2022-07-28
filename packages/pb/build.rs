use prost_build::compile_protos;
use walkdir::WalkDir;

fn main() {
    let mut pb_filepaths: Vec<String> = Vec::new();

    for entry in WalkDir::new("./src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "proto" {
                pb_filepaths.push(String::from(path.to_str().unwrap()));
            }
        }
    }

    compile_protos(&pb_filepaths, &["src"]).unwrap();
}
