//! Hello-world snippet for `ProgrammingLanguageName::AlgolW`.
//!
//! Imported from leachim6/hello-world (`a/ALGOL W.algol` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ALGOL%20W.algol>
//! - <https://www.wikidata.org/wiki/Q1538458> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Procedural, Structured};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: AlgolW => "ALGOL W",
    versions: [AlgolW => "ALGOL W"],
    extensions: [Algol => "algol"],
}

const ALGOL_W: &str = concat!(
    r#"begin
    write( ""#,
    output_for_hello_world!(),
    r#"" )
end."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AlgolW,
        extensions: &[FileExtension::Algol],
        categories: &[Compiled, Historical],
        families: &[],
        paradigms: &[Imperative, Procedural, Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AlgolW,
            syntax: ALGOL_W,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ALGOL_W,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"begin
    write( ""#,
                            output_for_hello_world!(),
                            r#"" )
end."#
                        ),
                    )],
                )
            },
        }],
    };
