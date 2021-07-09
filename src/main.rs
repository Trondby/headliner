use reqwest;
use xml;
use xml::reader::XmlEvent;

fn main() {
    println!("{}", parse_headline(get_news()));
}

fn get_news() -> String {
    reqwest::blocking::get("https://news.google.com/rss?hl=en-US&gl=US&ceid=US:en").unwrap().text().unwrap()
}

fn parse_headline(news: String) -> String {
    let parser = xml::reader::EventReader::from_str(news.as_str());
    let mut found_item = false;

    for event in parser {
        match event.unwrap() {
            XmlEvent::StartElement { name, attributes: _, namespace: _ } => found_item = found_item || name.local_name == "item",
            XmlEvent::Characters(headline) => if found_item { return headline; },
            _ => {}
        };
    }

    String::from("No headlines found.")
}