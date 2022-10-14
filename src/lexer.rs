use logos::Logos;

#[derive(Logos, PartialEq, Debug)]
pub enum Dictionary {

    // Text

    #[regex("[^ \n\r\t]+")] // arguments, or executable
    Text,

    // Commands

    #[regex("echo")]  // Echos the argument to terminal
    Echo,

    #[regex("export")] // Set val of environment variable
    Export,

    #[regex("cd")] // Change directory
    CD,

    #[regex("pwd")] // Echo current directory name
    PWD,

    #[regex("quit|exit")] // Exit
    Quit,

    #[regex("kill")] // Kill a process
    Kill,

    #[regex("jobs")] // Show all active jobs in quash
    Jobs,

    // Operators

    #[regex("<")] // Take input from file
    LANGLE,

    #[regex(">")] // Write to file
    RANGLE,

    #[regex(">>")] // Append to file
    DRANGLE,

    #[token("[|]")] // Pipes output of one to input of next
    PIPE,

    #[regex("&")] // Run in background
    AMPERSAND,

    // Skipped / Error

    #[regex("#.+\n", logos::skip)] // Skip line
    Comment,

    #[regex("[ \n\t\r]+", logos::skip)] // Skip ws
    Whitespace,

    #[error] // Oops! Bad syntax
    Error
}