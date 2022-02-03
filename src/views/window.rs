use iced::{
    text_input::TextInput, Text, Sandbox, Element, Column, Button, Length, 
    HorizontalAlignment, Color, Row, Container,
};

use super::{text_input_enter, button_next, automata, style2};


#[derive(Default)]
pub struct AutomataMatricula{
    input: text_input_enter::TextInputEnter,
    button_next: button_next::ButtonNext,
    _text_tipo_vehiculo:String,
    value_tipo: String,
    _text_estado: String,
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
               
                //test();
            }
        }
    }

    fn view(&mut self) -> Element<Message>{
        let column_component_input_and_button: Column<_> = Column::new()
        .push(
            TextInput::new(
                &mut self.input.input,
                "Ingresar Matricula",
                &self.input.input_value,
                Message::DataInputChanged,
            )
            .width(Length::Units(300))
            .padding(10),
        )
        .push(
            Button::new(
                &mut self.button_next.btn,
                Text::new("Validar matricula")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(iced::VerticalAlignment::Center),
            ).on_press(Message::ButtonChanged)
            .width(Length::Units(250))
            .style(style2::style::Button)
            .height(Length::Units(35)),
        ).align_items(iced::Align::Center)
        .spacing(20);

        let column_info = if self.valido_estado != "2" {
            Column::new()
            .push(Text::new("Tipo de Vehiculo:").horizontal_alignment(HorizontalAlignment::Center).color(Color::from_rgb(0.1, 0.1, 0.1)))
            .push(Text::new(self.value_tipo.clone()).horizontal_alignment(HorizontalAlignment::Center).color(Color::from_rgb(65.0/255.0,90.0/255.0,109.0/255.0)))
            .push(Text::new("Estado:").horizontal_alignment(HorizontalAlignment::Center).color(Color::from_rgb(0.1,0.1,0.1)))
            .push(Text::new(self.value_estado.clone()).horizontal_alignment(HorizontalAlignment::Center).color(Color::from_rgb(65.0/255.0,90.0/255.0,109.0/255.0)))
            .push(Text::new("Matricula:").horizontal_alignment(HorizontalAlignment::Center).color(Color::from_rgb(0.1,0.1,0.1)))
            .push(Text::new(self.entrada_t.clone()).horizontal_alignment(HorizontalAlignment::Center).color(Color::from_rgb(65.0/255.0,90.0/255.0,109.0/255.0)))
            .spacing(20)
        } else {
            Column::new().push(
                Text::new("La matricula no pertenece a Guanajuato")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(iced::VerticalAlignment::Center)
                    .color(Color::from_rgb(237.0 / 255.0, 44.0 / 255.0, 29.0 / 255.0))
                    .size(20),
            )
            .push(
                Text::new("o Guerrero o es invalida")
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(iced::VerticalAlignment::Center)
                    .color(Color::from_rgb(237.0 / 255.0, 44.0 / 255.0, 29.0 / 255.0))
                    .size(20),
            )
        };

        let row_component: Row<_> = Row::new()
            .push(column_component_input_and_button)
            .push(column_info)
            .spacing(30);

        let column_component_welcome = Column::new()
            .push(
                Text::new("Bienvenido :)")
                    .size(40)
                    .color(Color::from_rgb(0.1, 0.1, 0.1)),
            )
            .push(
                Text::new(
                    "Programa que valida placas de Guanajuato y Guerrero",
                )
                .color(Color::from_rgb(129.0 / 255.0, 129.0 / 255.0, 135.0 / 255.0))
                .size(15),
            )
            .push(row_component)
            .spacing(30)
            .align_items(iced::Align::Center);

        Container::new(column_component_welcome)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style2::style::Container)
            .into()
    }
}

trait C1 {
    fn component_container_body(&self);
}

impl C1 for AutomataMatricula {
    fn component_container_body(&self) {}
}


/*fn test(){
    println!("Guanajuato");
    let abc = "ABCDEFGHJKLMNPRSTUVWXYZ";
    G-G/Y-A/Z001/999A/Z
    let mut entrada_test = String::new();
    Digito 1
    for x in 6..22{
        entrada_test = "G".to_string()+&abc[x..x+1].to_string()+"A"+"-"+"001"+ "-"+"A";
        for y in 0..2{
            for z in 0..9{
                for xx in 0..2{
                    entrada_test = "G".to_string()+&abc[x..x+1].to_string()+&abc[y..y+1].to_string()+"-00"+&z.to_string()+ "-"+&abc[xx..xx+1].to_string();
                    let mut run_automata_test = automata::automata::ResultMatricula{
                        valido: String::from("0"),
                        entrada_text: entrada_test.clone(),
                        entrada_slide: entrada_test,
                        tipo_vehiculo: String::new(),
                        estado: String::new()
                    };

                    run_automata_test.estado_0();

                    if (run_automata_test.valido == String::from("1")){
                        println!("Test paso correctamente, {}",run_automata_test.entrada_text)
                    }else{
                        println!("Test no paso correctamente, {}",run_automata_test.entrada_text)
                    }
                }
            }
        }
    }
}*/
