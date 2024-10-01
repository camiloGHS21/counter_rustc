mod gui;
mod interface;
mod states;
use interface::counter::Counter;
use states::update::update;
use gui::view::view;
pub fn main() -> iced::Result {
    iced::application(
        "A cool counter",
        update, // Actualiza el estado del contador
        view,        // Crea una nueva instancia de Counter
    )
    .theme(theme)
    .default_font(iced::Font::with_name("Cascadia Code"))
    .resizable(false)
    .window_size(iced::Size::new(300.0, 300.0))
    .decorations(false)
    .transparent(true)
    .centered()
    .run()
}

fn theme(_counter: &Counter) -> iced::Theme {
    iced::Theme::CatppuccinMocha
}
