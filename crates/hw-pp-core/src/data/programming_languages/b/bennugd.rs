//! Hello-world snippet for `ProgrammingLanguageName::Bennugd`.
//!
//! Imported from leachim6/hello-world (`b/BennuGD` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BennuGD>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Bennugd => "BennuGD",
    versions: [Bennugd => "BennuGD"],
    extensions: [],
}

const BENNUGD: &str = concat!(
    r#"import "mod_say" 
 
Process Main()
Begin 
    say(""#,
    output_for_hello_world!(),
    r#"");
End"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bennugd,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bennugd,
            syntax: BENNUGD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BENNUGD,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"import "mod_say" 
 
Process Main()
Begin 
    say(""#,
                            output_for_hello_world!(),
                            r#"");
End"#
                        ),
                    )],
                )
            },
        }],
    };
