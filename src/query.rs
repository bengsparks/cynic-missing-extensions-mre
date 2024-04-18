use crate::schema::MRE as schema;
use cynic; // Import for derive macros // Rename is vital! Must import as schema!

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
pub struct HelloWorld {
    pub hello_world: String,
}
