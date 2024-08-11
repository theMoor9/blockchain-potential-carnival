pub mod MonitorManager {
    fn type_print(s: &str) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        for c in s.chars() {
            handle.write_all(c.to_string().as_bytes()).unwrap();
            handle.flush().unwrap(); // Ensure each character is output immediately
            thread::sleep(Duration::from_millis(50)); // Adjust delay to control typing speed
        }
    }
    pub fn print_wrapped_and_type(message: &str) {
        let width = termwidth(); // Get the current terminal width
        let wrapped = fill(message, width); // Wrap the message to fit the terminal width
    
        type_print(&wrapped); // Print wrapped message with typing effect
        println!(); // Move to a new line after message is completely printed
    }

    fn get_user_input() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        Return input;
    }

    fn welcome() {
        println!("Welcome to the Evaluation Framework App!");
        print_wrapped_and_type("This framework introduces a systematic approach "+
                                "to assessing the viability and potential of Initial Coin Offerings (ICOs) "+
                                "through a detailed scoring system."+
                                "Designed to guide users through a structured evaluation process, " +
                                "it helps uncover the strengths and weaknesses of different ICO projects.");
        println!("\nWhen you are ready, Press enter to start to progress.");
        print!("\x1B[2J\x1B[1;1H"); // Clear the terminal screen
        io::stdout().flush().unwrap(); // Ensure the screen is cleared before moving on
    }
    fn menu() {
        println!("Main Menu\n");
        print_wrapped_and_type("1. Scoring System Information");
        print_wrapped_and_type("2. Start Evaluation");
        print_wrapped_and_type("3. Exit");
        println!("\nPlease select an option by entering the corresponding number:");
    }
    fn scoring_system_info() {
        println!("Scoring System");
        print_wrapped_and_type("The evaluation is based on a series of ratings ranging from -5 to +5, where:");
        print_wrapped_and_type("\n[-5] indicates that an aspect of the ICO is extremely inadequate," +
                                " suggesting significant concerns or risks.");
        print_wrapped_and_type("\n[ 0] represents a neutral stance, indicating that the aspect meets basic "+
                                "expectations without significant strengths or weaknesses.");
        print_wrapped_and_type("\n[+5] signifies that an aspect is excellent, demonstrating outstanding qualities "+
                                "or advantages that significantly enhance the ICO's appeal."); 
        println!("\n\nMacro Areas of Evaluation Weghting System");  
        print_wrapped_and_type("The system categorizes ICO characteristics into six major areas,"+
                                " each containing specific elements to be evaluated:");
        print_wrapped_and_type("\n1. Idea: Evaluates the novelty, necessity, and economic impact of the ICO's core concept.");
        print_wrapped_and_type("\n2. Technology: Assesses the technical feasibility, innovation,"+
                                " and scalability of the technology used.")
        print_wrapped_and_type("\n3. Blockchain Specifics: Examines the choice of blockchain,"+
                                " token economics, and alignment with project needs.")
        print_wrapped_and_type("\n4. Team: Reviews the experience, expertise, and reliability of the team behind the ICO.");
        print_wrapped_and_type("\n5. Execution: Considers the operational strategy, legal compliance, "+
                                "and financial planning of the ICO.");
        print_wrapped_and_type("\n6. Market Potential: Analyzes the market demand, competition, and growth potential of the ICO.");
        print_wrapped_and_type("\n\nEach macro area carries a different weight, reflecting its relative importance "+
                                "in the overall evaluation of an ICO. Users can assign multipliers to each macro area "+
                                "based on their individual assessment priorities, with values ranging from 1 to 3:");
        print_wrapped_and_type("\n- A multiplier of 1 suggests standard importance.");
        print_wrapped_and_type("\n- A multiplier of 2 indicates increased importance.");
        print_wrapped_and_type("\n- A multiplier of 3 denotes critical importance.");
        print_wrapped_and_type("\n\nThese multipliers are used to adjust the impact of each macro"+
                                " area's score on the overall evaluation,"+
                                " allowing for a customized and prioritized assessment that aligns with"+
                                " the user's strategic investment criteria.");
        print!("\x1B[2J\x1B[1;1H"); // Clear the terminal screen
        io::stdout().flush().unwrap(); // Ensure the screen is cleared before moving o
    }
}

pub mod Evaluation {
    /*
    
    */
}