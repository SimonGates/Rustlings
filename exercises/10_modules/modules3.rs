// You can use the `use` keyword to bring module paths from modules from
// anywhere and especially from the standard library into your scope.

// TODO: Bring `SystemTime` and `UNIX_EPOCH` from the `std::time` module into
// your scope. Bonus style points if you can do it with one line!
// use ???;

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    if let Ok(n) = SystemTime::now().duration_since(UNIX_EPOCH) {
        println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs())
    } else {
        panic!("SystemTime before UNIX EPOCH!")
    }
}
