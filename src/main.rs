mod oxide_lib;

use crate::oxide_lib::di::*;
use oxide_macro::Component;
use std::any::Any;
use std::hash::{Hash, Hasher};

#[derive(Component, Clone)]
struct SmallService {
    #[wired]
    tiny_service: TinyService,
}

#[derive(Component, Clone)]
struct TinyService;

impl TinyService {
    fn call3(&self) {
        println!("i am so tiny!");
    }
}

impl SmallService {
    fn call1(&self) {
        println!("small service starts");
        self.tiny_service.call3();
        println!("small service finishes");
    }
}
#[derive(Component)]
struct BigService {
    #[wired]
    small_service: SmallService,
    #[wired]
    tiny_service: TinyService,
}

impl BigService {
    fn call2(&self) {
        println!("big service started");
        println!("also, let the tiny speak a little");
        self.tiny_service.call3();
        println!("okay thank you tiny");
        self.small_service.call1();
        println!("big service finished");
    }
}

fn main() {
    let mut ctx = Context::new();
    ctx.init();
    ctx.get::<BigService>().unwrap().call2();
}
