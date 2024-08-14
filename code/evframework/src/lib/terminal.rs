pub mod models;
use std::io::{self, Write, Read, Error};
use std::fs::File;
use std::thread;
use std::time::Duration;
use textwrap::{fill, termwidth};
use unicode_width::UnicodeWidthStr;
use crossterm::terminal::size;

const TYPING_SPEED: u64 = 23000; //1000 = 1ms per character

pub enum AsciiFile {
    EvFramework,
    Easteregg,
    Credits,
    MacroAreas,
    System,
    Idea,
    Technology,
    BlockchainSpecifics,
    Team,
    Execution,
    MarketPotential
}
pub enum MenuAction {
    Start,
    Exit,
}

pub mod output_manager {
    use super::*;
    use crate::input_manager::{get_input, get_user_input};

    pub fn type_print_wrppd(message: &str, delay: u64) -> io::Result<()> {

        let width = termwidth(); // Get the current terminal width
        let wrapped = fill(message, width); 

        for c in wrapped.chars() {
            io::stdout().lock().write_all(c.to_string().as_bytes())?;
            io::stdout().lock().flush()?; // Flush at the end of the loop to ensure immediate output
            thread::sleep(Duration::from_micros(delay)); // Allow customizable typing speed
        }
        println!(); // Add a newline after the message is fully typed out
        Ok(())
    }
    pub fn print_cntrd_txt(text: &str) {
        let (width, _) = size().unwrap_or((80, 0));

        for line in text.lines() {
            let text_width = UnicodeWidthStr::width(line) as u16;
            let padding = if width > text_width {
                (width - text_width) / 2
            } else {
                0
            };

        print!("{:padding$}{}\n", "", line, padding = padding as usize);
    }
    }
    pub fn print_txt(a: AsciiFile) -> io::Result<String>{
        match a {
            AsciiFile::Easteregg => {
                // open .txt
                let mut file = File::open("../../assets/ascii/easteregg.txt")?;

                // read the file content into a String
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                Ok(content)
            },
            AsciiFile::EvFramework => {
                // open .txt
                let mut file = File::open("../../assets/ascii/evframework.txt")?;

                // reads the file content into a String
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                Ok(content)
            },
            AsciiFile::Credits => {
                // open .txt
                let mut file = File::open("../../assets/ascii/credits.txt")?;

                // reads the file content into a String
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                Ok(content)
            },
            AsciiFile::MacroAreas => {
                // open .txt
                let mut file = File::open("../../assets/ascii/macroareas.txt")?;

                // reads the file content into a String
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                Ok(content)
            },
            AsciiFile::System => {
                // open .txt
                let mut file = File::open("../../assets/ascii/system.txt")?;

                // reads the file content into a String
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                Ok(content)
            },
            AsciiFile::Idea => {
                // open .txt
                let mut file = File::open("../../assets/ascii/macro/idea.txt")?;

                // reads the file content into a String
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                Ok(content)
            },
            AsciiFile::Technology => {
                // open .txt
                let mut file = File::open("../../assets/ascii/macro/technology.txt")?;

                // reads the file content into a String
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                Ok(content)
            },
            AsciiFile::BlockchainSpecifics => {
                // open .txt
                let mut file = File::open("../../assets/ascii/macro/blockchainspecs.txt")?;

                // reads the file content into a String
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                Ok(content)
            },
            AsciiFile::Team => {
                // open .txt
                let mut file = File::open("../../assets/ascii/macro/team.txt")?;

                // reads the file content into a String
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                Ok(content)
            },
            AsciiFile::Execution => {
                // open .txt
                let mut file = File::open("../../assets/ascii/macro/execution.txt")?;

                // reads the file content into a String
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                Ok(content)
            },
            AsciiFile::MarketPotential => {
                // open .txt
                let mut file = File::open("../../assets/ascii/macro/marketpotential.txt")?;

                // reads the file content into a String
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                Ok(content)
            },
        }
    }

