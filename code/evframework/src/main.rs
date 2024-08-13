use terminal::output_manager;
use terminal::questionary;
use terminal::models::{ValidScore, ValidMultiplier, Question, Macro};


fn summatory_total_score(areas : Vec<Macro>) -> i16 {
    let _abslt_score: i16 = 405; //±405
    /*  
    Multiply the area's Macro's ValidMultiplier value
    by the summatory of the ValidScore values in the micros vector
    Return then the summatory of the Macro's moltiplications
    Normalized with the _abslt_score
    */
    return 0;
}


fn main() {
    let mut areas: Vec<Macro> = Vec::new(); // Vettore di Macro per il questionario

    output_manager::welcome();

    if output_manager::menu().is_ok() {
        // `questionary::display` modifica direttamente `areas`
        questionary::display(&mut areas);
        summatory_total_score(areas);

        // Generazione del documento PDF con i punteggi e le domande
    }
}

