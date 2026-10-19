//! Hello-world snippet for `ProgrammingLanguageName::HolyC`.
//!
//! Imported from leachim6/hello-world (`h/Holy C.HC` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: HolyC => "Holy C",
    versions: [HolyC => "Holy C"],
    extensions: [Hc => "HC"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::HolyC,
        extensions: &[FileExtension::Hc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::HolyC,
            syntax: concat!(
                r#"U0 main() {
    print(""#,
                output_for_hello_world!(),
                r#"\n");
    return 0;
}

main();"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"U0 main() {
    print(""#,
                    output_for_hello_world!(),
                    r#"\n");
    return 0;
}

main();"#
                ),
            )],
        }],
    };
