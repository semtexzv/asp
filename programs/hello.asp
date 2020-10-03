use std.mem
use std.io


#inline T, V always
fn reverse of T, V (self ref T) ref T
    where T is Seq of V
      and V is Ord {

  for i in 0..len(self)/2 {
    mem.swap(ref self[i], ref self[len(self) - i])
  }
  outer: loop {
  }
  return self
}

fn main() {
    let world = "ABC"
    print("Hello {world}")
}

