extern crate protobuf_codegen_pure;
fn main() {
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/",
        input: &["proto_defs/verb.proto","proto_defs/index.proto","proto_defs/category.proto"],
        includes: &["proto_defs"],
        customize: protobuf_codegen_pure::Customize {
        ..Default::default()
        },
    }).expect("protoc");
}