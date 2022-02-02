
pub mod prueba{
    



    pub fn prueba_c2(){
        println!("Guanajuato");
        let abc = "ABCDEFGHJKLMNPRSTUVWXYZ";
        //G-G/Y-A/Z//001/999//A/Z
        let mut entrada_test = String::new();
        //Digito 1
        for x in 7..22{
            entrada_test = "G"+abc[x..x+1]+"A"+"-"+"001"+"A";
            let run_automata_test = automata::automata::ValidarCFE{
                valid: String::from("0"),
                text_entry: entrada_test,
                slide_entry: entrada_test,
                date_paid: String::new(),
                amount_paid: String::new(),
                service_number: String::new()
            };

            run_automata_test.estado_0();

            if (run_automata_test.valid == "1"){
                println!("Test paso correctamente, {}",run_automata_test.text_entry)
            }else{
                println!("Test no paso correctamente")
            }
        }
    }


}