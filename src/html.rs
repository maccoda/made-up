use pulldown_cmark::{Event, Tag};

#[derive(Debug)]
struct Consumer<'a, I> {
    iter: I,
    buffer: String,
    current: Option<Tag<'a>>,
}

impl<'a, I: Iterator<Item = Event<'a>>> Consumer<'a, I> {
    fn consume<'b>(&mut self) -> String {
        while let Some(event) = self.iter.next() {
            println!("{:?}", event);
            match event {
                Event::Start(tag) => {
                    self.buffer.push_str(&print_start_elem(&tag));
                    self.current = Some(tag);
                }
                Event::End(tag) => self.buffer.push_str(&print_end_elem(&tag)),
                Event::Text(text) => {
                    if let Some(tag) = self.current.clone() {
                        match tag {
                            Tag::Header(_) => {
                                let to_add =
                                    format!("{}\"> {}", name_to_id(&text.to_string()), text);
                                self.buffer.push_str(&to_add)
                            }
                            Tag::Image(_, _) => self.buffer.push_str(&format!(" alt=\"{}\"", text)),
                            _ => self.buffer.push_str(&text),
                        }
                    } else {
                        self.buffer.push_str(&text)
                    }
                }
                Event::Html(content) => self.buffer.push_str(&content.to_string()),
                elem => println!("Unhandled type: {:?}", elem),
            }
        }
        self.buffer.clone()
    }
}

fn print_start_elem(tag: &Tag) -> String {
    let result = match tag {
        &Tag::Header(int) => format!("<h{} id=\"", int),
        &Tag::Strong => "<b>".to_string(),
        &Tag::Emphasis => "<em>".to_string(),
        &Tag::Item => "<li>".to_string(),
        &Tag::List(_) => "<ul>".to_string(),
        &Tag::Paragraph => "<p>".to_string(),
        &Tag::Image(ref src, _) => format!("<img src=\"{}\"", src.as_ref()),
        &Tag::Code => "<code>".to_string(),
        &Tag::CodeBlock(ref lang) => format!("<pre><code class=\"language-{}\">", lang),
        &Tag::Link(ref href, _) => format!("<a href=\"{}\">", href),
        // TODO Handle alignment
        &Tag::Table(_) => "<table>".to_string(),
        &Tag::TableHead => "<thead>".to_string(),
        &Tag::TableCell => "<td>".to_string(),
        &Tag::TableRow => "<tr>".to_string(),
        tag => {
            println!("{:?}", tag);
            unimplemented!();
        }
    };

    println!("{:?}", result);
    result
}

fn print_end_elem(tag: &Tag) -> String {
    let result = match tag {
        &Tag::Header(int) => format!("</h{}>\n", int),
        &Tag::Strong => "</b>\n".to_string(),
        &Tag::Emphasis => "</em>\n".to_string(),
        &Tag::Item => "</li>\n".to_string(),
        &Tag::List(_) => "</ul>\n".to_string(),
        &Tag::Paragraph => "</p>\n".to_string(),
        &Tag::Image(_, _) => "/>\n".to_string(),
        &Tag::Code => "</code>\n".to_string(),
        &Tag::CodeBlock(_) => "</code></pre>\n".to_string(),
        &Tag::Link(_, _) => "</a>\n".to_string(),
        // TODO Handle alignment
        &Tag::Table(_) => "</table>\n".to_string(),
        &Tag::TableHead => "</thead>\n".to_string(),
        &Tag::TableCell => "</td>\n".to_string(),
        &Tag::TableRow => "</tr>\n".to_string(),
        tag => {
            println!("{:?}", tag);
            unimplemented!();
        }
    };

    println!("{:?}", result);
    result
}

fn name_to_id(name: &String) -> String {
    name.to_lowercase().replace(" ", "-")
}

pub fn consume<'a, I: Iterator<Item = Event<'a>>>(iter: I) -> String {
    let mut consumer = Consumer {
        iter: iter,
        buffer: String::new(),
        current: None,
    };
    consumer.consume()
}
