//! Hello-world snippet for `ProgrammingLanguageName::TuringMachine`.
//!
//! Imported from the Hello World Collection (`files/Turing&nbsp;Machine` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Turing%26nbsp%3BMachine>
//! - <http://helloworldcollection.de/#Turing%C2%A0Machine>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: TuringMachine => "Turing Machine",
    versions: [TuringMachine => "Turing Machine"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TuringMachine,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TuringMachine,
            syntax: concat!(
                output_for_hello_world!(),
                r" as a Turing machine.

State   Read   |   Write     Step    Next state
---------------|---------------------------------
1       empty  |   H         >       2
2       empty  |   e         >       3
3       empty  |   l         >       4
4       empty  |   l         >       5
5       empty  |   o         >       6
6       empty  |   blank     >       7
7       empty  |   W         >       8
8       empty  |   o         >       9
9       empty  |   r         >       10
10      empty  |   l         >       11
11      empty  |   d         >       12
12      empty  |   !         >       STOP"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    output_for_hello_world!(),
                    r" as a Turing machine.

State   Read   |   Write     Step    Next state
---------------|---------------------------------
1       empty  |   H         >       2
2       empty  |   e         >       3
3       empty  |   l         >       4
4       empty  |   l         >       5
5       empty  |   o         >       6
6       empty  |   blank     >       7
7       empty  |   W         >       8
8       empty  |   o         >       9
9       empty  |   r         >       10
10      empty  |   l         >       11
11      empty  |   d         >       12
12      empty  |   !         >       STOP"
                ),
            )],
        }],
    };
