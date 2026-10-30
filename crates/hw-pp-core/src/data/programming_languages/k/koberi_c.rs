//! Hello-world snippet for `ProgrammingLanguageName::KoberiC`.
//!
//! Imported from leachim6/hello-world (`k/Kobeři-C.koberice` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/Kobe%C5%99i-C.koberice>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: KoberiC => "Kobeři-C",
    versions: [KoberiC => "Kobeři-C"],
    extensions: [Koberice => "koberice"],
}

const KOBERI_C: &str = concat!(
    r#"(int main()
    (print ""#,
    output_for_hello_world!(),
    r#"")
    (return 0))"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::KoberiC,
        extensions: &[FileExtension::Koberice],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::KoberiC,
            syntax: KOBERI_C,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KOBERI_C,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"(int main()
    (print ""#,
                            output_for_hello_world!(),
                            r#"")
    (return 0))"#
                        ),
                    )],
                )
            },
        }],
    };
