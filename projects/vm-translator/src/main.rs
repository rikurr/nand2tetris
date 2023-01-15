use std::{
    env, fs,
    io::{self, Read},
    path::Path,
    process,
};

struct Parser {
    line: Vec<String>,
}

impl Parser {
    pub fn new(source: String) -> Self {
        let line = source
            .split('\n')
            .filter(|line| {
                let line = line.trim().split('/').next().unwrap().trim();
                !line.is_empty()
            })
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        Self { line }
    }
}

impl Iterator for Parser {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.line.iter().next().cloned()
    }
}

// cargo run ../07/StackArithmetic/SimpleAdd/SimpleAdd.vm
// cargo run ../07/StackArithmetic/SimpleAdd/
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let source = args.get(1).unwrap_or_else(|| {
        println!("missing argument");
        process::exit(1);
    });
    let path = Path::new(source);
    println!("{:?}", path);

    if !path.exists() {
        println!("no such path or filename: {}", source);
        process::exit(1);
    }

    let basename = path
        .file_name()
        .unwrap_or_else(|| {
            println!("invalid filename: {}", source);
            process::exit(1);
        })
        .to_string_lossy()
        .into_owned();

    let dir = if path.is_dir() {
        path.to_string_lossy().into_owned()
    } else {
        path.parent()
            .unwrap_or_else(|| {
                println!("invalid filename: {}", source);
                process::exit(1);
            })
            .to_string_lossy()
            .into_owned()
    };

    let vm_file = if path.is_dir() {
        basename
    } else if basename.ends_with("vm") {
        basename.replace(".vm", "")
    } else {
        println!("filename is expected to be ends with .vm: {}", source);
        process::exit(1)
    };

    println!("basename: {}", vm_file);

    let vm_file = fs::File::open(format!("{}/{}.vm", dir, vm_file))?;

    let mut buf_reader = io::BufReader::new(vm_file);
    let mut vm_source = String::new();
    buf_reader.read_to_string(&mut vm_source)?;

    println!("{}", vm_source);

    let parser = Parser::new(vm_source);

    for l in parser {
        println!("hello{}", l)
    }

    Ok(())
}
