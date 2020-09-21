fn get_version() -> u16 {
    2828
}

fn usage() {
    let the_version : u16;
    the_version = get_version();
    println!("MiniMD, a markdown compiler written by Anthony Mendonca");
    println!("Version {}", the_version);
}

fn main() {
    usage();
}
