//! Hello-world snippet for `ProgrammingLanguageName::Idk`.
//!
//! Imported from leachim6/hello-world (`i/IDK.idk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/IDK.idk>
//! - <https://esolangs.org/wiki/IDK> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Idk => "IDK",
    versions: [Idk => "IDK"],
    extensions: [Idk => "idk"],
}

const IDK: &str = concat!(
    r"openC
movLoc
2
extract
movVar
",
    output_for_hello_world!(),
    r"
execute
closeC"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Idk,
        extensions: &[FileExtension::Idk],
        categories: &[],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Idk,
            syntax: IDK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    IDK,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"openC
movLoc
2
extract
movVar
",
                            output_for_hello_world!(),
                            r"
execute
closeC"
                        ),
                    )],
                )
            },
        }],
    };
