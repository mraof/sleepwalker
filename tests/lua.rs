extern crate rlua;

use rlua::{Lua, Error, UserData, UserDataMethods};

#[test]
fn lua_test() {
    let lua = Lua::new();
    let globals = lua.globals();
    let lua_meow = lua.create_function(meow);
    globals.set("meow", lua_meow).unwrap();
    let blob = Blobject {
        coords: (8.0, 8.0),
        jiggle: 0.7,
    };
    globals.set("blob", blob).unwrap();
    lua.exec::<()>(include_str!("assets/something.lua"), None)
        .unwrap();
}

fn meow(_: &Lua, times: u32) -> Result<(), Error> {
    for i in 0..times {
        println!("meow, {}", i);
    }
    Ok(())
}

#[derive(Copy, Clone)]
struct Blobject {
    coords: (f32, f32),
    jiggle: f32,
}

impl UserData for Blobject {
    fn add_methods(methods: &mut UserDataMethods<Self>) {
        methods.add_method_mut("move", |_, blobject: &mut Blobject, (x, y): (f32, f32)| {
            blobject.coords.0 += x as f32 * blobject.jiggle.sqrt();
            blobject.coords.1 += y as f32 * blobject.jiggle.sqrt();
            Ok((blobject.coords.0 as u32, blobject.coords.1 as u32))
        });
    }
}
