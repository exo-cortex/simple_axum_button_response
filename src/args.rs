use argh::FromArgs;

fn default_portnumber() -> u32 {
    4321
}

#[derive(FromArgs)]
/// Setup
pub struct Arguments {
    /// folder of `index.html`
    #[argh(
        option,
        short = 'f',
        default = "String::from(\"/home/nt_user/html/index.html\")"
    )]
    pub html_folder: String,
    /// listening port
    #[argh(option, short = 'p', default = "default_portnumber()")]
    pub portnumber: u32,
}
