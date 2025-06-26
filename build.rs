extern crate embed_resource;

fn main() {
    let _ = embed_resource::compile("resources/resources.rc", embed_resource::NONE);
}