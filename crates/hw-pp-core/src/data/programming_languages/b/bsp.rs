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
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Bsp => "BSP",
    versions: [Bsp => "BSP"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bsp,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bsp,
            syntax: concat!(
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
            tokenized_syntax: &[(
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
        }],
    };
