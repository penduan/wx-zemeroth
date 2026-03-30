use std::time::Duration;

use mq::{input::KeyCode, math::Vec2};
use ui::{self, Gui, Widget};

use crate::{
    assets,
    screen::{Screen, StackCommand},
    utils, ZResult,
};

#[derive(Clone, Debug)]
enum Message {
    Back,
}

fn help_lines() -> Vec<&'static str> {
    vec![
        "=== Basic Controls ===",
        "Left Click on a unit: Select it",
        "Left Click on empty tile: Move selected unit",
        "Left Click on enemy unit: Attack",
        "Left Click ability icon: Activate ability",
        "Enter / Space: End Turn",
        "Escape: Deselect unit / Cancel ability",
        "",
        "=== Battle Basics ===",
        "Defeat all enemy units to win the battle.",
        "If all your units die, you lose.",
        "",
        "=== Unit Stats ===",
        "Strength: Hit points. Unit dies when it reaches 0.",
        "Attacks: Number of attack actions per turn.",
        "Moves: Number of move actions per turn.",
        "Jokers: Can be used as either an attack OR move.",
        "Reactive Attacks: Triggered when enemies move adjacent.",
        "",
        "=== Abilities ===",
        "Each unit may have special active abilities.",
        "Abilities consume Attacks or Jokers to activate.",
        "After use, abilities enter a cooldown period.",
        "",
        "=== Campaign Mode ===",
        "Complete battles to earn Renown.",
        "Spend Renown to recruit new units or upgrade existing ones.",
        "Surviving units carry over to the next battle.",
        "The campaign ends in victory when all battles are won.",
    ]
}

#[derive(Debug)]
pub struct Help {
    gui: Gui<Message>,
}

impl Help {
    pub fn new() -> ZResult<Self> {
        let font = assets::get().font;
        let mut gui = ui::Gui::new();
        let h = utils::line_heights().normal;
        let h_title = utils::line_heights().big;
        let mut layout = Box::new(ui::VLayout::new().stretchable(true));
        let text_ = |s: &str| ui::Drawable::text(s, font);
        let label = |text: &str| -> ZResult<Box<dyn ui::Widget>> {
            Ok(Box::new(ui::Label::new(text_(text), h)?))
        };
        let label_s = |text: &str| -> ZResult<Box<dyn ui::Widget>> {
            Ok(Box::new(
                ui::Label::new(text_(text), h_title)?.stretchable(true),
            ))
        };
        let spacer = || Box::new(ui::Spacer::new_vertical(h * 0.4));
        layout.add(label_s("~~~ How to Play ~~~")?);
        layout.add(spacer());
        for line in help_lines() {
            if line.is_empty() {
                layout.add(spacer());
            } else {
                layout.add(label(line)?);
            }
        }
        layout.add(spacer());
        {
            let mut button =
                ui::Button::new(text_("back"), h, gui.sender(), Message::Back)?.stretchable(true);
            button.stretch(layout.rect().w / 3.0);
            button.set_stretchable(false);
            layout.add(Box::new(button));
        }
        layout.stretch_to_self();
        let layout = utils::add_offsets_and_bg_big(layout)?;
        let anchor = ui::Anchor(ui::HAnchor::Middle, ui::VAnchor::Middle);
        gui.add(&ui::pack(layout), anchor);
        Ok(Self { gui })
    }
}

impl Screen for Help {
    fn update(&mut self, _dtime: Duration) -> ZResult<StackCommand> {
        Ok(StackCommand::None)
    }

    fn draw(&self) -> ZResult {
        self.gui.draw();
        Ok(())
    }

    fn click(&mut self, pos: Vec2) -> ZResult<StackCommand> {
        let message = self.gui.click(pos);
        match message {
            Some(Message::Back) => Ok(StackCommand::Pop),
            None => Ok(StackCommand::None),
        }
    }

    fn resize(&mut self, aspect_ratio: f32) {
        self.gui.resize_if_needed(aspect_ratio);
    }

    fn move_mouse(&mut self, pos: Vec2) -> ZResult {
        self.gui.move_mouse(pos);
        Ok(())
    }

    fn handle_key_press(&mut self, key: KeyCode) -> ZResult<StackCommand> {
        match key {
            KeyCode::Escape | KeyCode::H => Ok(StackCommand::Pop),
            _ => Ok(StackCommand::None),
        }
    }
}
