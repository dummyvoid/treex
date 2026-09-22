use std::{
    env,
    fs,
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct Options {
    path: PathBuf,
    show_files: bool,
    ascii: bool,
}

fn main() {
    let options = match parse_args() {
        Ok(options) => options,
        Err(message) => {
            if !message.is_empty() {
                eprintln!("{message}");
            }
            eprintln!("Usage: treex [drive:][path] [/F] [/A]");
            std::process::exit(1);
        }
    };

    if let Err(error) = run(&options) {
        eprintln!("treex: {error}");
        std::process::exit(1);
    }
}

fn parse_args() -> Result<Options, String> {
    let mut path = None;
    let mut show_files = false;
    let mut ascii = false;

    for arg in env::args().skip(1) {
        match arg.to_ascii_uppercase().as_str() {
            "/F" => show_files = true,
            "/A" => ascii = true,
            "/?" | "-H" | "--HELP" => {
                println!("Usage: treex [drive:][path] [/F] [/A]");
                println!("  /F  Display files as well as directories.");
                println!("  /A  Use ASCII characters instead of line-drawing characters.");
                return Err(String::new());
            }
            _ if arg.starts_with('/') => return Err(format!("Unknown option: {arg}")),
            _ if path.is_none() => path = Some(PathBuf::from(arg)),
            _ => return Err("Only one path may be specified.".into()),
        }
    }

    Ok(Options {
        path: path.unwrap_or_else(|| PathBuf::from(".")),
        show_files,
        ascii,
    })
}

fn run(options: &Options) -> io::Result<()> {
    let root = fs::canonicalize(&options.path)?;
    let name = root
        .file_name()
        .and_then(|n| n.to_str())
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| root.to_str().unwrap_or("."));

    println!("{name}");
    print_tree(&root, "", options)
}

fn print_tree(path: &Path, prefix: &str, options: &Options) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|entry| options.show_files || entry.path().is_dir())
        .collect();

    entries.sort_by_key(|entry| entry.file_name().to_ascii_lowercase());

    for (index, entry) in entries.iter().enumerate() {
        let last = index + 1 == entries.len();
        let (branch, next_prefix) = if options.ascii {
            (if last { "\\-- " } else { "|-- " }, if last { "    " } else { "|   " })
        } else {
            (if last { "└── " } else { "├── " }, if last { "    " } else { "│   " })
        };

        println!("{prefix}{branch}{}", entry.file_name().to_string_lossy());

        if entry.path().is_dir() {
            print_tree(
                &entry.path(),
                &format!("{prefix}{next_prefix}"),
                options,
            )?;
        }
    }

    Ok(())
}
