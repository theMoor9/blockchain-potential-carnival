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
pub enum ValidMultiplier {
    One = 1,
    Two = 2,
    Three = 3
}
/// Macro
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
    pub macros: Vec<Macro>,
    pub total_score: i32,
}