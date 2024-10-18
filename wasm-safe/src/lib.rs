use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlSelectElement};



#[wasm_bindgen]
pub fn verify_flag(flag: String) -> u8 {
    let mut answer: u8 = 0;
    if verify_part_one() == true {
        answer ^= 0b1;
    };
    if verify_part_two(flag) == true {
        answer = answer ^ 0b010;
    };
    if verify_part_three() == true {
        answer = answer ^ 0b100;
    };
    answer
}

#[wasm_bindgen]
pub fn verify_part_one() -> bool {
    let part_1 = match get_element_by_id("part-1") {
        Some(elem) => elem,
        None => {
            return false;
        }
    };

    let p1 = part_1.value();
    if p1 != "W4sm" {
        return false;
    }
    true
}
#[wasm_bindgen]
pub fn verify_part_three() -> bool {
    let part_3 = match get_input_element_by_id("part-3") {
        Some(elem) => elem,
        None => {
            return false;
        }
    };
    let p3_str = part_3.value();

    let p3 = match p3_str.parse::<i32>() {
        Ok(e) => e,
        Err(_) => {
            return false;
        }
    };
    if 3 * (p3 + 31) != 4107 {
        return false;
    }
    true
}

#[wasm_bindgen]
pub fn verify_part_two(f: String) -> bool {
    let buffer = [
        16, 22, 24, 8, 24, 2, 16, 6, 24, 9, 27, 85, 1, 17, 17, 81, 13, 22,
    ];
    if f.len() != 18 {
        return false;
    }
    for (idx, c) in f.bytes().enumerate() {
        if idx % 2 == 0 {
            if (buffer[idx] ^ 121) != c {
                return false;
            }
        } else {
            if (buffer[idx] ^ 101) != c {
                return false;
            }
        }
    }
    true
}

fn get_element_by_id(id: &str) -> Option<HtmlSelectElement> {
    let document = web_sys::window()?.document()?;
    document
        .get_element_by_id(id)?
        .dyn_into::<HtmlSelectElement>()
        .ok()
}

fn get_input_element_by_id(id: &str) -> Option<HtmlInputElement> {
    web_sys::window()?
        .document()?
        .get_element_by_id(id)?
        .dyn_into::<HtmlInputElement>()
        .ok()
}
