use terminal::{
    output_manager, 
    questionary,  
    MenuAction, 
};
use terminal::models::{
    Question, 
    Macro, 
    IcoEvaluation, 
    ValidMultiplier, 
    ValidScore
};
use std::{
    fs::File,
    io::{self, Write},
    process::Command,
    thread,
    time::Duration
};

/* 
ICO Evaluation Framework: Macro Areas and Questions Overview

1. Idea:
   - Evaluates the novelty, necessity, and economic impact of the ICO's core concept.
   - Questions include:
     * What problem is the ICO trying to solve? Is it significant and meaningful?
     * Does the market already exist for this product, or does it need to be created?
     * Is the problem being addressed something that is currently handled by a layer of unnecessary complexity or middlemen?
     * What unique solution does the ICO propose? Is this solution a significant improvement on existing alternatives?
     * Is the idea scalable and capable of achieving wide adoption?

2. Technology:
   - Assesses the technical feasibility, innovation, and scalability of the technology used.
   - Questions include:
     * Is the technological solution proposed by the ICO feasible and currently available?
     * Does the ICO's technology offer a substantial improvement over existing technology?
     * How does the ICO handle data security and privacy concerns?
     * Is the underlying technology behind the ICO robust and scalable?
     * What are the potential technical challenges the ICO might face, and how does it plan to address them?

3. Blockchain Specifics:
   - Examines the choice of blockchain, token economics, and alignment with project needs.
   - Questions include:
     * What blockchain platform is being used, and why was it chosen?
     * Is there a genuine need for blockchain technology in this ICO, or could the problem be solved using traditional technology?
     * Does the ICO introduce a new token, and what is its utility within the ecosystem?
     * How does the tokenomics structure impact the potential for long-term sustainability?
     * Are there any innovative blockchain features, such as smart contracts or decentralized applications, being utilized?

4. Team:
   - Reviews the experience, expertise, and reliability of the team behind the ICO.
   - Questions include:
     * What are the backgrounds and previous experiences of the team members?
     * Does the team have a proven track record in similar ventures or in the blockchain industry?
     * How is the team's expertise relevant to the success of the project?
     * Are there any advisors, and what roles do they play in the project?
     * How does the team plan to grow, and what strategies are in place for scaling up?

5. Execution:
   - Considers the operational strategy, legal compliance, and financial planning of the ICO.
   - Questions include:
     * What is the roadmap for the ICO, and what milestones have already been achieved?
     * How does the ICO plan to use the funds raised?
     * Are there clear marketing and outreach strategies in place?
     * How does the ICO plan to handle regulatory and legal issues?
     * What are the risks associated with the ICO, and how does it plan to mitigate them?

6. Market Potential:
   - Analyzes the market demand, competition, and growth potential of the ICO.
   - Questions include:
     * Is the idea scalable? What is a realistic share of the market?
     * Is there positive convexity to apply a similar idea in related applications?
*/

mod score_math {
    use super::*;
    pub fn weighted_summation(areas : &Vec<Macro>) -> i16 {
        /*  
        Multiply the area's Macro's ValidMultiplier value
        by the summation of the ValidScore values in the micros vector
        Return then the summation of the Macro's moltiplications
        Normalized with the _abslt_score
        */
        let mut w_summation: i16 = 0;
    
        for a in areas.iter() {
            let mut summation: i16 = 0;
            for q in a.questions.iter(){
                summation += q.score.unwrap_or(ValidScore::Zero) as i16;
            }
            summation *= a.weight.unwrap_or(ValidMultiplier::One) as i16;
            w_summation += summation;
        }
        w_summation
    }
    pub fn to_normalize(score_x: i16) -> u8 {
        let abslt_score: i16 = 405; //±405
        (((score_x + abslt_score) * 100) / (abslt_score * 2)) as u8
    }
}

mod create_document {
    use super::*;
    // PDF or Txt to export on personal editor
        // Add Personal Observations sections at the end of question with brief spaces for comments
        // Be sure to add a line at the end of the pdf for signatures
        // Add a footer with the date of the report

