#![allow(dead_code)]
#![allow(unused)]
//#[derive(Debug)]
struct Base {
    #[cfg(feature = "core")]
    core_data: u32,
    #[cfg(feature = "anyone_can_pay")]
    anyone_can_pay_data: u64,
}

impl Base {
    pub fn new() -> Self {
        Base { 
            #[cfg(feature = "core")]
            core_data: u32,
            #[cfg(feature = "anyone_can_pay")]
            anyone_can_pay_data: u64,
        }
    }
}

fn main() {
    //println!("{:?}", Base{});
}
