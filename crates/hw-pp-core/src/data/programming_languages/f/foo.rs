//! Hello-world snippet for `ProgrammingLanguageName::Foo`.
//!
//! Imported from leachim6/hello-world (`f/Foo.foo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Foo.foo>
//! - <https://esolangs.org/wiki/Foo> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Foo => "Foo",
    versions: [Foo => "Foo"],
    extensions: [Foo => "foo"],
}

const FOO: &str = concat!(r#"""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Foo,
        extensions: &[FileExtension::Foo],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Foo,
            syntax: FOO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FOO,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
