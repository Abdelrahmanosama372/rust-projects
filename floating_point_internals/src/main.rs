const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
    let num: f32 = 42.2;
    let (sign, exponent, fraction) = to_parts(num);
    let (sign_, exponent_, mantissa) = decode(sign, exponent, fraction);
    let num_ = from_parts(sign_, exponent_, mantissa);
    println!("{} -> {}", num, num_);
    println!("field  |  as bits  |  as real number");
    println!("sign   | {:1b}        | {}", sign, sign_);
    println!("exponent | {:8b}        | {}", exponent, exponent_);
    println!("mantissa  | {:23b}       | {}", fraction, mantissa)
}

fn to_parts(num: f32) -> (u32, u32, u32)
{
    let n = num.to_bits();
    let sign = n >> 31;
    let exponent = (n >> 23) & 0xFF;
    let fraction = n & 0x7FFFFF;
    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32)
{
    let sign_ = (-1_f32).powf(sign as f32);
    let exponent_ = (exponent as i32 - BIAS) as f32;
    let exponent_ = RADIX.powf(exponent_);
    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let bit = fraction & (1 << i);
        if bit != 0 {
            let weight = 2_f32.powf((i - 23) as f32);
            mantissa += weight;
        }
    }
    (sign_, exponent_, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32
{
    sign * exponent * mantissa
}
