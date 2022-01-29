use iced::{
    text_input::{self,TextInput},Text,Sandbox,Settings,Element,Column, Button,Length
};

use super::{text_input_enter, button_next, automata};


#[derive(Default)]
pub struct AutomataMatricula{
    input: text_input_enter::TextInputEnter,
    button_next: button_next::ButtonNext,
    text_tipo_vehiculo:String,
    value_tipo: String,
    text_estado: String,
    value_estado: String,
    valido_estado: String,
    entrada_t: String
}


#[derive(Debug,Clone)]
pub enum Message{
    DataInputChanged(String),
    ButtonChanged
}


impl Sandbox for AutomataMatricula {
    type Message = Message;
    fn new()-> Self{
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Automata matricula")
    }

    fn update(&mut self,message: Message){
        match message {
            Message::DataInputChanged(mut value) => {
                value.truncate(9);
                self.input.input_value = value;
            }
            Message::ButtonChanged => {
              let mut entra =   automata::automata::ResultMatricula{
                  valido: String::from("0"),
                  entrada_text: self.input.input_value.clone(),
                  entrada_slide: self.input.input_value.clone(),
                  tipo_vehiculo: String::new(),
                  estado: String::new()
              };
               entra.estado_0();
              self.value_tipo = entra.tipo_vehiculo;
              self.value_estado = entra.estado;
              self.valido_estado = entra.valido;
              self.entrada_t = entra.entrada_text;
            }
        }
    }

    fn view(&mut self) -> Element<Message>{
        let input_one = TextInput::new(
            &mut self.input.input,
            "Ingresar Matricula",
            &self.input.input_value,
            Message::DataInputChanged,
        ).size(20).padding(25);
        let button_next = Button::new(
            &mut self.button_next.btn,
            Text::new("Validar matricula")
        ).on_press(Message::ButtonChanged).width(Length::Shrink);

        let text_tipo_vehiculo = Text::new(
            "Tipo vehiculo"
        ).size(20).horizontal_alignment(iced::HorizontalAlignment::Center).width(Length::Fill);
        let value_tipo_vehiculo = Text::new(
            self.value_tipo.clone()
        ).horizontal_alignment(iced::HorizontalAlignment::Center).color([0.5,0.5,0.5]).width(Length::Fill);

        let text_estado = Text::new(
            "estado"
        ).size(20).horizontal_alignment(iced::HorizontalAlignment::Center).width(Length::Fill);
        let value_estado = Text::new(
            self.value_estado.clone()
        ).horizontal_alignment(iced::HorizontalAlignment::Center).color([0.5,0.5,0.5]).width(Length::Fill).height(Length::Shrink);
        let status = if self.valido_estado == "2"  {
            "Error de matricula"
        } else {
            " "
        };
        let error_text = Text::new(
                status
        ).size(20).horizontal_alignment(iced::HorizontalAlignment::Center).width(Length::Fill);

        Column::new()
        .padding(60)
        .push(input_one)
        .push(button_next)
        .push(text_tipo_vehiculo)
        .push(value_tipo_vehiculo)
        .push(text_estado)
        .push(value_estado)
        .push(error_text)
        .push(Text::new("Matricula").size(20).width(Length::Fill))
        .push(Text::new(self.entrada_t.clone()).size(20).width(Length::Fill))
        .into()
    }
}

