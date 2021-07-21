use reqwest;
use std::env;
use std::fs;

const URL: &str = "https://warframe.com/repos/hnfvc0o3jnfvc873njb03enrf56.html";

fn main() {
    let path_home = env::var("HOME").expect("HOME variable not bound");
    let path_tables = format!("{}/.local/share/warframe_drops.html", path_home);
    let mut args = env::args();
    let invocation = args.next().expect("missing invocation argument");

    if let Some(key) = args.next() {
        if &key[..] == "--update" || &key[..] == "-u" {
            let body = reqwest::blocking::get(URL)
                .expect("request failure")
                .text()
                .expect("request failure");
            fs::write(&path_tables, &body).expect("write failure");
        } else if &key[..] == "--dedicated" || &key[..] == "-d" {
            // The "--dedicated" flag will return only one table if the listed key has its own in
            // the drop tables; otherwise, it will inform the user that no such table exists.
            let key = args.next().expect("missing key");
            let tables = fs::read_to_string(path_tables).expect("read failrue");
            let narrowed = dedicated_table(&tables, &key);
            println!("{}", narrowed);
        } else {
            let tables = fs::read_to_string(path_tables).expect("read failure");
            let narrowed = drop_info(&tables, &key);
            println!("{}", narrowed);
        }
    } else {
        eprintln!("usage: {} [ --update | <search key> ]", invocation);
    }
}

fn dedicated_table(tables: &str, key: &str) -> String {
    let key = key.to_ascii_lowercase();
    let mut lines: Vec<String> = Vec::new();

    tables.split("<tr")
        .filter(|string| {
            !string.contains("blank-row")
        })
        .skip_while(|string| !string.contains("><th ") || !string.to_ascii_lowercase().contains(&key))
        .take_while(|string| !string.contains("><th ") || string.to_ascii_lowercase().contains(&key))
        .map(|string| {
            let mut line = string
                .split(&['<', '>'][..])
                .skip(3)
                .step_by(4)
                .filter(|string| !string.is_empty())
                .collect::<Vec<&str>>()
                .join(" : ");
            if string.contains("Rotation") {
                lines.push(String::new());
            } else if !string.contains("><th ") {
                line = format!("\t{}", line);
            }
            lines.push(line);
        })
        .last(); // last() is here to consume the iterator

    lines.join("\n")
}

fn drop_info(tables: &str, key: &str) -> String {
    let key = key.to_ascii_lowercase();
    let mut lines: Vec<String> = Vec::new();
    let mut headers: (Option<&str>, Option<&str>) = (None, None);
    let mut push_next = false;
    let mut indentation = 0;

    tables.split("><")
        .skip_while(|string| !string.contains("Missions:"))
        .filter(|string|
            !string.starts_with('/')
            && string.len() > 2
            && !string.contains("blank-row")
            && !string.starts_with("h3")
            && !string.starts_with("tr ")
            && !string.starts_with("td ")
            && string != &"table"
        )
        .map(|string| {
            let content = string
                .split(&['<', '>'][..])
                .nth(1).expect("parsing error");
            if string.starts_with("th") {
                if content.contains("Rotation") || content.contains('%') {
                    headers.1 = Some(content);
                } else {
                    headers.0 = Some(content);
                    headers.1 = None;
                }
            } else if content.to_ascii_lowercase().contains(&key) {
                if let Some(header) = headers.0.take() {
                    indentation = 0;
                    lines.push(format!("\n{}{}", "\t".repeat(indentation), header));
                    indentation += 1;
                }
                if let Some(header) = headers.1.take() {
                    indentation = 1;
                    lines.push(format!("{}{}", "\t".repeat(indentation), header));
                    indentation += 1;
                }
                lines.push(format!("{}{}", "\t".repeat(indentation), content));
                indentation += 3;
                push_next = true;
            } else if push_next {
                lines.push(format!("{}{}", "\t".repeat(indentation), content));
                indentation -= 3;
                push_next = false;
            }
        })
        .last(); // last() is here to consume the iterator

    lines.join("\n")
}
