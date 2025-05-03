use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

fn main() {
    // lesen hat funktioniert und ist in tst.html gespeichert
    let document = Document::from(include_str!("../tst.html"));

    //println!("{}", document.nodes.len());
    for node in document.find(Attr("class", "arrow0")) {
        let mut line: Vec<String> = vec![];
        for child in node.children() {
            let temp = child
                .text()
                .trim()
                .split(' ')
                .nth(0)
                .unwrap()
                .to_string()
                .replace(",", ".");
            if temp.len() > 1 {
                line.push(temp);
            }
        }
        println!("{}", line[..5].join(","));
    }
}