    pub fn welcome() -> io::Result<()> {
        clear_screen()?;
        thread::sleep(Duration::from_secs(3));
        print_cntrd_txt(print_txt(AsciiFile::EvFramework)?.as_str());//Title
        type_print_wrppd("\nWelcome to the Evaluation Framework App!\n\n",TYPING_SPEED)?;
        type_print_wrppd("This framework provides a structured approach to evaluate ICOs across various dimensions,\n\
                          aiming to assist investors, analysts, and enthusiasts in making informed decisions.\n\n\n",
                          TYPING_SPEED-22600)?;
        type_print_wrppd("ONGOING COMMANDS\n\
                          ----------------\n\n\
                          [Ctrl + C] Exit",TYPING_SPEED-22600)?;
        println!("\n\n\nWhen you are ready, press enter to Start.");

        get_input()?; // Read input only after typing is done

        clear_screen()?;
        Ok(())
    }
    pub fn menu() -> io::Result<MenuAction> {
        println!("Main Menu\n");
        type_print_wrppd("[1] Scoring System Information", TYPING_SPEED)?;
        type_print_wrppd("[2] Start Evaluation", TYPING_SPEED)?;
        type_print_wrppd("[3] Credits", TYPING_SPEED)?;
        type_print_wrppd("[4] Exit", TYPING_SPEED)?;
        println!("\nPlease select an option by entering the corresponding number:");

        match get_user_input()?.as_str() {
                "1" => scoring_system_info()?,
                "2" => { 
                    clear_screen()?;
                    return Ok(MenuAction::Start);
                },
                "3" => {
                    clear_screen()?;
                    print_cntrd_txt(print_txt(AsciiFile::Credits)?.as_str());//Credits
                    type_print_wrppd(
                        "\nThis Evaluation Framework App was developed by Kenneth Boldrini as part of the blockchain-potential-carnival CheatSheet Repository.\n\
                        \n( This app is inspired by a checklist vetted by venture capitalists and improved by Dr. Harvey R. Campbell )\n",
                        TYPING_SPEED-22600)?;
                    println!("\n\n\nWhen you are ready, press enter to go back.\n");
                    get_input()?;
                    clear_screen()?;
                    menu()?;
                }
                "4" => {
                    clear_screen()?;
                    println!("Exiting EvFramework. Goodbye!");
                    thread::sleep(Duration::from_secs(6)); // Time delay before exiting 6 seconds
                    clear_screen()?;
                    return Ok(MenuAction::Exit);
                },
                _ => {
                    clear_screen()?;
                    println!("Invalid input. Please select a valid option.");
                    thread::sleep(Duration::from_secs(6)); // Time delay before exiting 6 seconds
                    clear_screen()?;
                    menu()?;
                },
        }
        Ok(MenuAction::Exit)
    }
    pub fn scoring_system_info() -> io::Result<()> {
        clear_screen()?;
        print_cntrd_txt(print_txt(AsciiFile::System)?.as_str());//System
        type_print_wrppd(
            "\n\nThe evaluation of the questionnaire is based on a series of ratings ranging from -5 to +5, where:\n\n\
            [-5] indicates that an aspect of the ICO is extremely inadequate, suggesting significant concerns or risks.\n\
            [ 0] represents a neutral stance, indicating that the aspect meets basic expectations without significant strengths or weaknesses.\n\
            [+5] signifies that an aspect is excellent, demonstrating outstanding qualities or advantages that significantly enhance the ICO's appeal.\n\n\n",
            TYPING_SPEED-22600,
        )?;
        print_cntrd_txt(print_txt(AsciiFile::MacroAreas)?.as_str());//Macro Areas
        type_print_wrppd("\n\nThe system categorizes ICO characteristics into six major areas, each containing specific elements to be evaluated:\n\n\
            1. Idea: Evaluates the novelty, necessity, and economic impact of the ICO's core concept.\n\
            2. Technology: Assesses the technical feasibility, innovation, and scalability of the technology used.\n\
            3. Blockchain Specifics: Examines the choice of blockchain, token economics, and alignment with project needs.\n\
            4. Team: Reviews the experience, expertise, and reliability of the team behind the ICO.\n\
            5. Execution: Considers the operational strategy, legal compliance, and financial planning of the ICO.\n\
            6. Market Potential: Analyzes the market demand, competition, and growth potential of the ICO.",
            TYPING_SPEED-22600,
        )?;
        type_print_wrppd(
            "\nEach macro area carries a different weight, reflecting its relative importance in the overall evaluation of an ICO. \
            Users can assign multipliers to each macro area based on their individual assessment priorities, with values ranging from 1 to 3:\n\n\
            - A multiplier of 1 suggests standard importance.\n\
            - A multiplier of 2 indicates increased importance.\n\
            - A multiplier of 3 denotes critical importance.",
            TYPING_SPEED-22600,
        )?;
        type_print_wrppd(
            "\nThese multipliers are used to adjust the impact of each macro area's score on the overall evaluation, \
            allowing for a customized and prioritized assessment that aligns with the user's strategic investment criteria.",
            TYPING_SPEED-22600,
        )?;

        type_print_wrppd("\n\nWith this app i introduce a systematic approach to assessing the viability and \
            potential of Initial Coin Offerings (ICOs) through a detailed scoring system. Designed \
            to guide users through a structured evaluation process, it helps uncover the strengths \
            and weaknesses of different ICO projects.",TYPING_SPEED)?;

        println!("\n\n\nWhen you are ready, press enter to go back.");

        get_input()?; 

        clear_screen()?;
        menu()?;
        Ok(())

    }

    pub fn clear_screen() -> io::Result<()> {
        print!("\x1B[2J\x1B[1;1H"); // Clear the terminal screen
        io::stdout().flush()?; // Ensure the screen is cleared before moving on
        Ok(())
    }

}

pub mod input_manager {

    use super::*;

