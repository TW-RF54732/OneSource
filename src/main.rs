use clap::{Parser};
use std::{fs::File,path::PathBuf, task::Context};
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
    
    //content setting
    #[arg(long,action = clap::ArgAction::SetTrue,help="Don't use .gitignore")]
    no_ignore:bool,
    #[arg(short,long,default_value="*")]
    include:String,
    #[arg(short='x',long,default_value="")]
    exclude:String,
    
    //Tree setting
    #[arg(long,visible_alias="ti")]
    tree_include: Option<String>,
    #[arg(long,visible_alias="tx")]
    tree_exclude: Option<String>,
    #[arg(long,action = clap::ArgAction::SetTrue,help="Don't write tree")]
    no_tree:bool,
    #[arg(long,action = clap::ArgAction::SetTrue,help="Tree don't use .gitignore")]
    tree_no_ignore:bool,

    // Behavior setting
    #[arg(long, action = clap::ArgAction::SetTrue, help = "Preview file list without writing to disk")]
    dry_run: bool,
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
fn rw_file<W: Write>(args:&Args,writer:&mut W){
    let filter = filter_utils::FileFilter::new(&args.include, &args.exclude);
    let walker = WalkBuilder::new(&args.path)
        .standard_filters(!args.no_ignore)
        .build();
    let mut count:u32 = 0;
    for result in walker{
        match result {
            Ok(entry)=>{
                let rel_path = entry.path().strip_prefix(&args.path).unwrap_or(entry.path());
                if !filter.is_match(rel_path){continue;}
                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                if !is_dir{
                    if args.dry_run {
                        println!("[EXPECT] {}", rel_path.display());
                        count += 1;
                        continue;
                    }
                    if let Ok(bytes) = std::fs::read(entry.path()) {
                        let content = String::from_utf8_lossy(&bytes);
                        writeln!(writer, "<file path=\"{}\">", rel_path.display()).unwrap();
                        writeln!(writer, "{}", content).unwrap();
                        writeln!(writer, "</file>\n").unwrap();
                        println!("  + {}",rel_path.display());
                        count += 1;
                    }
                }
            }
            Err(error)=>{
                print!("{}",error)
            }
        }
    }
    if !args.dry_run {
        writer.flush().expect("last input flush fail");
    }
    println!("======File processing completed======");
    println!("Files Processed: {}",count);
}
fn main() {
    let args = Args::parse();
    debug_log(&args);

    if args.dry_run {
        println!("\n[DRY RUN MODE] Previews only, no files will be written.\n");
        
        if !args.no_tree {
            struct_tree(&args, &mut std::io::stdout());
        }

        // if dry run, no writer
        let mut sink = std::io::sink();
        rw_file(&args, &mut sink);

        println!("Dry run finished. If executed, file would be saved at: {}",std::env::current_dir()
                                                                                .map(|dir| dir.join(&args.output_path))
                                                                                .unwrap_or_else(|_| args.output_path.clone()) 
                                                                                .display());

    } else {
        
        // Only not dry will creat file
        let file = File::create(&args.output_path).expect("Create output file failed");
        let mut writer = BufWriter::new(file);
        
        let abs_path_display = args.output_path.canonicalize()
            .unwrap_or_else(|_| args.output_path.clone())
            .display()
            .to_string();
        if !args.no_tree {
            let mut stdout = std::io::stdout();
            let mut multi_writer = io_utils::tee(&mut writer, &mut stdout);
            struct_tree(&args, &mut multi_writer);
        }

        rw_file(&args, &mut writer);
        
        writer.flush().expect("Flush failed");
        
        println!("Output saved to: {}", abs_path_display);
    }

    println!("=====================================");
}

fn debug_log(args:&Args){
    println!("======ARGS======");
    println!("Target path: {:#?}",args);    
    println!("======Others======")    
}
