//! Hello-world snippet for `ProgrammingLanguageName::Chaiscript`.
//!
//! Imported from the Hello World Collection (`files/ChaiScript.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ChaiScript.htm>
//! - <http://helloworldcollection.de/#ChaiScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Chaiscript => "ChaiScript",
    versions: [Chaiscript => "ChaiScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Chaiscript,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Chaiscript,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in ChaiScript

#include <chaiscript/chaiscript.hpp>

std::string helloWorld(const std::string &t_name) {
  return "Hello " + t_name + "!";
}

int main() {
  chaiscript::ChaiScript chai;
  chai.add(chaiscript::fun(&helloWorld), "helloWorld");

  chai.eval(R"(
    puts(helloWorld("World"));
  )");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in ChaiScript

#include <chaiscript/chaiscript.hpp>

std::string helloWorld(const std::string &t_name) {
  return "Hello " + t_name + "!";
}

int main() {
  chaiscript::ChaiScript chai;
  chai.add(chaiscript::fun(&helloWorld), "helloWorld");

  chai.eval(R"(
    puts(helloWorld("World"));
  )");
}"#
                ),
            )],
        }],
    };
