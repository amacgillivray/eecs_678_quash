use logos::Logos;

#[derive(Logos, PartialEq)]
pub enum Token {

    #[regex("&")]
    AMPERSAND,

    #[regex("<")]
    LANGLE,

    #[regex(">")]
    RANGLE,

    #[regex(">>")]
    DRANGLE,

    #[token("[|]")]
    PIPE,

    // Commands

    #[regex("echo")]
    Echo,

    #[regex("export")]
    Export,

    #[regex("cd")]
    CD,

    #[regex("pwd")]
    PWD,

    #[regex("quit|exit")]
    Quit,

    #[regex("kill")]
    Kill,

    #[regex("jobs")]
    Jobs,

    // Misc

    #[regex("#.+\n", logos::skip)]
    Comment,

    #[regex("[^ \n\r\t]+")]
    Text,

    #[regex("[ \n\t\r]+", logos::skip)]
    Whitespace,

    #[error]
    Error
}