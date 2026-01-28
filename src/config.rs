use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "onesource", author = "lolLeo", version = "1.0")]
pub struct Args {
    // File setting
    #[arg(default_value = ".", help = "Set location")]
    pub path: PathBuf,
    #[arg(short, long, default_value = "allCode.txt")]
    pub output_path: PathBuf,

    // Content setting
    #[arg(long, action = clap::ArgAction::SetTrue, help = "Don't use .gitignore")]
    pub no_ignore: bool,
    #[arg(short, long, default_value = "*")]
    pub include: String,
    #[arg(short = 'x', long, default_value = "")]
    pub exclude: String,

    // Tree setting
    #[arg(long, visible_alias = "ti")]
    pub tree_include: Option<String>,
    #[arg(long, visible_alias = "tx")]
    pub tree_exclude: Option<String>,
    #[arg(long, action = clap::ArgAction::SetTrue, help = "Don't write tree")]
    pub no_tree: bool,
    #[arg(long, action = clap::ArgAction::SetTrue, help = "Tree don't use .gitignore")]
    pub tree_no_ignore: bool,

    // Behavior setting
    #[arg(long, action = clap::ArgAction::SetTrue, help = "Preview file list without writing to disk")]
    pub dry_run: bool,
}