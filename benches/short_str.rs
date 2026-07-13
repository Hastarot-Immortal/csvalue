use std::mem::MaybeUninit;

fn main() {
    divan::main();
}

const MAX_LEN: usize = 24;
const S_6_BYTES: &str = "Justo.";
const S_12_BYTES: &str = "Fusce nulla.";
const S_24_BYTES: &str = "Etiam ac vehicula fusce.";

#[divan::bench(args = [S_6_BYTES, S_12_BYTES, S_24_BYTES])]
fn init_maybeuninit(s: &str) {
  let mut arr = [MaybeUninit::uninit(); MAX_LEN];
  for (i, b) in s.bytes().enumerate() {
    arr[i].write(b);
  }
}

#[divan::bench(args = [S_6_BYTES, S_12_BYTES, S_24_BYTES])]
fn init_arr(s: &str) {
  let mut arr = [0u8; MAX_LEN];
  for (i, b) in s.bytes().enumerate() {
    arr[i] = b;
  }
}
