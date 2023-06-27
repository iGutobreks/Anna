mod Combat;
mod Memory;

use Memory::memory_parser;

fn main() {
  println!("Anna by guto.rs");
  let mut player = Combat::Modules::Combat::default();

  player.draw();
  player.writer_bytes("javaw");
}


