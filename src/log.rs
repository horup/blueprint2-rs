use web_sys::console;


pub fn log(str:&str) {
    let array = js_sys::Array::new();
    array.push(&str.into());
    console::log(&array);
}