    pub fn start(ico_name: String, updated_assessment: Vec<Macro>){

        let new_ico_assessment = IcoEvaluation::new(
            ico_name, 
            score_math::weighted_summation(&updated_assessment),
            score_math::to_normalize(
                score_math::weighted_summation(&updated_assessment)
            ), //investment_suitability_value
            updated_assessment
        );

        let md_file_path = "assets/pdf/md/buffer.md";
        let pdf_file_path = "assets/pdf/EvFrameworkReport.pdf";

        match create_buffer(new_ico_assessment, md_file_path){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };
        match create_pdf(md_file_path, pdf_file_path) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };

    }

    fn create_buffer(ico: IcoEvaluation, file_path: &str) -> io::Result<()> {
        // Create a buffer to write the data
        let mut buffer = File::create(file_path)?;

        // ICO's info
        writeln!(buffer, "#EvFramework Report")?;
        writeln!(buffer)?;
        writeln!(buffer, "##**{}**", ico.name)?;
        writeln!(
            buffer,
            "##**Investment Suitability Value**:   {}", 
            ico.investment_suitability_value
        )?;
        writeln!(buffer, "**Total Score**: {}", ico.total_score)?;
        writeln!(buffer, "---")?;

        // Macro Areas and Question
        for macro_item in &ico.macros {
            if let Some(ref name) = macro_item.name {
                writeln!(buffer, "**{}**", name)?;// Macro name
            }
            if let Some(ref description) = macro_item.description {
                writeln!(buffer, "***Description***: {}", description)?;
            }
            match macro_item.weight {
                Some(ValidMultiplier::One) => {
                    writeln!(buffer, "***Relevance***: Standard")?;
                },
                Some(ValidMultiplier::Two) => {
                    writeln!(buffer, "***Relevance***: High")?;
                },
                Some(ValidMultiplier::Three) => {
                    writeln!(buffer, "***Relevance***: Critical")?;
                },
                None => {
                    writeln!(buffer, "***Relevance***: Standard")?;
                }
            }

            writeln!(buffer)?;
            
            for question in &macro_item.questions {
                if let Some(ref question_text) = question.question {
                    writeln!(buffer,"- {}", question_text)?; // Question
                    writeln!(buffer, "    `Score: {}`", question.score.unwrap_or(ValidScore::Zero) as i16)?;
                    writeln!(buffer)?;
                    writeln!(buffer, "    Personal Observations: ")?;
                    writeln!(buffer)?;
                    writeln!(buffer)?;
                }
            }
            writeln!(buffer)?;
        }

        writeln!(buffer)?;
        writeln!(buffer)?;
        writeln!(buffer)?;

        // Footer
        writeln!(buffer, "Date: {}", chrono::Local::now())?;
        Ok(())

    }
    fn create_pdf(md_file_path: &str, pdf_file_path: &str) -> io::Result<()>{
        // Create a pdf file from the buffer
        Command::new("pandoc")
            .arg(md_file_path)
            .arg("-o")
            .arg(pdf_file_path)
            .status()
            .expect("Failed to convert markdown to PDF with Pandoc");

        output_manager::clear_screen()?;
        output_manager::type_print_wrppd("The report has been successfully created.", 2)?;
        thread::sleep(Duration::from_secs(6));

        Ok(())
    }
}



