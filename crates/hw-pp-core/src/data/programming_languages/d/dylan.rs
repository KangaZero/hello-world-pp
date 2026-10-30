//! Hello-world snippet for `ProgrammingLanguageName::Dylan`.
//!
//! Imported from leachim6/hello-world (`d/Dylan.dl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Dylan.dl>
//! - <https://www.wikidata.org/wiki/Q1268744> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, ObjectOriented};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Lisp, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dylan => "Dylan",
    versions: [Dylan => "Dylan"],
    extensions: [],
}

const DYLAN: &str = concat!(
    r#"define method main (#rest args)
  princ(""#,
    output_for_hello_world!(),
    r#"");
end;

main();"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dylan,
        extensions: &[],
        categories: &[Compiled],
        families: &[Lisp],
        paradigms: &[ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dylan,
            syntax: DYLAN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DYLAN,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"define method main (#rest args)
  princ(""#,
                            output_for_hello_world!(),
                            r#"");
end;

main();"#
                        ),
                    )],
                )
            },
        }],
    };
