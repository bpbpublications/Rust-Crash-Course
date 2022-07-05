mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-wasm!");
}

#[wasm_bindgen]
pub fn login_success(uname: &str) {
    alert(&format!("Hello, {}!", uname))
}

#[wasm_bindgen]
pub fn login_failure() {
    alert("Invalid credentials !!! Try again...");
}

#[wasm_bindgen]
pub fn login(uname: &str, pwd: &str) -> bool{
    if uname == "abhishek" && pwd == "rust" {
        return true;
    }
    else {
        return false;
    }
}
