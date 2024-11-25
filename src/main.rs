use bevy::prelude::*;
use bracket_lib::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BTermBuilder::simple80x50())
        .add_system(tick)
        .run();
}

fn tick(ctx: Res<BracketContext>) {
    ctx.set_active_console(0);
    ctx.cls();
    ctx.print(1, 1, "Hello Bracket-Bevy World ☻");
}
