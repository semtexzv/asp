use std.mem
use std.io

const a is Map of int, int = Map.new()

fn alloc of T () ref T where T is Sized {
    mem.currentAllocator.alloc(mem.layout of T())
}

#inline T, V always
fn reverse of T, V (self ref T) ref T
    where T is Seq of V
      and V is Ord {


  for i in 0..len(self)/2 {
    mem.swap(ref self[i], ref self[len(self) - i])
  }
  return self
}

fn main() {
    let world = "ABC"
    print("Hello {world}")
}

