use member_a;
use member_b;

fn main() {
    println!("Hello, world!");
    let res_a = member_a::add_two_numbers(1, 2);
    println!("adding 1 and 2: {}", res_a);

    let res_b = member_b::multiply_two_numbers(4, 3);
    println!("multiplying 4 and 3: {}", res_b);
}
