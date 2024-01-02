// https://www.codewars.com/kata/57a429e253ba3381850000fb

fn bmi(weight: u32, height: f32) -> &'static str {
    let ratio = weight as f32 / height.powi(2);
    match ratio {
        ratio if ratio <= 18.5 => "Underweight",
        ratio if ratio <= 25.0 => "Normal",
        ratio if ratio <= 30.5 => "Overweight",
        _ => "Obese"
    }
}