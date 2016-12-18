use std::io::{self, Write};

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
use templates::*;

fn main() {
    println!("### Page:");
    page(&mut io::stdout()).unwrap();
    for s in statics::STATICS {
        println!("### /static/{}:", s.name);
        io::stdout().write(s.content).unwrap();
    }
}

#[test]
fn test_page_w_static() {
    assert_eq!(r2s(|o| page(o)),
               "<html>\n  \
                <head>\n    \
                <title>Example with stylesheet</title>\n    \
                <link rel=\"stylesheet\" href=\"/static/style-o2rFo1lI.css\" \
                      type=\"text/css\"/>\n  \
                </head>\n  \
                <body>\n    \
                Hello world!\n  \
                </body>\n\
                </html>\n");
}

#[test]
fn test_static_css_data() {
    // TODO The css content should be minified!
    use templates::statics::style_css;
    use std::str::from_utf8;
    assert_eq!(from_utf8(&style_css.content).unwrap(),
               "body {\n    background: white;\n    color: black;\n}\n");
}

#[test]
fn test_all_statics_known() {
    use templates::statics::STATICS;
    assert_eq!(STATICS.iter().map(|s| s.name).collect::<Vec<_>>(),
               ["foo-JckCHvyv.css", "foo-R-7hhHLr.js", "style-o2rFo1lI.css"]);
}

#[cfg(test)]
fn r2s<Call>(call: Call) -> String
    where Call: FnOnce(&mut Write) -> io::Result<()>
{
    let mut buf = Vec::new();
    call(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}
