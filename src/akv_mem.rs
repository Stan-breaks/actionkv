#[cfg(target_os = "windows")]
const USAGE: &str = "
USAGE:
     akv_mem.exe FILE get KEY
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action: &str = args.get(2).expect(&USAGE).as_ref();
    let key: &str = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);
}
