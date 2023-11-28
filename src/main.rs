use economy::Good;

mod economy;

fn create_goods() -> Vec<Good> {
    vec![
        Good::new("Ore".to_owned(), 40),
        Good::new("Refined Metals".to_owned(), 110),
        Good::new("Methane".to_owned(), 10),
        Good::new("Graphene".to_owned(), 80),
        Good::new("Hull Item".to_owned(), 200),
        Good::new("Food Ration".to_owned(), 60),
    ]
}


fn main() {
    
    let goods = create_goods();

    println!("Hello, world!");
}
