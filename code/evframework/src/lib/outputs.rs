use printpdf::*;
use webbrowser;
use crate::models::*;
use std::{
    path::Path,
    io::{self, Write, BufWriter},
    fs::{File,remove_file},
    thread,
    time::Duration
};
use textwrap::fill;
use crate::terminal::output_manager;

pub mod create_document {
    use super::*;

    pub fn start(ico_name: String, score: i16, isv: u8, updated_assessment: Vec<Macro>){

        let new_ico_assessment = IcoEvaluation::new(
            ico_name, 
            score,
            isv,
            updated_assessment
        );

        let md_file_path =  Path::new("../../output/md/buffer.md");
        let pdf_file_path =  Path::new("../../output/pdf/EvFrameworkReport.pdf");

        match create_md(&new_ico_assessment, md_file_path.to_str().unwrap()){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };

        match create_pdf(new_ico_assessment, pdf_file_path.to_str().unwrap()){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };

    }

    fn create_md(ico: &IcoEvaluation, file_path: &str) -> io::Result<()> {
        // Create a buffer to write the data
        remove_file(file_path)?;
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
    fn create_pdf(ico: IcoEvaluation, file_path: &str) -> io::Result<()> {
            // Create a new PDF document
            let (doc, page1, layer1) = PdfDocument::new(&ico.name, Mm(210.0), Mm(297.0), "Layer 1");
            let mut current_layer = doc.get_page(page1).get_layer(layer1);
            // Start position for the content
            let mut y_position = 287.0;
            // Page width
            let page_width = 100.0;

        
            // Load a font
            let font = doc.add_external_font(File::open("../../assets/fonts/Helvetica.ttf").map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            let title_font = doc.add_external_font(File::open("../../assets/fonts/helvetica-compressed-5871d14b6903a.otf").map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            let oblique_font = doc.add_external_font(File::open("../../assets/fonts/Helvetica-Oblique.ttf").map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            let bold_font = doc.add_external_font(File::open("../../assets/fonts/Helvetica-Bold.ttf").map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        
            
            let draw_text_wrapped = |current_layer: &PdfLayerReference, text: &str, font_size: f32, x_pos: f32, y_pos: &mut f32, font: &IndirectFontRef| -> Result<(), printpdf::Error> {
                let wrapped_text = fill(text, page_width as usize); // 190 As the full sixe
                for line in wrapped_text.lines() {
                    current_layer.use_text(line, font_size, Mm(x_pos), Mm(*y_pos), &font);
                    *y_pos -= 8.0;
                }
                Ok(())
            };

            let check_page_break = |doc: &PdfDocumentReference, current_layer: &mut PdfLayerReference, y_position: &mut f32| -> Result<(), printpdf::Error> {
                if *y_position < 20.0 {
                    let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                    *current_layer = doc.get_page(new_page).get_layer(new_layer);
                    *y_position = 287.0; // Reset y_position to the top of the new page
                }
                Ok(())
            };



            // Centered Title: EvFramework Report
            let _ = draw_text_wrapped(&current_layer, "EvFramework Report", 36.0, 55.0, &mut y_position, &title_font);
            y_position -= 12.0;

            // Title: ICO Name
            let _ = draw_text_wrapped(&current_layer, &ico.name, 24.0, 10.0, &mut y_position, &bold_font);
        
            // Investment Suitability Value and Total Score
            let _ = draw_text_wrapped(&current_layer, format!("Investment Suitability Value: {}%", ico.investment_suitability_value).as_str(), 12.0, 10.0, &mut y_position, &bold_font);
            let _ = draw_text_wrapped(&current_layer, format!("Total Score: {}", ico.total_score).as_str(), 12.0, 10.0, &mut y_position, &oblique_font);
            y_position -= 22.0;
        
            // Add each Macro Area
            for macro_item in &ico.macros {

                let _ = check_page_break(&doc, &mut current_layer, &mut y_position);

                if let Some(ref name) = macro_item.name {
                    let _ = draw_text_wrapped(&current_layer, name, 18.0, 10.0, &mut y_position, &bold_font);
                }
                let _ = check_page_break(&doc, &mut current_layer, &mut y_position);
        
                if let Some(ref description) = macro_item.description {
                    let _ = draw_text_wrapped(&current_layer, format!("Description: {}", description).as_str(), 12.0, 10.0, &mut y_position, &oblique_font);
                }
                let _ = check_page_break(&doc, &mut current_layer, &mut y_position);
        
                if let Some(weight) = macro_item.weight {
                    let weight_str = match weight {
                        ValidMultiplier::One => "Relevance: Standard",
                        ValidMultiplier::Two => "Relevance: High",
                        ValidMultiplier::Three => "Relevance: Critical",
                    };
                let _ = draw_text_wrapped(&current_layer, weight_str, 12.0, 10.0, &mut y_position, &oblique_font);
                y_position -= 12.0;
                let _ = check_page_break(&doc, &mut current_layer, &mut y_position);
                }
        
                // Add each question
                for question in &macro_item.questions {

                    let _ = check_page_break(&doc, &mut current_layer, &mut y_position);

                    if let Some(ref question_text) = question.question {
                        let _ = draw_text_wrapped(&current_layer, format!("Question: {}", question_text).as_str(), 12.0, 10.0, &mut y_position, &font);
                        y_position -= 2.0;
                    }
                    let _ = check_page_break(&doc, &mut current_layer, &mut y_position);
                    
                    let _ = draw_text_wrapped(&current_layer, format!("Score: {}", question.score.unwrap_or(ValidScore::Zero) as i16).as_str(), 12.0, 10.0, &mut y_position, &font);
                    y_position -= 2.0;
                    let _ = check_page_break(&doc, &mut current_layer, &mut y_position);

                    // Placeholder for Personal Observations
                    let _ = draw_text_wrapped(&current_layer, "Personal Observations:", 12.0, 10.0, &mut y_position, &font);
                    y_position -= 12.0; // leave space for observations
                    let _ = check_page_break(&doc, &mut current_layer, &mut y_position);
                }
        
                y_position -= 12.0;
                let _ = check_page_break(&doc, &mut current_layer, &mut y_position);
            }
        
            // Footer with date
            let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
            let _ = draw_text_wrapped(&current_layer, format!("Date: {}", current_date).as_str(), 12.0, 10.0, &mut y_position, &font);
        
            // Save the PDF document
            remove_file(file_path)?;
            let mut file = BufWriter::new(File::create(file_path)?);
            doc.save(&mut file).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
            

            output_manager::clear_screen()?;
            println!("\n\n");
            output_manager::print_cntrd_txt("The report has been successfully created.");
            thread::sleep(Duration::from_secs(6));
            // Open the PDF file in the default web browser
            if let Err(e) = webbrowser::open(file_path) {
                eprintln!("Failed to open PDF in the browser: {}", e);
            };
            Ok(())
        }
        
}