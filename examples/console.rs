#[derive(Clone, Debug)]
struct Console {
    count: usize
}

use termtpl::ast::*;

// If no input tpl is specified, we'll use this
const EXAMPLE_TPL: &str = "{test_tag_1} someliteral {missing_tag}
{newline!}
literal start {another.tag}
";

impl Print for Console {
    /// Console doesnt know how to print backward, but will skip lines if r > count
    fn print(&mut self, s: &str, r: usize, c: usize, style: Option<Style>) {
        println!("{}{}", &" ".repeat(c), s);
        // TODO increment based on actual rows printed
    }

    fn size(&self) -> (usize, usize) {
        (80, 20)
    }
}

struct Doc<'a> {
    data: std::collections::HashMap<&'a str,&'a str>
}

impl<'a> Doc<'a> {
    fn new() -> Self {
        Doc {
            data: std::collections::HashMap::<&'a str,&'a str>::new()
        }
    }
}

impl Context for Doc<'_> {
    fn get(&self, s: &str) -> Result<String, String> {
        Ok(format!("{}", self.data.get(s).unwrap_or(&"")))
    }
}

fn main() {
    use std::io::*;

    let mut source = String::new();
    match std::env::args().nth(1) {
        Some(filename) => {
            use std::fs::File;

            File::open(&filename)
                .expect(&format!("Can't open {}", &filename))
                .read_to_string(&mut source)
                .expect(&format!("Can't read contents of {}", &filename));
        }

        None => {
            source = String::from(EXAMPLE_TPL);
        }
    }

    if source.is_empty() {
        println!("Empty file");
        return;
    }
    let mut d = Doc::new();
    d.data.insert("test_tag_1", "hey there");
    d.data.insert("another.tag", "hey down here");

    let c = Console{count: 0};
    termtpl::compile(&source).expect("OH NO").render(d, c);
}
