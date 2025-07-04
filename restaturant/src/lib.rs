mod front_of_house {
    mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    
    mod serving {
        use crate::front_of_house::hosting as h;
        
        fn take_order() {
            super::hosting::add_to_waitlist();
            h::add_to_waitlist();
        }
        
        fn serve_order() {}
        fn take_payment() {}
    }
}







// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
// 
// 

