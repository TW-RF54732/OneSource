use clap::{Parser};
use std::{fs::File,path::PathBuf};
use ignore::WalkBuilder;
use std::io::{BufWriter,Write};

mod tree_utils;
mod io_utils;
mod filter_utils;

#[derive(Parser,Debug)]
#[command(name = "onesource", author = "lolLeo", version = "1.0")]
struct Args{
    // File setting
    #[arg(default_value = ".",help = "Set location")]//Target path
    path :PathBuf,
    #[arg(short,long,default_value="allCode.txt")]//Output file name
    output_path:PathBuf,
    #[arg(short,long,default_value="*")]
    include:String,
    #[arg(short='x',long,default_value="")]
    exclude:String,
    
    //content setting
    #[arg(long,action = clap::ArgAction::SetTrue,help="Don't use .gitignore")]
    no_ignore:bool,
    
    
    //Tree setting
    #[arg(long,visible_alias="ti")]
    tree_include: Option<String>,
    #[arg(long,visible_alias="tx")]
    tree_exclude: Option<String>,
    #[arg(long,action = clap::ArgAction::SetTrue,help="Don't write tree")]
    no_tree:bool,
    #[arg(long,action = clap::ArgAction::SetTrue,help="Tree don't use .gitignore")]
    tree_no_ignore:bool,
    
}
fn struct_tree<W: Write>(args:&Args,writer: &mut W){
    let final_include = args.tree_include.as_deref().unwrap_or(&args.include);
    let final_exclude = args.tree_exclude.as_deref().unwrap_or(&args.exclude);
    let filter = filter_utils::FileFilter::new(final_include, final_exclude);
    let mut tree_root = tree_utils::Node::new(true);
    
    let walker = WalkBuilder::new(&args.path)
        .standard_filters(!args.tree_no_ignore)
        .build();
    
    for result in walker{
        match result {
            Ok(entry)=>{
                let rel_path = entry.path().strip_prefix(&args.path).unwrap_or(entry.path());
                if !filter.is_match(rel_path){continue;}
                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                tree_root.insert_path(rel_path, is_dir);
            }
            Err(error)=>{
                print!("{}",error)
            }
        }
    }
    let root_name = args.path.canonicalize() // 先轉為絕對路徑
        .ok()
        .and_then(|p| p.file_name().map(|s| s.to_string_lossy().to_string()))
        .unwrap_or_else(|| ".".into());
    writeln!(writer,"{}/", root_name).expect("Write root failed");
    tree_root.print("",writer).expect("Error at print tree");
}
fn rw_file(args:&Args,writer:&mut BufWriter<File>){
    let filter = filter_utils::FileFilter::new(&args.include, &args.exclude);
    let walker = WalkBuilder::new(&args.path)
        .standard_filters(!args.no_ignore)
        .build();
    for result in walker{
        match result {
            Ok(entry)=>{
                let rel_path = entry.path().strip_prefix(&args.path).unwrap_or(entry.path());
                if !filter.is_match(rel_path){continue;}
                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                if !is_dir{
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        writeln!(writer, "<file path=\"{}\">", rel_path.display()).unwrap();
                        writeln!(writer, "{}", content).unwrap();
                        writeln!(writer, "</file>\n").unwrap();
                        println!("  + {}",rel_path.as_os_str().to_string_lossy())
                    }
                }
            }
            Err(error)=>{
                print!("{}",error)
            }
        }
    }
    writer.flush().expect("last input flust fail");
}
fn main() {
    let args = Args::parse();
    
    debug_log(&args);

    let file = File::create(&args.output_path).expect("Create output file failed");
    let mut writer = BufWriter::new(file);
    if !args.no_tree{
        let mut stdout = std::io::stdout();
        let mut multi_writer = io_utils::tee(&mut writer, &mut stdout);
        struct_tree(&args,&mut multi_writer);
    }
    rw_file(&args,&mut writer);
}

fn debug_log(args:&Args){
    println!("======ARGS======");
    println!("Target path: {:#?}",args);    
    println!("======Others======")    
}
