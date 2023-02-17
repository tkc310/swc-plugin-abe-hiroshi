use super::TransformVisitor;
use std::path::PathBuf;
use swc_ecma_transforms_testing::{test, test_fixture, FixtureTestConfig};
use swc_ecma_parser::{TsConfig, Syntax};
use swc_core::ecma::visit::{Fold, as_folder};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Typescript(TsConfig {
        tsx: true,
        ..Default::default()
    })
}

fn transformer() -> impl Fold {
    as_folder(TransformVisitor)
}

fn config() -> FixtureTestConfig {
    FixtureTestConfig {
        sourcemap: false,
        allow_error: false
    }
}

#[fixture("fixture/input.ts")]
fn it_replace_value(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|_tr| transformer(),
        &input,
        &output,
        config()
    );
}
