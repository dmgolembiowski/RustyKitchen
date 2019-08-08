use crate::front_of_house::hosting;
// ^ Better to use this idiomatic path than to use:
//      `use crate::front_of_house::hosting::add_to_waitlist;`
//      because no one knows where `add_to_waitlist` came from;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
