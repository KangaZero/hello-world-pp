//! Hello-world snippet for `ProgrammingLanguageName::L`.
//!
//! Imported from the Hello World Collection (`files/L.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/L.htm>
//! - <http://helloworldcollection.de/#L>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: L => "L",
    versions: [L => "L"],
    extensions: [],
}

const L: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in L

namespace Hello is
    class Main is
        void init is()
            IO.Std.out.println(""#,
    output_for_hello_world!(),
    r#"");
        end
    end
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::L,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::L,
            syntax: L,
            tokenized_syntax: &const {
                checked_token_lengths(
                    L,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r#" in L

namespace Hello is
    class Main is
        void init is()
            IO.Std.out.println(""#,
                            output_for_hello_world!(),
                            r#"");
        end
    end
end"#
                        ),
                    )],
                )
            },
        }],
    };
