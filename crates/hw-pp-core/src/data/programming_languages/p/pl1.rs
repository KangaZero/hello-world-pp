//! Hello-world snippet for `ProgrammingLanguageName::Pl1`.
//!
//! Imported from the Hello World Collection (`files/PL1` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/PL1>
//! - <http://helloworldcollection.de/#PL1>
//! - <https://en.wikipedia.org/wiki/PL/I> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Procedural, Structured};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Pl1 => "PL1",
    versions: [Pl1 => "PL1"],
    extensions: [],
}

const PL1: &str = concat!(
    r"/* ",
    output_for_hello_world!(),
    r" in PL/1 */

Hello: procedure options(main);
       put skip list('",
    output_for_hello_world!(),
    r"');
end Hello;"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pl1,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, Procedural, Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pl1,
            syntax: PL1,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PL1,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"/* ",
                            output_for_hello_world!(),
                            r" in PL/1 */

Hello: procedure options(main);
       put skip list('",
                            output_for_hello_world!(),
                            r"');
end Hello;"
                        ),
                    )],
                )
            },
        }],
    };
