use bevy::prelude::*;
use std::ops::Add;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_startup_system(say_hello_system)
        .insert_resource(XpGain(10))
        .add_startup_system(spawn_player)
        .add_system(action_system)
        .run();
}

fn say_hello_system(
    xp_gain: Res<XpGain>
) {
    println!("Hello bevy! You will gain {} xp for each action!", xp_gain.0);
}

fn spawn_player(mut commands: Commands) {
    println!("Spawning player!");
    let some_player_entity = commands.spawn().insert(Player).id();
    println!("Spawned player {:?}", some_player_entity);
    let player_bundle = commands.spawn_bundle(PlayerBundle::default()).id();
    println!("Spawned player {:?}", player_bundle);
}

fn action_system(
    mut query: Query<(&mut Xp, &Player)>,
    xp_gain: Res<XpGain>,
) {
    for (xp, player) in query.iter_mut() {
        let mut xp: Mut<Xp> = xp;
        let player: &Player = player;
        // let new_xp_val = xp.as_ref() + xp_gain.as_ref();
        *xp = xp.as_ref() + xp_gain.as_ref();
        println!("Player {:?} has new xp: {}", player, xp.0);
    }
}

#[derive(Component)]
struct Health(i32);

#[derive(Component)]
struct Xp(i32);

impl Add<&XpGain> for &Xp {
    type Output = Xp;

    fn add(self, rhs: &XpGain) -> Self::Output {
        Xp(self.0 + rhs.0)
    }
}

#[derive(Component, Debug)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    health: Health,
    xp: Xp,
    _p: Player,
}

struct XpGain(i32);

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            health: Health(100),
            xp: Xp(0),
            _p: Player,
        }
    }
}