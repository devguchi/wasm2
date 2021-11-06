use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Counter {
    key: char,
    count: u32
}

#[wasm_bindgen]
impl Counter {
    pub fn new(key:char) -> Counter {
        log(&format!("Counter::new({})", key));
        Counter { key: key, count: 0 }
    }

    pub fn count(&self) -> u32 {
        self.count
     }
     
     pub fn increment(&mut self) {
         self.count += 1;
     }
}
