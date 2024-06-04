use argh::FromArgs;

fn default_portnumber() -> u32 {
    4321
}

fn default_tls_directory() -> String {
    String::from("./")
}

#[derive(FromArgs)]
/// Setup
pub struct Arguments {
    /// path to `index.html`
    #[argh(
        option,
        short = 'f',
        default = "String::from(\"/home/nt_user/html/index.html\")"
    )]
    pub html_folder: String,
    /// listening port
    #[argh(option, short = 'p', default = "default_portnumber()")]
    pub portnumber: u32,
    /// tls files directory
    #[argh(option, short = 't', default = "default_tls_directory()")]
    pub tls_directory: String,
}
