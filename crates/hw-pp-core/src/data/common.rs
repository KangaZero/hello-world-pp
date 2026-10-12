#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumanLanguageCategory {
    Natural,
    Constructed,
    Signed,
    Extinct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgrammingLanguageCategory {
    Esolang,
    Scripting,
    Compiled,
    Markup,
    Historical,
}
