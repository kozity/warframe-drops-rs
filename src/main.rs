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
        } else {
            let iter = fs::read_to_string(path_tables).expect("read failure");
            let mut iter = iter
                .split("><")
                .filter_map(|string| {
                    if string.starts_with('/') { return None; }
                    //else { println!("{}", string); } // test
                    let string = &string.to_ascii_lowercase();
                    let (bracket_1, bracket_2) = match (string.find('>'), string.find('<')) {
                        (Some(first), Some(second)) => (first, second),
                        _ => return Some((string, "")),
                    };
                    Some((&string[0..bracket_1], &string[bracket_1 + 1..bracket_2]))
                });
            let mut headers: Vec<String> = Vec::new();
            loop { // eat introductory header
                if let Some(pair) = iter.next() {
                    if pair.1 == "Missions:" { break; }
                }
            }
            let mut in_dedicated_table = false;
            for pair in iter {
                println!("{:?}", pair); // test
                if in_dedicated_table {
                    if pair.0.contains("blank") {
                        in_dedicated_table = false;
                        continue;
                    } else {
                        print!("\t{}", pair.1); // TODO: adjust indentation
                    }
                }
                if pair.0.starts_with("th") {
                    if pair.1 == &key {
                        print!("{}", pair.1); // TODO: adjust indentation
                        in_dedicated_table = true;
                    }
                } else if pair.0.starts_with("td") {
                    if pair.1.contains(&key) {
                        let mut i = 0;
                        while i < headers.len() {
                            print!("{}", headers[i]);
                            i += 1;
                            for _ in 0..i {
                                print!("\t");
                            }
                        }
                        print!("{}", pair.1); // TODO: LATEST: nothing is printing for some reason.
                        headers.pop();
                    }
                } else if pair.0.starts_with("tr") { // TODO: adjust
                    print!("\n");
                }
            }
        }
    } else {
        eprintln!("usage: {} [ --update | <search key> ]", invocation);
    }
}
