extern crate otp_generator;

fn main () {
    let flags = otp_generator::Flags {digits : true, ..Default::default()};
    println!("6 digit Otp = {}", otp_generator::generate(6,&flags).unwrap());
}