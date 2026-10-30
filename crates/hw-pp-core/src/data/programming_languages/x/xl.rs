//! Hello-world snippet for `ProgrammingLanguageName::Xl`.
//!
//! Imported from leachim6/hello-world (`x/XL.xl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XL.xl>
//! - <https://www.wikidata.org/wiki/Q1859276> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, Imperative};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Xl => "XL",
    versions: [Xl => "XL"],
    extensions: [Xl => "xl"],
}

const XL: &str = concat!(
    r#"// XL programming language: http://xlr.sourceforge.net
writeln ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xl,
        extensions: &[FileExtension::Xl],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xl,
            syntax: XL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"// XL programming language: http://xlr.sourceforge.net
writeln ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
