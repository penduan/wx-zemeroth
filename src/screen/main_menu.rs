use std::{
    sync::mpsc::{channel, Receiver},
    time::Duration,
};

use log::trace;
use mq::{input::KeyCode, math::Vec2};
use ui::{self, Widget};

use crate::{
    assets,
    core::battle::{scenario, state},
    screen::{self, Screen, StackCommand},
    utils, ZResult,
};

#[derive(Copy, Clone, Debug)]
enum Message {
    #[cfg_attr(target_arch = "wasm32", allow(unused))] // can't quit WASM so it's not used there
    Exit,

    StartInstant,

    StartCampaign,

    HowToPlay,
}

fn make_gui() -> ZResult<ui::Gui<Message>> {
    let font = assets::get().font;
    let mut gui = ui::Gui::new();
    let h = utils::line_heights().large;
    let h_title = utils::line_heights().large;
    let space = || Box::new(ui::Spacer::new_vertical(h / 8.0));
    let button = &mut |text, message| -> ZResult<_> {
        let text = ui::Drawable::text(text, font);
        let b = ui::Button::new(text, h, gui.sender(), message)?.stretchable(true);
        Ok(Box::new(b))
    };
    let mut layout = Box::new(ui::VLayout::new().stretchable(true));
    // Game title
    {
        let title_text = ui::Drawable::text("Zemeroth", font);
        let title_label = ui::Label::new(title_text, h_title)?.stretchable(true);
        layout.add(Box::new(title_label));
        layout.add(space());
        layout.add(Box::new(ui::Spacer::new_vertical(h / 4.0)));
    }
    layout.add(button("demo battle", Message::StartInstant)?);
    layout.add(space());
    layout.add(button("campaign", Message::StartCampaign)?);
    layout.add(space());
    layout.add(button("how to play", Message::HowToPlay)?);
    #[cfg(not(target_arch = "wasm32"))] // can't quit WASM
    {
        layout.add(space());
        layout.add(button("exit", Message::Exit)?);
    }
    layout.stretch_to_self();
    let layout = utils::add_offsets_and_bg_big(layout)?;
    let anchor = ui::Anchor(ui::HAnchor::Middle, ui::VAnchor::Middle);
    gui.add(&ui::pack(layout), anchor);
    // Version label in bottom-right corner
    {
        let version_text = ui::Drawable::text(concat!("v", env!("CARGO_PKG_VERSION")), font);
        let h_small = utils::line_heights().small;
        let version_label = ui::Label::new(version_text, h_small)?;
        let anchor = ui::Anchor(ui::HAnchor::Right, ui::VAnchor::Bottom);
        gui.add(&ui::pack(version_label), anchor);
    }
    Ok(gui)
}

#[derive(Debug)]
pub struct MainMenu {
    gui: ui::Gui<Message>,
    receiver_battle_result: Option<Receiver<Option<state::BattleResult>>>,
}

impl MainMenu {
    pub fn new() -> ZResult<Self> {
        let gui = make_gui()?;
        Ok(Self {
            gui,
            receiver_battle_result: None,
        })
    }
}

impl Screen for MainMenu {
    fn update(&mut self, _: Duration) -> ZResult<StackCommand> {
        Ok(StackCommand::None)
    }

    fn draw(&self) -> ZResult {
        self.gui.draw();
        Ok(())
    }

    fn click(&mut self, pos: Vec2) -> ZResult<StackCommand> {
        let message = self.gui.click(pos);
        trace!("MainMenu: click: pos={:?}, message={:?}", pos, message);
        match message {
            Some(Message::StartInstant) => {
                let prototypes = assets::get().prototypes.clone();
                let scenario = assets::get().demo_scenario.clone();
                let (sender, receiver) = channel();
                self.receiver_battle_result = Some(receiver);
                let battle_type = scenario::BattleType::Skirmish;
                let screen = screen::Battle::new(scenario, battle_type, prototypes, sender)?;
                Ok(StackCommand::PushScreen(Box::new(screen)))
            }
            Some(Message::StartCampaign) => {
                let screen = screen::Campaign::new()?;
                Ok(StackCommand::PushScreen(Box::new(screen)))
            }
            Some(Message::HowToPlay) => {
                let screen = screen::Help::new()?;
                Ok(StackCommand::PushPopup(Box::new(screen)))
            }
            Some(Message::Exit) => Ok(StackCommand::Pop),
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
            KeyCode::H => {
                let screen = screen::Help::new()?;
                Ok(StackCommand::PushPopup(Box::new(screen)))
            }
            _ => Ok(StackCommand::None),
        }
    }
}

