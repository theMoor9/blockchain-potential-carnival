#[derive(Copy,Clone)]
pub enum ValidScore {
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
#[derive(Copy,Clone)]
pub enum ValidMultiplier {
    One = 1,
    Two = 2,
    Three = 3
}

pub struct Question  {
    pub question: Option<String> ,
    pub score: Option<ValidScore>,
}
pub struct Macro {
    pub name: Option<String>,
    pub description: Option<String>,
    pub weight: Option<ValidMultiplier>,
    pub questions: Vec<Question>,
}
pub struct IcoEvaluation {
    pub name: String,
    pub total_score: i16,
    pub investment_suitability_value: u8,
    pub macros: Vec<Macro>,
}


impl Macro {
    pub fn new(name: String, description: String, questions: Vec<Question>) -> Macro {
        Macro {
            name: Some(name),
            description: Some(description),
            weight: None,
            questions: questions,
        }
    }
}
impl Question {
    pub fn new(question: String) -> Question {
        Question {
            question: Some(question),
            score: None,
        }
    }
}


impl IcoEvaluation {
    pub fn new(ico_name:String, score: i16, isv: u8, questionnaire: Vec<Macro>) -> IcoEvaluation {
        IcoEvaluation {
            name: ico_name,
            total_score: score,
            investment_suitability_value: isv,
            macros: questionnaire,
        }
    }
}