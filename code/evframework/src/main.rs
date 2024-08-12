use terminal::output_manager;
use termimal::questionary;


enum ValidScore {
    NFive = -5,
    NFour = -4,
    NThree = -3,
    NTwo = -2,
    NOne = -1,
    Zero = 0,
    POne = 1,
    PTwo = 2,
    PThree = 3,
    PFour = 4,
    PFive = 5,
}
enum ValidMultiplier {
    One = 1,
    Two = 2,
    Three = 3
}
/// Macro
struct Question <'a> {
    question: &'a str ,
    score: ValidScore,
}

struct Macro <'a> {
    name: &'a str,
    weight: ValidMultiplier,
    questions: Vec<Question<'a>>,
}

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

    let placeholder = 0; // This will be the vector of Macro as the questionary to be displayed
    
    output_manager::welcome();
    match output_manager::menu() {
        Ok(_) => {
             // questionary::display takesa a vector of Macro and returns an updated vector of Macro
            match questionary::display(  placeholder ) {
                Ok(results) => {
                    summatory_total_score(results)
                    /*
                    Una volta ricevuto il vettore di Macro aggiornato, inizia la dissezione dei valori 
                    per calcolare il punteggio totale con summatory_total_score(). In seguito si stipulerà
                    il documento pdf con le domande i punteggi per la ICO in questione.
                    */
                },
                Err(e) => println!("Questionary returned Err {}", e)
            }
        }, 
        Err(e) => println!("Menu returned Err {}", e)
    }
}
