#[allow(dead_code)]
pub fn i32_to_f32(source: i32) -> f32 {
    unsafe { std::mem::transmute::<i32, f32>(source) }
}

#[allow(dead_code)]
pub fn f32_to_i32(source: f32) -> i32 {
    unsafe { std::mem::transmute::<f32, i32>(source) }
}

#[allow(dead_code)]
pub fn i64_to_i32seq(source: i64) -> [i32; 2] {
    unsafe { std::mem::transmute::<i64, [i32; 2]>(source) }
}

#[allow(dead_code)]
pub fn i32seq_to_i64(source: [i32; 2]) -> i64 {
    unsafe { std::mem::transmute::<[i32; 2], i64>(source) }
}

#[allow(dead_code)]
pub fn f64_to_i32seq(source: f64) -> [i32; 2] {
    unsafe { std::mem::transmute::<f64, [i32; 2]>(source) }
}

#[allow(dead_code)]
pub fn i32seq_to_f64(source: [i32; 2]) -> f64 {
    unsafe { std::mem::transmute::<[i32; 2], f64>(source) }
}
