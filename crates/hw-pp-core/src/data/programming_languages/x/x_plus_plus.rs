//! Hello-world snippet for `ProgrammingLanguageName::XPlusPlus`.
//!
//! Imported from leachim6/hello-world (`x/X++.xpp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/X%2B%2B.xpp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Esolang],
        families: &[],
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
