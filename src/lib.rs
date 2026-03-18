mod front_of_house{
    // add code here
    pub mod hosting{
        pub fn add_to_waitlist()  {
            todo!();
        }
    }
}
use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
hosting::add_to_waitlist;
}
