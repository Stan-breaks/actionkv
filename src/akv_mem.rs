use std::{env::args, path::Path};

use actionkv::ActionKV;

#[cfg(target_os = "windows")]
const USAGE: &str = "
USAGE:
     akv_mem
     akv_mem.exe FILE delete KEY
     akv_mem.exe FILE insert KEY VALUE
     akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
USAGE:
     akv_mem FILE get KEY
     akv_mem FILE delete KEY
     akv_mem FILE insert KEY VALUE
     akv_mem FILE update KEY VALUE
";

type Bytestr = [u8];

fn main() {
    let args: Vec<String> = args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action: &str = args.get(2).expect(&USAGE).as_ref();
    let key: &Bytestr = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = Path::new(&fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action {
        "get" => match store.get(key).unwrap() {
            Some(value) => println!("{:?}", value),
            None => eprintln!("unable to get value of key: {:?}", key),
        },
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap()
        }
        "delete" => store.delete(key).unwrap(),
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap()
        }
        _ => eprintln!("{}", &USAGE),
    };
}
