fn main() {
    println!("{:?}", protocol::index::file_descriptor_proto());
    println!("{:?}", protocol::category::file_descriptor_proto());
    println!("{:?}", protocol::verb::file_descriptor_proto());
}
