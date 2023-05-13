mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, test!");
}

#[wasm_bindgen]
pub fn untar_lzma(input : String, output : String) -> bool
{
  if let Ok(file) = std::fs::File::open(input)
  {
    let mut f = std::io::BufReader::new(file);
    let mut o = Vec::new();


    match lzma_rs::xz_decompress( &mut f, &mut o) {
      Ok(()) => {

      },
      Err(..)=> {
        return false;
      }
    }

    let mut a = tar::Archive::new(o.as_slice());
    match a.unpack(output) {
      Ok(_r)=> {
        return true
      },
      Err(e)=> {
        println!("{}", e);
        return false
      }
    }
  }
false
}