use std::io::{self, Write};
use textwrap::{fill, termwidth};
use std::thread;
use std::time::Duration;
use lib::monitor::MonitorManager;



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
struct Macro {
    mlt: ValidMultiplier,
    micros: Vec<ValidScore>,
}

fn summatory_total_score(areas : Vec<Macro>) -> i16 {
    /*  
    Multiply the area's Macro's ValidMultiplier value
    by the summatory of the ValidScore values in the micros vector
    Return then the summatory of the Macro's moltiplications
    */
}


fn main() {
    let _abslt_score: i16 = 405; //±405

    

}