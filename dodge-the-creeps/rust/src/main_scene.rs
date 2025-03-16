use crate::hud::Hud;
use crate::mob;
use crate::player;

use godot::classes::{Marker2D, PathFollow2D, RigidBody2D, Timer};
use godot::prelude::*;

use rand::Rng as _;
use std::f32::consts::PI;

// Deriving GodotClass makes the class available to Godot.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    mob_scene: OnReady<Gd<PackedScene>>,
    player: OnReady<Gd<player::Player>>,
    music: OnReady<Gd<AudioStreamPlayer>>,
    death_sound: OnReady<Gd<AudioStreamPlayer>>,
    score: i64,
    base: Base<Node>,
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        // We could also initialize those manually inside ready(), but OnReady automatically defers initialization.
        Self {
            mob_scene: OnReady::new(|| load("res://Mob.tscn")),
            player: OnReady::node("Player"),
            music: OnReady::node("Music"),
            death_sound: OnReady::node("DeathSound"),
            score: 0,
            base,
        }
    }

    fn ready(&mut self) {}
}

#[godot_api]
impl Main {
    #[func]
    fn game_over(&mut self) {
        let mut score_timer = self.base().get_node_as::<Timer>("ScoreTimer");
        let mut mob_timer = self.base().get_node_as::<Timer>("MobTimer");

        score_timer.stop();
        mob_timer.stop();

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        hud.bind_mut().show_game_over();

        self.music.stop();
        self.death_sound.play();
    }

    #[func]
    pub fn new_game(&mut self) {
        let start_position = self.base().get_node_as::<Marker2D>("StartPosition");
        let mut start_timer = self.base().get_node_as::<Timer>("StartTimer");

        self.score = 0;

        self.player.bind_mut().start(start_position.get_position());
        start_timer.start();

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        let hud = hud.bind_mut();
        hud.update_score(self.score);
        hud.show_message("Get Ready".into());

        self.music.play();
    }

    #[func]
    fn on_start_timer_timeout(&self) {
        let mut mob_timer = self.base().get_node_as::<Timer>("MobTimer");
        let mut score_timer = self.base().get_node_as::<Timer>("ScoreTimer");
        mob_timer.start();
        score_timer.start();
    }

    #[func]
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        hud.bind_mut().update_score(self.score);
    }

    #[func]
    fn on_mob_timer_timeout(&mut self) {
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");

        let mut mob_scene = self.mob_scene.instantiate_as::<RigidBody2D>();

        let mut rng = rand::thread_rng();
        let progress = rng.gen_range(u32::MIN..u32::MAX);

        mob_spawn_location.set_progress(progress as f32);
        mob_scene.set_position(mob_spawn_location.get_position());

        let mut direction = mob_spawn_location.get_rotation() + PI / 2.0;
        direction += rng.gen_range(-PI / 4.0..PI / 4.0);

        mob_scene.set_rotation(direction);

        self.base_mut().add_child(&mob_scene);

        let mut mob = mob_scene.cast::<mob::Mob>();
        let range = {
            // Local scope to bind `mob` user object
            let mob = mob.bind();
            rng.gen_range(mob.min_speed..mob.max_speed)
        };

        mob.set_linear_velocity(Vector2::new(range, 0.0).rotated(real::from_f32(direction)));

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        hud.connect("start_game", &mob.callable("on_start_game"));
    }
}
