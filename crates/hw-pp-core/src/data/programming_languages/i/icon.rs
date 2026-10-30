//! Hello-world snippet for `ProgrammingLanguageName::Icon`.
//!
//! Imported from leachim6/hello-world (`i/Icon.icn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/Icon.icn>
//! - <https://www.wikidata.org/wiki/Q1156474> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Procedural, Structured};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Historical, Scripting},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{Icon, Snobol},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Icon => "Icon",
    versions: [Icon => "Icon"],
    extensions: [Icn => "icn"],
}

const ICON: &str = concat!(
    r#"procedure main()
    write(""#,
    output_for_hello_world!(),
    r#"");
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Icon,
        extensions: &[FileExtension::Icn],
        categories: &[Scripting, Historical],
        families: &[Icon, Snobol],
        paradigms: &[Procedural, Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Icon,
            syntax: ICON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ICON,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"procedure main()
    write(""#,
                            output_for_hello_world!(),
                            r#"");
end"#
                        ),
                    )],
                )
            },
        }],
    };
