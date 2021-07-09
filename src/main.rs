use reqwest;
use xml;
use xml::reader::XmlEvent;

fn main() {
    println!("{}", parse_headline(get_news()).unwrap());
}

fn get_news() -> String {
    reqwest::blocking::get("https://news.google.com/rss").unwrap().text().unwrap()
}

fn parse_headline(news: String) -> Result<String, String> {
    let parser = xml::reader::EventReader::from_str(news.as_str());
    let mut found_item = false;

    for event in parser {
        match event.unwrap() {
            XmlEvent::StartElement { name, attributes: _, namespace: _ } => if name.local_name == "item" { found_item = true },
            XmlEvent::Characters(headline) => if found_item { return Ok(headline); },
            _ => {}
        };
    }

    Err(String::from("No headlines found"))
}
