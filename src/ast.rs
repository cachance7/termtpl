pub trait Context {
    fn get(&self, s: &str) -> Result<String, String>;
}

#[derive(Debug, Clone, Default)]
pub struct Style {
   pub fg: u8,
   pub bg: u8,
   pub bold: bool,
   pub underline: bool,
}

pub trait Print: Clone {
    fn print(&mut self, s: &str, r: usize, c: usize, style: Option<Style>);
    fn size(&self) -> (usize, usize);
}

pub type Line = Vec<Elt>;

pub enum Elt {
    Literal(String),
    Tag(Tag),
}

pub enum Tag {
    Simple(String),
}

pub struct Template {
    lines: Vec<Line>,
}

impl Template {
   pub fn new(lines: Vec<Line>) -> Self {
        Template { lines }
    }

    pub fn render(&self, context: impl Context, target: impl Print) {
        Renderer::new(self)
            .render(context, target)
            .unwrap_or_else(|err| println!("nope{}", err));
    }
}


/// Behind the scenes mechanism for rendering the template for a given context
struct Renderer<'template> {
    source: &'template Vec<Line>,
}

impl<'template> Renderer<'template> {
    fn new(tpl: &'template Template) -> Self {
        Renderer {
            source: &tpl.lines,
        }
    }

    fn print_to_target(&self, target: &mut impl Print, s: &str, r: usize, c: usize) -> Result<(), String> {
        let (_width, _height) = target.size();
        target.print(s, r, c, None);
        // TODO format for width
        Ok(())
    }

    fn render(&mut self, context: impl Context, mut target: impl Print) -> Result<(), String> {
        let r: usize = 0;
        let c: usize = 0;

        for line in self.source {
            let mut sb = String::default();
            for elt in line {
                match elt {
                    Elt::Literal(s) => sb.push_str(s),
                    Elt::Tag(t) => match t {
                        Tag::Simple(s) => {
                            if let Ok(cs) = context.get(s) {
                                sb.push_str(&cs);
                            } else {
                                // no content for this
                            }
                        }
                    }
                }
            }
            // TODO Format for width
            let _ = self.print_to_target(&mut target, &sb, r, c);
        }
        Ok(())
    }
}
