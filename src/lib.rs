use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fac(n:f64) -> f64{
    if n <= 1.0{
        1.0
    }else{
        n*fac(n-1.0)
    }
}