use std::env;
use std::fs;

pub fn build_proto(target_path: &str, out_dir: &str, include_dir: &[String]) {
    env::set_var("OUT_DIR", out_dir);
    fs::create_dir_all(out_dir).expect("cannot create dir");

    let paths_raw = fs::read_dir(target_path).expect("path not valid");
    let files = paths_raw
        .map(|path| {
            path.unwrap()
                .path()
                .to_str()
                .expect("path error")
                .to_string()
        })
        .collect::<Vec<String>>();

    prost_build::compile_protos(&files, include_dir).expect("prost build failed");
}
