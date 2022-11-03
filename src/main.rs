use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use pinyin::ToPinyin;
use shellexpand;
use std::env;
use std::path::Path;
use std::process;
use std::vec;

fn to_pinyin(path: &str) -> String {
    let mut converted: Vec<String> = vec![];
    let chars: Vec<char> = path.chars().collect();
    for (idx, lst) in path.to_pinyin().enumerate() {
        if let Some(lst) = lst {
            converted.push(lst.plain().to_string());
        } else {
            converted.push(chars[idx].to_string());
        }
    }
    converted.join("")
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        process::exit(0);
    }
    let matcher = SkimMatcherV2::default();

    let dironly = args[1] == "-d";
    let path = shellexpand::tilde(&args[2]).replace("\\", "");
    let path = Path::new(&path);

    let parent = match path.parent() {
        Some(par) if par.to_str().unwrap().len() > 0 => par,
        _ => Path::new("./"),
    };
    let file_name = match path.file_name() {
        Some(item) => item.to_str().unwrap(),
        None => "",
    };
    let mut ret: Vec<String> = vec![];
    for entry in parent.read_dir().expect("Read dir failed") {
        if let Ok(entry) = entry {
            if let Some(entry_fn) = entry.path().file_name() {
                let entry_fn = entry_fn.to_str().unwrap();
                if matcher
                    .fuzzy_match(&to_pinyin(entry_fn), file_name)
                    .is_some()
                    && (!dironly || entry.path().is_dir())
                {
                    ret.push(String::from(entry_fn));
                }
            }
        }
    }

    for item in ret {
        println!("{}", item);
    }
}
