//! Hello-world snippet for `ProgrammingLanguageName::HPlusPlus`.
//!
//! Imported from leachim6/hello-world (`h/H++.hpp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/H%2B%2B.hpp>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: HPlusPlus => "H++",
    versions: [HPlusPlus => "H++"],
    extensions: [],
}

const H_PLUS_PLUS: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::HPlusPlus,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::HPlusPlus,
            syntax: H_PLUS_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    H_PLUS_PLUS,
                    [(
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
                )
            },
        }],
    };
