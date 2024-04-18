fn main() {
    cynic_codegen::register_schema("MRE")
        .from_sdl_file("./src/schema.graphql".to_string())
        .expect("Failed to find MRE GraphQL Schema")
        .as_default()
        .expect("Failed to register default schema for MRE");
}
