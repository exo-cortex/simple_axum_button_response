use argh::FromArgs;

fn default_tls_directory() -> String {
    String::from("./")
}

fn default_port() -> u16 {
    4321
}

#[derive(FromArgs)]
/// Config
pub struct Arguments {
    /// path to `index.html`
    #[argh(
        option,
        short = 'f',
        default = "String::from(\"/home/nt_user/html/index.html\")"
    )]
    pub html_folder: String,
    /// listening port
    #[argh(option, short = 'p', default = "default_port()")]
    pub portnumber: u16,
    /// do not use tls
    #[argh(switch)]
    pub no_tls: bool,
    /// tls files directory
    #[argh(option, short = 't', default = "default_tls_directory()")]
    pub tls_dir: String,
}
