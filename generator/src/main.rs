extern crate flate2;
extern crate heck;
extern crate itertools;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::{io::Read, path::Path};

use generatable::Generatable;
use serde::Deserialize;

mod codegen;
pub(crate) mod generatable;
mod model;

fn generate<G, P>(bytes: &[u8], output_path: P)
where
    for<'de> G: Generatable + Deserialize<'de>,
    P: AsRef<Path>,
{
    let mut data = String::new();
    let mut gz = flate2::read::GzDecoder::new(&bytes[..]);
    gz.read_to_string(&mut data)
        .expect("failed to decompress specification file");
    let specification =
        serde_json::from_str::<G>(&data).expect("failed to parse specification data");
    codegen::generate(specification, output_path).expect("failed to generate output files");
}

fn main() {
    let bytes = include_bytes!("../CloudFormationResourceSpecification.json.gz");
    generate::<model::Specification, _>(bytes, "cfn/src/aws");
}
