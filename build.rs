use heck::CamelCase;
use prost::Message;
use prost_types::FileDescriptorSet;
use quote::{format_ident, quote};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    prost_build::Config::new()
        .file_descriptor_set_path(out_dir.join("file_descriptor_set.bin"))
        .compile_protos(&["proto/foo.proto"], &["proto"])
        .unwrap();

    let file_descriptor_set_bytes =
        fs::read(out_dir.join("file_descriptor_set.bin")).unwrap();
    let file_descriptor_set =
        FileDescriptorSet::decode(&file_descriptor_set_bytes[..]).unwrap();

    for fd in &file_descriptor_set.file {
        let package = match fd.package {
            Some(ref pkg) => pkg,
            None => continue,
        };

        if package.starts_with("google.") {
            continue;
        }

        let gen_path = out_dir.join(format!("{}.rs", package));
        let mut gen_file =
            OpenOptions::new().append(true).open(gen_path).unwrap();

        for msg in &fd.message_type {
            let name = match msg.name {
                Some(ref name) => name,
                None => continue,
            };

            let type_url = format!("type.googleapis.com/{}.{}", package, name);
            let type_name = name.to_camel_case();

            gen_type_url(&mut gen_file, &type_url, &type_name);
        }
    }
}

fn gen_type_url(gen_file: &mut File, type_url: &str, type_name: &str) {
    let type_name = format_ident!("{}", type_name);

    let tokens = quote! {
        impl crate::TypeUrl for #type_name {
            fn type_url() -> &'static str {
                #type_url
            }
        }
    };

    writeln!(gen_file).unwrap();
    writeln!(gen_file, "{}", &tokens).unwrap();
}
