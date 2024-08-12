use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;
use textwrap::{fill, termwidth};
use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    execute,
    terminal::{Clear, ClearType},
};

pub mod output_manager {
    use super::*;
    use crate::input_manager::{get_user_input, get_choice_input};

    const typing_speed: u64 = 23; //ms per character

    fn type_wrapped_print(message: &str, delay: u64) -> io::Result<()> {
        let width = termwidth(); // Get the current terminal width
        let wrapped = fill(message, width); 

        for c in wrapped.chars() {
            io::stdout().lock().write_all(c.to_string().as_bytes())?;
            io::stdout().lock().flush()?; // Flush at the end of the loop to ensure immediate output
            thread::sleep(Duration::from_millis(delay)); // Allow customizable typing speed
        }
        println!(); // Add a newline after the message is fully typed out
        Ok(())
    }

    pub fn welcome() -> io::Result<()> {
        clear_screen()?;
        println!("EvFramework\n");
        type_wrapped_print(
            "Welcome to the Evaluation Framework App!\n\
            Through the app i introduce a systematic approach to assessing the viability and \
            potential of Initial Coin Offerings (ICOs) through a detailed scoring system. Designed \
            to guide users through a structured evaluation process, it helps uncover the strengths \
            and weaknesses of different ICO projects.",
            typing_speed,
        )?;
        println!("\nWhen you are ready, press enter to Start.");

        get_user_input()?; // Read input only after typing is done

        clear_screen()?;
        Ok(())
    }

    pub fn menu() -> io::Result<()> {
        println!("Main Menu\n");
        type_wrapped_print("1. Scoring System Information", typing_speed)?;
        type_wrapped_print("2. Start Evaluation", typing_speed)?;
        type_wrapped_print("3. Credits", typing_speed)?;
        type_wrapped_print("4. Exit", typing_speed)?;
        println!("\nPlease select an option by entering the corresponding number:");

        match get_choice_input("")?.as_str() {
                "1" => scoring_system_info()?,
                "2" => { 
                    Ok(())
                },
                "3" => {
                    clear_screen()?;
                    println!("Credits\n");
                    type_wrapped_print(
                        "This Evaluation Framework App was developed by Kenneth Boldrini as part of the blockchain-potential-carnival CheatSheet Repository.\n\
                        ( This app is inspired by a checklist vetted by venture capitalists and improved by Dr. Harvey R. Campbell )\n",
                        typing_speed
                    )?;
                    println!("\nWhen you are ready, press enter to progress.\n");
                    get_user_input()?;
                    clear_screen()?;
                    menu();
                }
                "4" => {
                    clear_screen()?;
                    println!("Exiting the Evaluation Framework App. Goodbye!");
                    thread::sleep(Duration::from_secs(6)); // Time delay before exiting 6 seconds
                    clear_screen()?;
                    return Ok(());
                },
                _ => {
                    clear_screen()?;
                    println!("Invalid input. Please select a valid option.");
                    thread::sleep(Duration::from_secs(6)); // Time delay before exiting 6 seconds
                    clear_screen()?;
                    menu()?;
                },
        }

        Ok(())
    }

    pub fn scoring_system_info() -> io::Result<()> {
        clear_screen()?;
        println!("SCORING SYSTEM\n");
        type_wrapped_print(
            "The evaluation is based on a series of ratings ranging from -5 to +5, where:\n\n\
            [-5] indicates that an aspect of the ICO is extremely inadequate, suggesting significant concerns or risks.\n\
            [ 0] represents a neutral stance, indicating that the aspect meets basic expectations without significant strengths or weaknesses.\n\
            [+5] signifies that an aspect is excellent, demonstrating outstanding qualities or advantages that significantly enhance the ICO's appeal.",
            typing_speed,
        )?;
        type_wrapped_print(
            "\n\nMACRO AREAS\n\n\
            The system categorizes ICO characteristics into six major areas, each containing specific elements to be evaluated:\n\n\
            1. Idea: Evaluates the novelty, necessity, and economic impact of the ICO's core concept.\n\
            2. Technology: Assesses the technical feasibility, innovation, and scalability of the technology used.\n\
            3. Blockchain Specifics: Examines the choice of blockchain, token economics, and alignment with project needs.\n\
            4. Team: Reviews the experience, expertise, and reliability of the team behind the ICO.\n\
            5. Execution: Considers the operational strategy, legal compliance, and financial planning of the ICO.\n\
            6. Market Potential: Analyzes the market demand, competition, and growth potential of the ICO.",
            typing_speed,
        )?;
        type_wrapped_print(
            "\n\nEach macro area carries a different weight, reflecting its relative importance in the overall evaluation of an ICO. \
            Users can assign multipliers to each macro area based on their individual assessment priorities, with values ranging from 1 to 3:\n\n\
            - A multiplier of 1 suggests standard importance.\n\
            - A multiplier of 2 indicates increased importance.\n\
            - A multiplier of 3 denotes critical importance.",
            typing_speed,
        )?;
        type_wrapped_print(
            "\n\nThese multipliers are used to adjust the impact of each macro area's score on the overall evaluation, \
            allowing for a customized and prioritized assessment that aligns with the user's strategic investment criteria.",
            typing_speed,
        )?;

        println!("\nWhen you are ready, press enter to progress.");

        get_user_input()?; 

        clear_screen()?;
        Ok(menu()?)

    }

    fn clear_screen() -> io::Result<()> {
        print!("\x1B[2J\x1B[1;1H"); // Clear the terminal screen
        io::stdout().flush()?; // Ensure the screen is cleared before moving on
        Ok(())
    }
}

mod input_manager {
    use super::*;

    pub fn get_user_input() -> io::Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string()) // Trim to remove newlines
    }

    pub fn get_choice_input(prompt: &str) -> io::Result<String> {
        let mut input = String::new();
        let mut stdout = io::stdout();
    
        // Salva la posizione del cursore
        execute!(stdout, SavePosition)?;
    
        // Stampa il prompt iniziale
        print!("{}>>", prompt);
        stdout.flush()?;
    
        // Leggi l'input dell'utente e aggiorna l'area "[ ]"
        loop {
            let mut buffer = [0; 1]; // Leggi un byte alla volta
            io::stdin().read_exact(&mut buffer)?; // Usa read_exact per leggere esattamente 1 byte
            let c = buffer[0] as char;
    
            match c {
                '\n' => break, // L'utente ha premuto Invio
                '\x08' | '\x7f' => { // Gestione del backspace
                    if !input.is_empty() {
                        input.pop();
                    }
                }
                _ => {
                    input.push(c); // Aggiungi il carattere all'input
                }
            }
    
            // Ripristina la posizione del cursore, cancella l'area di input e riscrivi l'input
            execute!(
                stdout,
                RestorePosition,
                Clear(ClearType::FromCursorDown), // Pulisci dall'area corrente del cursore fino in fondo
                MoveTo(prompt.len() as u16 + 1, 0) // Sposta il cursore subito dopo il prompt
            )?;
            print!(">>{}", input.trim()); // Assicurati di non aggiungere spazi indesiderati
            stdout.flush()?;
        }
    
        // Vai a capo dopo l'input
        println!();
        Ok(input.trim().to_string()) // Rimuovi spazi indesiderati prima di restituire l'input
    }
}

pub mod questionary {

    use super::*;

    // fn display(area: Macro) -> Vec<Macro> {
    //     let header = "Macro Areas and Questions:"; 
    //     for subheader in area::Question::question {
    //         println!("{}\n{}",header,subheader); 
    //         for question in questions {
    //             type_wrapped_print(question);
    //         }  
    //     }   
    // }
    
}