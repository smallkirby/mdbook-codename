use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::PreprocessorContext;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

pub struct Preprocessor;

impl mdbook::preprocess::Preprocessor for Preprocessor {
    fn name(&self) -> &str {
        "codename"
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }

    fn run(&self, _: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut error: Option<Error> = None;
        book.for_each_mut(|item: &mut BookItem| {
            if error.is_some() {
                return;
            }
            if let BookItem::Chapter(ref mut chapter) = *item {
                if let Err(err) = handle_chapter(chapter) {
                    error = Some(err);
                }
            }
        });

        error.map_or(Ok(book), Err)
    }
}

struct CodeblockParser {
    content: String,
    codeblock: Option<Codeblock>,
}

struct Codeblock {
    lines: Vec<String>,
    lang: Option<String>,
    filename: Option<String>,
}

impl Codeblock {
    fn new(fence_line: &str) -> Self {
        let s = fence_line.trim_start_matches("```");

        let (lang, filename) = if s.contains(':') {
            let mut parts = s.split(':');
            let lang = parts.next().unwrap().trim().into();
            let filename = parts.collect::<Vec<&str>>().join(":").trim().into();

            (Some(lang), Some(filename))
        } else {
            let lang = if s.contains('.') {
                Some(s.split('.').last().unwrap().into())
            } else {
                None
            };

            (lang, Some(s.into()))
        };

        Self {
            lines: Vec::new(),
            lang,
            filename,
        }
    }

    fn push(&mut self, line: &str) {
        self.lines.push(line.into());
    }

    fn parse(&mut self) -> Vec<String> {
        let mut parsed_lines: Vec<String> = Vec::new();

        if let Some(filename) = &self.filename {
            if !filename.is_empty() {
                parsed_lines.push(format!("<div class=\"codeblock_filename_container\"><span class=\"codeblock_filename_inner hljs\">{}</span></div>\n",
                filename.clone(),
            ));
            }
        }

        parsed_lines.push(format!("```{}", self.lang.clone().unwrap_or("".into())));
        parsed_lines.extend(self.lines.clone());
        parsed_lines.push("```".into());

        parsed_lines
    }
}

impl CodeblockParser {
    fn new(content: &str) -> Self {
        Self {
            content: content.into(),
            codeblock: None,
        }
    }

    fn parse(&mut self) -> String {
        let mut parsed_lines: Vec<String> = Vec::new();

        for line in self.content.lines() {
            if is_fence(line) {
                if let Some(cb) = self.codeblock.as_mut() {
                    parsed_lines.extend(cb.parse());
                    self.codeblock = None;
                } else {
                    self.codeblock = Some(Codeblock::new(line));
                }
            } else {
                if let Some(cb) = self.codeblock.as_mut() {
                    cb.push(line);
                } else {
                    parsed_lines.push(line.into());
                }
            }
        }

        parsed_lines.join("\n")
    }
}

fn is_fence(line: &str) -> bool {
    line.starts_with("```")
}

fn handle_chapter(chapter: &mut Chapter) -> Result<(), Error> {
    chapter.content = inject_css(&chapter.content)?;
    chapter.content = render_codename(&chapter.content)?;

    Ok(())
}

fn inject_css(content: &str) -> Result<String, Error> {
    let style = Asset::get("style.css")
        .ok_or_else(|| Error::msg("Failed to read style.css from assets"))?;
    let style = std::str::from_utf8(style.data.as_ref())?;

    Ok(format!("<style>\n{style}\n</style>\n{content}"))
}

fn render_codename(content: &str) -> Result<String, Error> {
    let mut parser = CodeblockParser::new(content);

    Ok(parser.parse())
}
