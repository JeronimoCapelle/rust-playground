fn main() {
    let og_price = 100;
    let discount = 20;
    println!(
        "item was {og_price}, discount is {discount}, final_price is {}",
        final_price(100, 20)
    );
}

fn final_price(price: i32, discount: i32) -> i32 {
    price - discount
}
