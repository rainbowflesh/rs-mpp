extern crate bindgen;
use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    let inc_dir = "../../inc/";
    let gen_dir = "../../gen/";
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    for i in fs::read_dir(inc_dir).unwrap() {
        let file = i?;
        let header_path = file.path().clone();
        let gen_path = [
            gen_dir,
            &file.file_name().clone().into_string().unwrap(),
            ".rs",
        ]
        .concat();
        println!("generating rust code from c/cpp in {:?}", header_path);
        let bindings = bindgen::Builder::default()
            .header(header_path.into_os_string().into_string().unwrap())
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");
        println!("overwrite rust code in {}.rs", gen_path.clone());
        bindings.write_to_file(gen_path)?;
        println!("overwrite rust code in {:?}.rs", out_path.clone());
        bindings.write_to_file(
            out_path
                .clone()
                .join(&file.file_name().clone().into_string().unwrap()),
        )?;
    }
    Ok(())
}
