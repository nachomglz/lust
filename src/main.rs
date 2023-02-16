use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    path: Option<PathBuf>,
    #[arg(short, long)]
    all: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path = args.path.unwrap_or(std::env::current_dir().unwrap());

    if path.is_dir() {
        let dir_vec = std::fs::read_dir(path)
            .unwrap()
            .collect::<Result<Vec<std::fs::DirEntry>, _>>()
            .unwrap();

        let total_elements: usize;
        if args.all {
            total_elements = dir_vec.len();
        } else {
            total_elements = dir_vec
                .iter()
                .filter(|i| i.file_name().to_str().unwrap().chars().nth(0).unwrap() != '.')
                .collect::<Vec<&std::fs::DirEntry>>()
                .len();
        }

        println!("Total elements: {}", total_elements);

        for item in dir_vec.iter() {
            if args.all {
                println!("{}", item.file_name().to_str().unwrap());
            } else if item.file_name().to_str().unwrap().chars().nth(0).unwrap() != '.' {
                println!("{}", item.file_name().to_str().unwrap());
            }
        }
    } else {
        eprintln!("The path provided is not a directory -> {:?}", path);
    }

    Ok(())
}
