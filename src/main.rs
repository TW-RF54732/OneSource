use clap::{Parser, builder, error};
use std::{ path::PathBuf};
use ignore::{Walk, WalkBuilder};

#[derive(Parser,Debug)]
#[command(name = "onesource", author = "lolLeo", version = "1.0")]
struct Args{
    #[arg(default_value = ".",help = "Set location")]
    path :PathBuf,

    #[arg(long,action = clap::ArgAction::SetTrue,help="Don't use .gitignore")]
    no_ignore:bool,

    #[arg(short,long,default_value="allCode")]
    output_name:String,
    #[arg(short,long,default_value="txt")]
    extension:String,
}

fn main() {
    let args = Args::parse();
    debug_log(&args);
    let walker = WalkBuilder::new(&args.path)
        .standard_filters(!args.no_ignore)
        .build();

    for result in walker{
        match result {
            Ok(entry)=>{
                print!("{:#?}",entry)
            }
            Err(error)=>{
                print!("{}",error)
            }
        }
    }
}

fn debug_log(args:&Args){
    println!("======ARGS======");
    println!("Target path: {:#?}",args);    
    println!("======Others======")    
}
