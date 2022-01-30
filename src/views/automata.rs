pub mod automata{
    
    const  ABC: &str = "ABCDEFGHJKMNLPRSTUVWXYZ";
    const  ABC2: &str = "GHJKMNLPRSTUVW";
    const  ABC3: &str = "ABCDEF";

    const  NUMEROS: &str = "123456789";

    const NUMERO: &str = "0123456789";


   pub struct ResultMatricula{
      pub  valido: String,
      pub  entrada_text: String,
      pub  entrada_slide: String,
      pub  tipo_vehiculo: String,
      pub  estado: String
    }

    impl ResultMatricula {
        pub fn estado_0(&mut self){
            self.valido = String::from("2");
            if self.entrada_slide.len() == 9 {
                if self.entrada_slide[0..1].trim() == "G"{
                    self.entrada_slide = self.entrada_slide[1..].to_string();
                    self.estado_1();
                }else if self.entrada_slide[0..1].trim() == "H"{
                    self.entrada_slide = self.entrada_slide[1..].to_string();
                    self.estado_10();
                }else if self.entrada_slide[0..1].trim() == "F" {
                    self.entrada_slide = self.entrada_slide[1..].to_string();
                    self.estado_15();
                }
            } else if self.entrada_slide.len() == 0 {
                self.valido = String::from("0");
            }
        }
        
    }

    trait E1{
        fn estado_1(&mut self);
    }

    impl E1 for ResultMatricula{
         fn estado_1(&mut self){
             
            if ABC2.contains(self.entrada_slide[0..1].trim()) {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_2();
            } else if self.entrada_slide[..1].trim()  == "Y" || self.entrada_slide[..1].trim() == "X" {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_22();
            }else if self.entrada_slide[..1].trim() == "Z"{
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_11();
            }else if ABC3.contains(self.entrada_slide[0..1].trim()){
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_21();
            }
         }
    }

    trait E2{
        fn estado_2(&mut self);
    }

    impl E2 for ResultMatricula{
        fn estado_2(&mut self) {
            if ABC.contains(self.entrada_slide[0..1].trim()) {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_3();
               
            }else if self.entrada_slide[..1].trim() == "-" {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_17();
            }
        }
    }

    trait E3{
        fn estado_3(&mut self);
    }

    impl E3 for ResultMatricula{
        fn estado_3(&mut self) {
            if self.entrada_slide[0..1].trim() == "-" {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_4();
            }
        }
    }

    trait E4{
        fn estado_4(&mut self);
    }

    impl E4 for ResultMatricula{
        fn estado_4(&mut self) {
            if self.entrada_slide[0..1].trim() == "0"{
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_5();
            }else if NUMEROS.contains(self.entrada_slide[0..1].trim())  {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_14();
            }
        }
    }

    trait E5{
        fn estado_5(&mut self);
    }

    impl E5 for ResultMatricula{
        fn estado_5(&mut self) {
            if self.entrada_slide[0..1].trim() == "0"{
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_6();
            }else if NUMEROS.contains(self.entrada_slide[0..1].trim())  {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_13();
            }
        }
    }

    trait E6{
        fn estado_6(&mut self);
    }

    impl E6 for ResultMatricula{
        fn estado_6(&mut self) {
            if NUMEROS.contains(self.entrada_slide[0..1].trim()){
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_7();
            }
        }
    }

    trait E7{
        fn estado_7(&mut self);
    }

    impl E7 for ResultMatricula{
        fn estado_7(&mut self) {
            if self.entrada_slide[0..1].trim() == "-" {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_8();
            }
        }
    }

    trait E8{
        fn estado_8(&mut self);
    }

    impl E8 for ResultMatricula{
        fn estado_8(&mut self) {
            if ABC.contains(self.entrada_slide[0..1].trim()) {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_9();
            }
        }
    }

    trait E9{
        fn estado_9(&mut self);
    }

    impl E9 for ResultMatricula{
        fn estado_9(&mut self) {
            if self.entrada_text[..1].trim() == "G" && (ABC2.contains(self.entrada_text[1..2].trim()) || self.entrada_text[1..2].trim() == "X" || self.entrada_text[1..2].trim() == "Y") && ABC.contains(self.entrada_text[2..3].trim()){
                self.validar_datos(String::from("Automovil"), String::from("Guanajuato"));
            } else if ((self.entrada_text[..1].trim() == "G" && self.entrada_text[1..2].trim() == "Z") || (self.entrada_text[..1].trim() == "H" && ABC3.contains(self.entrada_text[1..2].trim()))) && ABC.contains(self.entrada_text[2..3].trim()){
                self.validar_datos(String::from("Automovil"), String::from("Guerrero"));
            } else if ((self.entrada_text[..1].trim() == "F" && (self.entrada_text[1..2].trim() == "Y" || self.entrada_text[1..2].trim() == "Z")) || (self.entrada_text[..1].trim() == "G" && (ABC3.contains(self.entrada_text[1..2].trim()) || ABC2.contains(self.entrada_text[1..2].trim())))) && self.entrada_text[2..3].trim() == "-"{
                self.validar_datos(String::from("Camion"), String::from("Guanajuato"));
            } else {
                self.validar_datos(String::from("Camion"), String::from("Guerrero"));
            } 
            self.valido= String::from("1");
            println!("Tipo: {}", self.tipo_vehiculo);
            println!("Estado: {}", self.estado);
            println!("Matricula: {}", self.entrada_text);
        }
    }

    trait VD{
        fn validar_datos(&mut self, tipo: String, estadoP: String);
    }
   

    impl VD for ResultMatricula{
        fn validar_datos(&mut self, tipo: String, estadoP: String) {
            self.tipo_vehiculo = tipo;
            self.estado = estadoP;
        }
    }

    trait E10{
        fn estado_10(&mut self);
    }

    impl E10 for ResultMatricula{
        fn estado_10(&mut self) {
            if self.entrada_slide[..1].trim() == "G"{
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_23();
            }else if  ABC3.contains(self.entrada_slide[0..1].trim()) {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_11();
            }
        }
    }

    trait E11{
        fn estado_11(&mut self);
    }

    impl E11 for ResultMatricula{
        fn estado_11(&mut self) {
            if ABC.contains(self.entrada_slide[..1].trim()){
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_12();
               
            } else if self.entrada_slide[..1].trim() == "-"{
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_17();
                
            }
        }
    }

    trait E12{
        fn estado_12(&mut self);
    }

    impl E12 for ResultMatricula{
        fn estado_12(&mut self) {
            if self.entrada_slide[..1].trim() == "-" {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_4();
               
            }
        }
    }

    trait E13{
        fn estado_13(&mut self);
    }

    impl E13 for ResultMatricula{
        fn estado_13(&mut self) {
            if NUMERO.contains(self.entrada_slide[0..1].trim()) {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_7();
                
            }
        }
    }

    trait E14{
        fn estado_14(&mut self);
    }

    impl E14 for ResultMatricula{
        fn estado_14(&mut self) {
            if NUMERO.contains(self.entrada_slide[0..1].trim()) {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_13();
                
            }
        }
    }

    trait E15{
        fn estado_15(&mut self);
    }

    impl E15 for ResultMatricula{
        fn estado_15(&mut self) {
            if self.entrada_slide[..1].trim() == "Y" || self.entrada_slide[..1].trim() == "Z"{
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_16();
                
            }
        }
    }

    trait E16{
        fn estado_16(&mut self);
    }

    impl E16 for ResultMatricula{
        fn estado_16(&mut self) {
            if self.entrada_slide[..1].trim() == "-"{
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_17();
                
            }
        }
    }

    trait E17{
        fn estado_17(&mut self);
    }

    impl E17 for ResultMatricula{
        fn estado_17(&mut self) {
            if NUMEROS.contains(self.entrada_slide[..1].trim()){
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_18();
               
            }else if self.entrada_slide[..1].trim() == "0"{
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_4();
                
            }
        }
    }

    trait E18{
        fn estado_18(&mut self);
    }

    impl E18 for ResultMatricula{
        fn estado_18(&mut self) {
            if NUMERO.contains(self.entrada_slide[..1].trim()){
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_19();
               
            }
        }
    }

    trait E19{
        fn estado_19(&mut self);
    }

    impl E19 for ResultMatricula{
        fn estado_19(&mut self) {
            if NUMERO.contains(self.entrada_slide[..1].trim()){
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_20();
               
            }
        }
    }

    trait E20{
        fn estado_20(&mut self);
    }

    impl E20 for ResultMatricula{
        fn estado_20(&mut self) {
            if NUMERO.contains(self.entrada_slide[..1].trim()){
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_7();
                
            }
        }
    }

    trait E21{
        fn estado_21(&mut self);
    }

    impl E21 for ResultMatricula{
        fn estado_21(&mut self) {
            if self.entrada_slide[..1].trim() == "-" {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_17();
            }
        }
    }

    trait E22{
        fn estado_22(&mut self);
    }

    impl E22 for ResultMatricula{
        fn estado_22(&mut self) {
            if self.entrada_slide[..1].trim() == "-" {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_17();
                
            }else if ABC.contains(self.entrada_slide[..1].trim()){
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_3();
                
            }
        }
    }

    trait E23{
        fn estado_23(&mut self);
    }

    impl E23 for ResultMatricula{
        fn estado_23(&mut self) {
            if self.entrada_slide[..1].trim() == "-" {
                self.entrada_slide = self.entrada_slide[1..].to_string();
                self.estado_17();
            }
        }
    }
    
}