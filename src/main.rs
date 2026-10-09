use std::{
    env,
    fs,
    io::{self, BufWriter, Write},
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

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    if let Err(error) = run(&options, &mut out).and_then(|()| out.flush()) {
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

fn run(options: &Options, out: &mut impl Write) -> io::Result<()> {
    let root = fs::canonicalize(&options.path)?;
    let name = root
        .file_name()
        .and_then(|n| n.to_str())
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| root.to_str().unwrap_or("."));

    writeln!(out, "{name}")?;
    print_tree(&root, &mut String::new(), options, out)
}

fn print_tree(
    path: &Path,
    prefix: &mut String,
    options: &Options,
    out: &mut impl Write,
) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let is_dir = entry.file_type().map(|kind| kind.is_dir()).unwrap_or(false);
            (options.show_files || is_dir).then_some((entry, is_dir))
        })
        .collect();

    entries.sort_by_cached_key(|(entry, _)| entry.file_name().to_ascii_lowercase());

    for (index, (entry, is_dir)) in entries.iter().enumerate() {
        let last = index + 1 == entries.len();
        let (branch, next_prefix) = if options.ascii {
            (if last { "\\-- " } else { "|-- " }, if last { "    " } else { "|   " })
        } else {
            (if last { "└── " } else { "├── " }, if last { "    " } else { "│   " })
        };

        writeln!(out, "{prefix}{branch}{}", entry.file_name().to_string_lossy())?;

        if *is_dir {
            let child_path = entry.path();
            prefix.push_str(next_prefix);
            let result = print_tree(&child_path, prefix, options, out);
            prefix.truncate(prefix.len() - next_prefix.len());
            result?;
        }
    }

    Ok(())
}
