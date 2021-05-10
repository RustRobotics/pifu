use regex::Regex;

use pifu::base::expand_file_macro_simple;

fn main() {
    let s = "app-${env.HOME}.deb";
    let pattern = Regex::new(r"\$\{env\.(\w+)\}").unwrap();
    println!("pattern is match: {:?}", pattern.is_match(s));
    for cap in pattern.captures_iter(s) {
        println!("cap: {:?}", &cap);
    }

    let ret = expand_file_macro_simple(s);
    println!("ret: {:?}", ret);
}
