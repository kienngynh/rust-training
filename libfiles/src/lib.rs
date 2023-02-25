#[allow(dead_code)]
fn call_order() {}
#[allow(dead_code)]
pub mod back_house {
    fn cook_order() {}
    fn fix_order() {
        super::call_order();
        cook_order()
    }
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String,
    }
    pub enum Pre{
        Soup,
        Salad,
    }
    impl Breakfast {
        pub fn monday(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("banana"),
            }
        }
    }
}
mod front_house;

#[allow(dead_code)]
fn eat_at_restaurant() {
    let mut _order = back_house::Breakfast::monday("Fish");
    _order.toast = String::from("Chicken");
    
    let _order1 = back_house::Breakfast{
        toast:String::from("Wheat"),
        fruit:String::from("apple"),
    };

    let _order2=back_house::Pre::Salad;
}
