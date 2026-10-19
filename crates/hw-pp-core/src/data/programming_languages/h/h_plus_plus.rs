//! Hello-world snippet for `ProgrammingLanguageName::HPlusPlus`.
//!
//! Imported from leachim6/hello-world (`h/H++.hpp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: HPlusPlus => "H++",
    versions: [HPlusPlus => "H++"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::HPlusPlus,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::HPlusPlus,
            syntax: concat!(
                r#"import "stdhpp\stdapi.hcc";

using printf = Console::WriteLn;

namespace
{
    class Application
    {
        public static void main()
        {
            printf(""#,
                output_for_hello_world!(),
                r#"");
            return;
        }
    };
};"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"import "stdhpp\stdapi.hcc";

using printf = Console::WriteLn;

namespace
{
    class Application
    {
        public static void main()
        {
            printf(""#,
                    output_for_hello_world!(),
                    r#"");
            return;
        }
    };
};"#
                ),
            )],
        }],
    };
