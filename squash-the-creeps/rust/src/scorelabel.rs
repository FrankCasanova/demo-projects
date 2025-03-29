use godot::classes::Control;
use godot::classes::Label;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct UserInterface {
    score: f32,
    base: Base<Control>,
}

#[godot_api]
impl UserInterface {
    #[func]
    pub fn on_mob_squashed(&mut self) {
        // score += 1
        self.score += 1.0;

        // text = "Score: %s" % score
        let mut label = self.base().get_node_as::<Label>("ScoreLabel");
        label.set_text(format!("Score: {}", self.score).as_str());
    }
}
