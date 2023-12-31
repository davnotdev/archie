use argh::FromArgs;

#[derive(FromArgs)]
/// Archie is a simple CLI tool that allows you to archive infrequently used folders effortlessly
/// and intelligently.
///
/// Archie knows to remove unneeded files from the final archive: npm packages, cargo packages,
/// etc.
///
/// Find out more at https://github.com/davnotdev/archie
pub struct Actions {
    #[argh(subcommand)]
    /// actions
    pub command: SubActions,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum SubActions {
    Push(PushAction),
    Pull(PullAction),
    List(ListAction),
    DefaultConfig(DefaultConfigAction),
}

#[derive(FromArgs)]
/// push a directory away
#[argh(subcommand, name = "push")]
pub struct PushAction {
    #[argh(positional)]
    /// which directory to push
    pub target: String,
}

#[derive(FromArgs)]
/// pull a directory back
#[argh(subcommand, name = "pull")]
pub struct PullAction {
    #[argh(positional)]
    /// which directory to pull
    pub target: String,
}

#[derive(FromArgs)]
/// list the directories that have been pushed away
#[argh(subcommand, name = "list")]
pub struct ListAction {}

#[derive(FromArgs)]
/// print out the default configuration
#[argh(subcommand, name = "default-config")]
pub struct DefaultConfigAction {}
