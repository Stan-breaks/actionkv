use std::{collections::HashMap, env::args, path::Path};

use actionkv::ActionKV;
use bincode::{config::standard, decode_from_slice, encode_to_vec};

#[cfg(target_os = "windows")]
const USAGE: &str = "
USAGE:
     akv_mem
     akv_mem
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
type ByteString = Vec<u8>;

fn store_index_on_disk(a: &mut ActionKV, index_key: &Bytestr) {
    a.index.remove(index_key);
    let config = standard();
    let vec_bytes = encode_to_vec(&a.index, config).unwrap();
    let boxed: Box<[u8]> = vec_bytes.into();
    let value: &Bytestr = &*boxed;
    a.index = HashMap::new();
    a.insert(index_key, value).unwrap();
}

fn main() {
    const INDEX_KEY: &Bytestr = b"+index";
    let args: Vec<String> = args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action: &str = args.get(2).expect(&USAGE).as_ref();
    let key: &Bytestr = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = Path::new(&fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    let config = standard();
    match action {
        "get" => {
            let vec_bytes = store.get(&INDEX_KEY).unwrap().unwrap();
            let boxed: Box<[u8]> = vec_bytes.into();
            let slice: &Bytestr = &*boxed;
            let (decoded_index, _): (HashMap<ByteString, u64>, usize) =
                decode_from_slice(slice, config).expect("Failed to decode index");
            match decoded_index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(&i) => {
                    let kv = store.get_at(i).unwrap();
                    let value = String::from_utf8(kv.value.clone()).unwrap();
                    println!("{:?}", (value))
                }
            }
        }
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();

            store.insert(key, value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        "delete" => store.delete(key).unwrap(),
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        _ => eprintln!("{}", &USAGE),
    };
}
