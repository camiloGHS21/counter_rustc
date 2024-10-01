use crate::interface::counter::Counter;
use crate::interface::message::Message;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill, Theme};
pub fn view(counter: &Counter) -> Element<Message> {
    let exit_button = container(button("Exit").on_press(Message::Exit).padding(10).style(
        |theme: &Theme, status| {
            let _palette = theme.palette();

            match status {
                button::Status::Active => {
                    button::Style {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(
                            0.8, 0.0, 0.0,
                        ))), // Fondo rojo
                        text_color: iced::Color::WHITE, // Texto blanco      // Radio de borde
                        ..Default::default()
                    }
                }
                _ => button::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(
                        0.6, 0.0, 0.0,
                    ))), // Fondo más oscuro al no estar activo
                    text_color: iced::Color::WHITE, // Texto blanco
                    ..Default::default()
                },
            }
        },
    ))
    .align_x(iced::alignment::Horizontal::Right) // Alineado a la derecha
    .width(Fill); // Ocupar todo el ancho

    // Contenedor principal con el resto del contenido centrado
    container(
        column![
            exit_button,
            row![text(counter.value.to_string()).size(50)].spacing(10),
            row![
                button("Increment").on_press(Message::Increment),
                button("Decrement").on_press(Message::Decrement)
            ]
            .spacing(10),
        ]
        .align_x(iced::alignment::Horizontal::Center)
        .spacing(10),
    )
    .into()
}
