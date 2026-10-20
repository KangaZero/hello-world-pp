//! Hello-world snippet for `ProgrammingLanguageName::Examplefuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#ExampleFuck>
//! - <https://esolangs.org/wiki/ExampleFuck>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Examplefuck => "ExampleFuck",
    versions: [Examplefuck => "ExampleFuck"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Examplefuck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Examplefuck,
            syntax: concat!(
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
FizzBuzz
Truth Machine
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
FizzBuzz
Truth Machine
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
Truth Machine
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
Truth Machine
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
Truth Machine
",
                output_for_hello_world!(),
                r"
Quine
Quine
Quine
Quine
Cat Program
99 Bottles of Beer
Truth Machine
",
                output_for_hello_world!(),
                r"
Truth Machine
",
                output_for_hello_world!(),
                r"
Truth Machine
Cat Program
Truth Machine
Truth Machine
",
                output_for_hello_world!(),
                r"
FizzBuzz
Quine
99 Bottles of Beer
Quine
Cat Program
99 Bottles of Beer
Truth Machine
Truth Machine
Self Interpreter
Truth Machine
Cat Program
Cat Program
Cat Program
Self Interpreter
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
Self Interpreter
Self Interpreter
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
Self Interpreter
Truth Machine
Truth Machine
Self Interpreter
Quine
Cat Program
Self Interpreter
Quine
Self Interpreter
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
Self Interpreter
Cat Program
Cat Program
Cat Program
Cat Program
Cat Program
Cat Program
Self Interpreter
Cat Program
Cat Program
Cat Program
Cat Program
Cat Program
Cat Program
Cat Program
Cat Program
Self Interpreter
Truth Machine
Truth Machine
",
                output_for_hello_world!(),
                r"
Self Interpreter
Truth Machine
",
                output_for_hello_world!(),
                r"
",
                output_for_hello_world!(),
                r"
Self Interpreter"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
FizzBuzz
Truth Machine
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
FizzBuzz
Truth Machine
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
Truth Machine
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
Truth Machine
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
Truth Machine
",
                    output_for_hello_world!(),
                    r"
Quine
Quine
Quine
Quine
Cat Program
99 Bottles of Beer
Truth Machine
",
                    output_for_hello_world!(),
                    r"
Truth Machine
",
                    output_for_hello_world!(),
                    r"
Truth Machine
Cat Program
Truth Machine
Truth Machine
",
                    output_for_hello_world!(),
                    r"
FizzBuzz
Quine
99 Bottles of Beer
Quine
Cat Program
99 Bottles of Beer
Truth Machine
Truth Machine
Self Interpreter
Truth Machine
Cat Program
Cat Program
Cat Program
Self Interpreter
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
Self Interpreter
Self Interpreter
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
Self Interpreter
Truth Machine
Truth Machine
Self Interpreter
Quine
Cat Program
Self Interpreter
Quine
Self Interpreter
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
Self Interpreter
Cat Program
Cat Program
Cat Program
Cat Program
Cat Program
Cat Program
Self Interpreter
Cat Program
Cat Program
Cat Program
Cat Program
Cat Program
Cat Program
Cat Program
Cat Program
Self Interpreter
Truth Machine
Truth Machine
",
                    output_for_hello_world!(),
                    r"
Self Interpreter
Truth Machine
",
                    output_for_hello_world!(),
                    r"
",
                    output_for_hello_world!(),
                    r"
Self Interpreter"
                ),
            )],
        }],
    };
