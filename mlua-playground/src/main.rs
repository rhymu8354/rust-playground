#![warn(clippy::pedantic)]

fn with_lua<F, R>(
    lua: &mut mlua::Lua,
    f: F
) where
    F: FnOnce(&mut mlua::Lua) -> mlua::Result<R>
{
    if let Err(error) = f(lua) {
        println!("Lua had a problem: {}", error);
    }
}

fn main() {
    println!("Hello, world!  Let me introduce you to my good friend, Lua!");

    // Construct a Lua interpreter.
    let mut lua = mlua::Lua::new();

    // Load and execute an introduction script.
    // This should print stuff and also define a Lua function "report"
    // we should be able to call later.
    with_lua(&mut lua, |lua| -> mlua::Result<()> {
        lua.load(
            r#"
                print("Hello, I am Lua.  How are you today?")
                function report(x, y)
                    local sum = x + y
                    return "The sum is '" .. sum .. "'"
                end
            "#,
        )
            .set_name("introduction")?
            .exec()?;
        Ok(())
    });

    // Call the Lua "report" function now.
    with_lua(&mut lua, |lua| -> mlua::Result<()> {
        let globals = lua.globals();
        let report = globals.get::<_, mlua::Function>("report")?;
        let result = report.call::<(i32, i32), String>((40, 2))?;
        println!("Lua said this: {}", result);
        Ok(())
    });

    // Let's make a Rust function to give to Lua to be called from there.
    with_lua(&mut lua, |lua| -> mlua::Result<()> {
        let ambassador = lua.create_function(|lua, (x, y): (i32, i32)| {
            lua.globals().set("secret", "foobar")?;
            Ok(x + y)
        })?;
        lua.globals().set("ambassador", ambassador)?;
        lua.load(
            r#"
                print("Using your ambassador: " .. ambassador(80, 2))
            "#,
        )
            .exec()?;
        let secret = lua.globals().get::<_, String>("secret").unwrap();
        println!("The secret is {}", secret);
        Ok(())
    });

    // Let's try to have Lua give us a function returned from a function,
    // and try to call it.
    with_lua(&mut lua, |lua| -> mlua::Result<()> {
        let lua_function = lua.load(
            r#"
                return function(x, y)
                    return x + y
                end
            "#,
        )
            .eval::<mlua::Function>()?;
        let sum = lua_function.call::<(i32, i32), i32>((5, 6))?;
        println!("Lua gave us back {} when we called their closure", sum);
        Ok(())
    });
}
