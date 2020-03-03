mod add;
mod base64;

use add::add;
use base64::b64encode;

fn main() {
    println!("{:?}", add(1, 2));
    println!("{:?}", b64encode("Hello".to_string()));
}
