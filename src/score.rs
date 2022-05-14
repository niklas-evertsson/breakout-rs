use bevy::{
    prelude::{Query, Res},
    text::Text,
    ui::Val,
};

pub const SCORE_TEXT: &str = "Score: ";
pub const SCORE_TEXT_FONT: &str = "fonts/FiraSans-Bold.ttf";
pub const SCORE_NUMBER_FONT: &str = "fonts/FiraMono-Medium.ttf";
pub const SCORE_FONT_SIZE: f32 = 40.0;
pub const SCORE_TEXT_PADDING: Val = Val::Px(5.0);

pub struct Score {
    score: usize,
}

impl Score {
    pub fn new() -> Self {
        Score { score: 0 }
    }

    pub fn add(&mut self, amount: usize) {
        self.score += amount;
    }
}

pub fn update_score_ui(score: Res<Score>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{}", score.score);
}
