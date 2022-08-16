mod jvdt;

fn main() {
    let jvlink: jvdt::AxJVLink = jvdt::AxJVLink::new();
    println!("{:?}", jvlink);
}
