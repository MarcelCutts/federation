use std::fs::{read_dir, write};

fn main() -> std::io::Result<()> {
  let mut output = String::from("// Autogenerated by graphql-parser/build.rs
//
// This file is autogenerated by scanning the tests for *.graphql files.
// To add a sample to the test corpus, don't edit this file. Instead, just
// add a new .graphql file to the tests/ directory.  

// Sample names are easier to read with two underscores in them
#![allow(non_snake_case)]

mod helpers;
use insta::assert_debug_snapshot;
use pretty_assertions::assert_eq;
use graphql_parser::parse_query;
use graphql_parser::parse_schema;

\n\n");

  for ent in read_dir("tests")? {
    let entry = ent?;
    let mut path = entry.path();
    if let (Some(osname), Some(ext)) = (path.file_stem(), path.extension()) {
      let name = osname.to_str().unwrap().to_owned();
      if name.contains(".") || ext != "graphql" { continue }
      let src = entry.file_name().to_str().unwrap().to_owned();
      let canonical_src = format!("{}.canonical.graphql", name);
      path.pop();
      path.push(canonical_src.clone());
      output.push_str(
        &format!(
          "test!({}, include_str!(\"{}\"), include_str!(\"{}\"));\n",
          &name, &src, if path.is_file() { &canonical_src } else { &src }
        )
      )      
    }
  }
  write("tests/tests.rs", output)?;
  Ok(())
}