fn main() {

    let mut assesment: Vec<Macro> = vec![
        Macro::new(
            "Idea".to_uppercase().to_string(), 
            "Description: Evaluates the novelty, necessity, and economic impact of the ICO's core concept.".to_string(), 
            vec![
                Question::new("What problem is the ICO trying to solve? Is it significant and meaningful?".to_string()),
                Question::new("Does the market already exist for this product, or does it need to be created?".to_string()),
                Question::new("Is the problem being addressed something that is currently handled by a layer of unnecessary complexity or middlemen?".to_string()),
                Question::new("What unique solution does the ICO propose? Is this solution a significant improvement on existing alternatives?".to_string()),
                Question::new("Is the underlying technology behind the ICO robust and scalable?".to_string()),
            ]
        ),
        Macro::new(
            "Technology".to_uppercase().to_string(),
            "Description: Assesses the technical feasibility, innovation, and scalability of the technology used.".to_string(),
            vec![
                Question::new("Is the technological solution proposed by the ICO feasible and currently available?".to_string()),
                Question::new("Does the ICO's technology offer a substantial improvement over existing technology?".to_string()),
                Question::new("How does the ICO handle data security and privacy concerns?".to_string()),
                Question::new("Is the underlying technology behind the ICO robust and scalable?".to_string()),
                Question::new("What are the potential technical challenges the ICO might face, and how does it plan to address them?".to_string()),
            ]
        ),
        Macro::new(
            "Blockchain Specifics".to_uppercase().to_string(),
            "Description: Examines the choice of blockchain, token economics, and alignment with project needs.".to_string(),
            vec![
                Question::new("What blockchain platform is being used, and why was it chosen?".to_string()),
                Question::new("Is there a genuine need for blockchain technology in this ICO, or could the problem be solved using traditional technology?".to_string()),
                Question::new("Does the ICO introduce a new token, and what is its utility within the ecosystem?".to_string()),
                Question::new("How does the tokenomics structure impact the potential for long-term sustainability?".to_string()),
                Question::new("Are there any innovative blockchain features, such as smart contracts or decentralized applications, being utilized?".to_string()),
            ]
        ),
        Macro::new(
            "Team".to_uppercase().to_string(),
            "Description: Reviews the experience, expertise, and reliability of the team behind the ICO.".to_string(),
            vec![
                Question::new("What are the backgrounds and previous experiences of the team members?".to_string()),
                Question::new("Does the team have a proven track record in similar ventures or in the blockchain industry?".to_string()),
                Question::new("How is the team's expertise relevant to the success of the project?".to_string()),
                Question::new("Are there any advisors, and what roles do they play in the project?".to_string()),
                Question::new("How does the team plan to grow, and what strategies are in place for scaling up?".to_string()),
            ]
        ),
        Macro::new(
            "Execution".to_uppercase().to_string(),
            "Description: Considers the operational strategy, legal compliance, and financial planning of the ICO.".to_string(),
            vec![
                Question::new("What is the roadmap for the ICO, and what milestones have already been achieved?".to_string()),
                Question::new("How does the ICO plan to use the funds raised?".to_string()),
                Question::new("Are there clear marketing and outreach strategies in place?".to_string()),
                Question::new("How does the ICO plan to handle regulatory and legal issues?".to_string()),
                Question::new("What are the risks associated with the ICO, and how does it plan to mitigate them?".to_string()),
            ]
        ),
        Macro::new(
            "Market Potential".to_uppercase().to_string(),
            "Description: Analyzes the market demand, competition, and growth potential of the ICO.".to_string(),
            vec![
                Question::new("Is the idea scalable? What is a realistic share of the market?".to_string()),
                Question::new("Is there positive convexity to apply a similar idea in related applications?".to_string()),
            ]
        ),
    ];

    match output_manager::welcome() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    }
    
    match output_manager::menu() {
        Ok(MenuAction::Exit) => return,
        Ok(MenuAction::Start) => {
            // Directly updates assesment
            match questionary::display(&mut assesment) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            }
            match output_manager::ask_document() {
                None => (),
                Some(ico) => {
                    create_document::start(ico, assesment)                    
                },
            }
            match output_manager::quit_message() {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            }
            // would you like to create a a pdf report?
            // if yes, ask for ICO name, Name of creator and create a pdf(formati aggiuntivi word/md/txt) report
                // All the data will be saved in a pdf file with questions ecaluation and total score %
                // ICOs Name
                // Report
                // total score
                // date
            // if no, exit the program

            
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    }
}

