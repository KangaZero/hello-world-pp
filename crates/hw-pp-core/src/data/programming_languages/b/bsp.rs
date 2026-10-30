//! Hello-world snippet for `ProgrammingLanguageName::Bsp`.
//!
//! Imported from the Hello World Collection (`files/BSP.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/BSP.htm>
//! - <http://helloworldcollection.de/#BSP>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Bsp => "BSP",
    versions: [Bsp => "BSP"],
    extensions: [],
}

const BSP: &str = concat!(
    r"<!— ",
    output_for_hello_world!(),
    r#" in  SAP Business Server Pages (BSP)
<%@page language="abap"%>
<%@extension name="htmlb" prefix="htmlb"%>
<html><body>
<%
  CONSTANTS lv_hworld TYPE string VALUE `"#,
    output_for_hello_world!(),
    r#"`.
%>
<%=lv_hworld%>">
</body>
</html>"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bsp,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bsp,
            syntax: BSP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BSP,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"<!— ",
                            output_for_hello_world!(),
                            r#" in  SAP Business Server Pages (BSP)
<%@page language="abap"%>
<%@extension name="htmlb" prefix="htmlb"%>
<html><body>
<%
  CONSTANTS lv_hworld TYPE string VALUE `"#,
                            output_for_hello_world!(),
                            r#"`.
%>
<%=lv_hworld%>">
</body>
</html>"#
                        ),
                    )],
                )
            },
        }],
    };
