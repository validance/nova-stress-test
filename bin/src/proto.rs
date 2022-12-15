use std::env;
use std::fs;
use std::path::Path;

pub fn build_proto(target_path: &str, out_dir: &str) {
    env::set_var("OUT_DIR", out_dir);

    if Path::new(out_dir).exists() {
        fs::remove_dir_all(out_dir).expect("cannot remove dir");
    }

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

    prost_build::compile_protos(
        &files,
        &[
            "/Users/sangyun/Documents/GolandProjects/cosmos-sdk/proto",
            "/Users/sangyun/Documents/GolandProjects/cosmos-proto/proto",
            "/Users/sangyun/Documents/GolandProjects/gogoproto",
            "/Users/sangyun/Documents/GolandProjects/googleapis",
            "/Users/sangyun/Documents/GolandProjects/nova/proto",
        ],
    )
    .expect("prost build failed");
}
