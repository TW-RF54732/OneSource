use clap::{Parser, builder, error};
use std::{ path::PathBuf};
use ignore::{Walk, WalkBuilder};
mod tree;

#[derive(Parser,Debug)]
#[command(name = "onesource", author = "lolLeo", version = "1.0")]
struct Args{
    // File setting
    #[arg(default_value = ".",help = "Set location")]//Target path
    path :PathBuf,
    #[arg(short,long,default_value="allCode")]//Output file name
    output_name:String,
    #[arg(short,long,default_value="txt")]//Output file extension
    extension:String,
    
    //File config
    #[arg(long,action = clap::ArgAction::SetTrue,help="Don't use .gitignore")]
    ignore:bool,
    
    //Tree config
    #[arg(long,action = clap::ArgAction::SetTrue,help="Don't write tree")]
    tree:bool,
    #[arg(long,action = clap::ArgAction::SetTrue,help="Tree don't use .gitignore")]
    tree_ignore:bool,
    
}
fn main() {
    let args = Args::parse();
    debug_log(&args);
    let root_name = args.path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| ".".into());
    
    let mut tree_root = tree::Node::new(root_name, true);

    let walker = WalkBuilder::new(&args.path)
        .standard_filters(true)
        .build();

    for result in walker{
        match result {
            Ok(entry)=>{
                // let depth = entry.depth();
                // let name = entry.file_name().to_string_lossy().to_string();
                let rel_path = entry.path().strip_prefix(&args.path).unwrap_or(entry.path());
                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                tree_root.insert_path(rel_path, is_dir);
            }
            Err(error)=>{
                print!("{}",error)
            }
        }
    }
    println!("{}/", tree_root.name);
    tree_root.print("");

}

fn debug_log(args:&Args){
    println!("======ARGS======");
    println!("Target path: {:#?}",args);    
    println!("======Others======")    
}