    pub fn get_input() -> io::Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string()) // Trim to remove newlines
    }

    pub fn get_user_input() -> io::Result<String> {
        let mut input = String::new();

        // Stampa il prompt ">>"
        print!(">> ");
        io::stdout().flush()?;  // Flush per assicurarsi che ">>" sia visualizzato prima di leggere l'input

        // Legge l'input dall'utente
        io::stdin().read_line(&mut input)?;

        // Restituisce l'input dell'utente, rimuovendo eventuali spazi bianchi (come newline)
        Ok(input.trim().to_string())
    }

}

pub mod questionary {

    use super::*;
    use crate::input_manager::{get_user_input};
    use crate::output_manager::{clear_screen, type_print_wrppd, print_txt, print_cntrd_txt};
    use models::{ValidScore, ValidMultiplier, Macro};
    
    

    /// Display the questionnaire for each Macro area
    pub fn display(areas: &mut Vec<Macro>) -> io::Result<()>{
        for a in areas.iter_mut(){
            match a.name.as_deref() {
                Some("IDEA") => print_cntrd_txt(print_txt(AsciiFile::Idea)?.as_str()),//Idea,
                Some("TECHNOLOGY") => print_cntrd_txt(print_txt(AsciiFile::Technology)?.as_str()),//Technology,
                Some("BLOCKCHAIN SPECIFICS") => print_cntrd_txt(print_txt(AsciiFile::BlockchainSpecifics)?.as_str()),//BlockchainSpecifics,
                Some("TEAM") => print_cntrd_txt(print_txt(AsciiFile::Team)?.as_str()),//Team,
                Some("EXECUTION") => print_cntrd_txt(print_txt(AsciiFile::Execution)?.as_str()),//Execution,
                Some("MARKET POTENTIAL") => print_cntrd_txt(print_txt(AsciiFile::MarketPotential)?.as_str()),//MarketPotential,
                Some(_other) => (),
                None => (),
            }

            let header = "\nMacro Area"; 
            match a.name {
                Some(ref name) => println!("{}: {}", header, name.to_string()),
                None => println!("Unnamed"),
            }
            match a.description {
                Some(ref description) => type_print_wrppd(description, TYPING_SPEED)?,//twprint description
                None => type_print_wrppd("No description available", TYPING_SPEED)?,
            }
            type_print_wrppd("\n\nDoes this Macro have a particular relavance, this will influence the evaluation weight?\n\
                                [1] Standard\n\
                                [2] Relevant\n\
                                [3] Crucial\n", TYPING_SPEED)?;//twprint Area questionay value
            //get_user_input multiplier & control multiplier & update multiplier value 
             while let Some(s) = mult_validation(get_user_input()) {
                a.weight = Some(s);// Macro.weight
                break;
             }
            println!("\n\n*Reply to the questions with a score from -5 to 5 \
                        (Wrong inputs will set neutral score 0)*\n"); // print question
            for q in a.questions.iter_mut() {
                if let Some(ref question) = q.question.as_deref() {
                    type_print_wrppd(question, TYPING_SPEED)?;
                } 
                while let Some(s) = score_validation(get_user_input()) {
                    q.score = Some(s);
                    break;
                }
                println!();
            } 
            clear_screen()?; 
        }   
        //twprint exit message
        type_print_wrppd("Thank you for completing the evaluation. Your scores have been recorded.\n\
                            \nDigit Enter to quit || theMoor9.", TYPING_SPEED)?;
        
        //get_input
        //match get_input for easteregg
        match get_user_input()?.as_str() {
            "theMoor9" => {
                clear_screen()?;
                type_print_wrppd(print_txt(AsciiFile::Easteregg)?.as_str(), TYPING_SPEED-22600)?;//twprint easteregg
                thread::sleep(Duration::from_secs(9)); // Time delay before exiting 9 seconds
            }   
            _ => (),
        }

        Ok(())
    }

    fn score_validation(score: Result<String, Error>) -> Option<ValidScore> {
        match score {
            Ok(s) => match s.as_str() {
                "-5" => Some(ValidScore::NFive),
                "-4" => Some(ValidScore::NFour),
                "-3" => Some(ValidScore::NThree),
                "-2" => Some(ValidScore::NTwo),
                "-1" => Some(ValidScore::NOne),
                "0" => Some(ValidScore::Zero),
                "1" => Some(ValidScore::POne),
                "2" => Some(ValidScore::PTwo),
                "3" => Some(ValidScore::PThree),
                "4" => Some(ValidScore::PFour),
                "5" => Some(ValidScore::PFive),
                _ => Some(ValidScore::Zero),
            },
            Err(_) => None, // Gestione dell'errore, restituisci None se c'è un errore
        }
    }
    
    fn mult_validation(mlt: Result<String, Error>) -> Option<ValidMultiplier> {
        match mlt {
            Ok(s) => match s.as_str() {
                "1" => Some(ValidMultiplier::One),
                "2" => Some(ValidMultiplier::Two),
                "3" => Some(ValidMultiplier::Three),
                _ => Some(ValidMultiplier::One),
            },
            Err(_) => None, // Gestione dell'errore, restituisci None se c'è un errore
        }
    }
}