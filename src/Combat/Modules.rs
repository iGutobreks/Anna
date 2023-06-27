use std::io::Write;
use std::ops::Index;
use winapi::um::winuser::{GetAsyncKeyState, VK_LBUTTON};
use crate::memory_parser;

pub const MODULES: [&str; 4] = ["reach", "megajump", "bhop", "speed"];
pub const MODULES_DISABLE: [&str; 4] = ["00 00 00 00 00 00 08 40 00 00 00 00 00", "3D 0A D7 3E", "C3 F5 68 3F", "00 00 00 00 00 40 8F 40"];
pub const MODULES_ENABLE: [&str; 3] = ["00 00 00 40", "10 00 80 3F", "00 00 00 00 00 00 89 40"];

#[derive(Debug)]
pub struct Combat {
  pub reach: f32,
  megaJump: bool,
  Bhop: bool,
  autoClicker: i32,
  speed: bool,
  pub modules: std::collections::BTreeMap<&'static str, String>,
}

impl Default for Combat {
  fn default() -> Self {
    Self {
      reach: 0f32, megaJump: false, Bhop: false, autoClicker: 1i32, speed: false, modules: std::collections::BTreeMap::from([("reach", String::from("0")), ("megajump", String::from("false")), ("bhop", String::from("false")), ("speed", String::from("false"))])
    }
  }
}

impl Combat {
  pub fn draw(&mut self) {
      print!("Reach: ");
      std::io::stdout().flush().unwrap();
      let mut content = String::new();
      std::io::stdin().read_line(&mut content).unwrap();
      self.reach = content.trim().parse::<f32>().unwrap();

      print!("MegaJump: ");
      std::io::stdout().flush().unwrap();
      let mut content = String::new();
      std::io::stdin().read_line(&mut content).unwrap();
      self.megaJump = if content.trim() == "enable" {true} else {false};

      print!("Bhop: ");
      std::io::stdout().flush().unwrap();
      let mut content = String::new();
      std::io::stdin().read_line(&mut content).unwrap();
      self.Bhop = if content.trim() == "enable" {true} else {false};

      print!("AutoClicker: ");
      std::io::stdout().flush().unwrap();
      let mut content = String::new();
      std::io::stdin().read_line(&mut content).unwrap();
      self.autoClicker = content.trim().parse::<i32>().unwrap();

      print!("Speed: ");
      std::io::stdout().flush().unwrap();
      let mut content = String::new();
      std::io::stdin().read_line(&mut content).unwrap();
      self.speed = if content.trim() == "enable" {true} else {false };

      *self.modules.get_mut("reach").unwrap() = (self.reach).to_string();
      *self.modules.get_mut("megajump").unwrap() = (self.megaJump).to_string();
      *self.modules.get_mut("bhop").unwrap() = (self.Bhop).to_string();
      *self.modules.get_mut("speed").unwrap() = (self.speed).to_string();
  }

  pub fn autoclicker(&self) {
    let mouse = mouse_rs::Mouse::new();
    loop {
      if unsafe { GetAsyncKeyState(VK_LBUTTON) }as u16 & 0x8000 != 0 {
        mouse.release(&mouse_rs::types::keys::Keys::LEFT).unwrap();
        mouse.press(&mouse_rs::types::keys::Keys::LEFT).unwrap();

        std::thread::sleep(std::time::Duration::from_millis((((self.autoClicker * 10) - 300).abs()) as u64));
      }
    }
  }

  pub fn writer_bytes(&mut self, process_name: &str) {
    let mut changed = std::collections::HashMap::new();
    let binding = self.modules.clone(); // clone BTreeMap, for bypass ownership

    for i in 0..MODULES.len() {
      changed.insert(MODULES[i], (None::<usize>, None::<usize>));

      let get_key = binding.get(MODULES[i]).unwrap();

      if MODULES[i].eq("reach") {
        match memory_parser(MODULES_DISABLE[0], 30, process_name, Some(self.reach), None) {
          Ok(q) => *changed.get_mut("reach").unwrap() = (Some(q.0), Some(q.1)), // change module name value, with the returned values of memory_parser (scanned quantity and changed quantity)
          Err(e) => eprintln!("{}", e),
        }
      }

      if get_key.eq("true") {
        match memory_parser(MODULES_DISABLE[i], 30, process_name,None, Some(MODULES_ENABLE[i - 1])) {
          Ok(q) => *changed.get_mut(MODULES[i]).unwrap() = (Some(q.0), Some(q.1)), // change module name value, with the returned values of memory_parser (scanned quantity and changed quantity)
          Err(e) => eprintln!("{}", e),
        }
      } else if i > 0{
        match memory_parser(MODULES_ENABLE[i -1], 30, process_name, None, Some(MODULES_DISABLE[i])) {
          Ok(q) => *changed.get_mut(MODULES[i]).unwrap() = (Some(q.0), Some(q.1)), // change module name value, with the returned values of memory_parser (scanned quantity and changed quantity)
          Err(e) => eprintln!("{}", e),
        }
      }
    }

    print!("\x1B[2J\x1B[1;1H"); // clear console (cmd)

    for (key, val) in changed.iter() {
      if val.0 == None || val.1 == None {
        continue;
      } //jump if no found results for address

      println!("{} scanned {} in {}, changed {} to {}", key, val.0.unwrap(), val.1.unwrap(), val.1.unwrap(), self.modules.get(*key).unwrap());
    }

    println!("autoclicker enable");
    self.autoclicker(); // start autoclicker
  }
}
