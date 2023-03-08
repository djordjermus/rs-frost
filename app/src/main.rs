
fn main() {
    // Vector test
    let x: frost_core::Vector4::<i32> = frost_core::Vector4::<i32>::new(1, 2, 3, 4);
    let y = x * x;
    println!("{}", y);

    let col : frost_core::Color = frost_core::Color::CMYKA { c: 1.0f32, m: -0.5f32, y: 0.0f32, k: 0.5f32, a: 0.67 };
    let rgba = frost_core::RGBA32::try_from(&col).expect("Color conversion failed.");
    println!("{}", rgba);
    return;
}
