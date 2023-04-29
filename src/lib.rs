#![no_std]
#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

mod helpers;
mod music;
mod sprites;
mod world;

static mut WORLD: Option<world::World> = None;

#[no_mangle]
fn update() {
    // init
    unsafe {
        if WORLD.is_none() {
            WORLD = Some(world::World::new());
        }
    }

    // update
    unsafe {
        WORLD.as_mut().unwrap().update();
    }

    // draw
    helpers::fill(3);
    unsafe {
        WORLD.as_mut().unwrap().draw();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
