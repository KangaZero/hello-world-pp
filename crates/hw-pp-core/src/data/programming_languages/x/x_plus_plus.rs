//! Hello-world snippet for `ProgrammingLanguageName::XPlusPlus`.
//!
//! Imported from leachim6/hello-world (`x/X++.xpp` at `a152253`),
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
    language: XPlusPlus => "X++",
    versions: [XPlusPlus => "X++"],
    extensions: [Xpp => "xpp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::XPlusPlus,
        extensions: &[FileExtension::Xpp],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::XPlusPlus,
            syntax: concat!(
                r#"class classHello
{
}

static void main(args Args)
{
 dialog dialog;
 dialog = new dialog();
 dialog.addText(""#,
                output_for_hello_world!(),
                r#"");
 dialog.run();
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"class classHello
{
}

static void main(args Args)
{
 dialog dialog;
 dialog = new dialog();
 dialog.addText(""#,
                    output_for_hello_world!(),
                    r#"");
 dialog.run();
}"#
                ),
            )],
        }],
    };
