// mod experience;
mod contact;
mod skills;

use inquire::Select;
use std::fs;
use colored::Colorize;
use contact::show_contact;
use skills::show_skills;
// use experience::show_experience;

fn main() {
    println!("");
    println!("");
    println!("Hey there! I'm {}, a full stack developer and currently learning new technologies.","Anubhav Chakraborty".bold().bright_yellow());

    let options = vec!["About","Skills","Contact","Exit"];

    loop{
        let choice = Select::new("What would you like to know?", options.clone()).prompt();

        match choice {
            Ok(choice) => {
                if choice == options[0] {
                    println!("");
                    println!("I am a proficient and driven {} with extensive experience in emerging technologies.","full-stack developer".bold().bright_yellow());
                    println!("I possess diverse technical skills, including proficiency in programming languages such as {}.","C, C++, Python, JavaScript, Java, Rust, and more".bold().bright_yellow());
                    println!("With practical knowledge in {}, I have the capacity to tackle complex projects in the tech industry.","web development, devops, and machine learning.".bold().bright_yellow());
                    println!("");
                    println!("I have honed my technical skills through multiple {}, and {} and am excited to continue building my skills and contributing to {} in the tech industry.","personal projects".bold().bright_yellow(),"online courses".bold().bright_yellow(),"innovative projects".bold().bright_yellow());
                    println!("Along with my technical expertise, I possess strong soft skills such as {}. I am a team player and enjoy collaborating with others to achieve common goals.","communication, critical thinking, and problem-solving".bold().bright_yellow());
                    println!("");
                }
                // else if choice == options[1] {
                //     let file_path = "./data/experience/experience.json".to_owned();
                //     let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
                //     let res = show_experience(&contents);
                //     match res {
                //         Ok(_res) => println!(""),
                //         Err(_) => println!("Error in experience.rs"),
                //     }
                // }
                else if choice == options[1] {
                    let file_path = "./data/skills/skills.json".to_owned();
                    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
                    let res = show_skills(&contents);
                    match res {
                        Ok(_res) => println!(""),
                        Err(_) => println!("Error in skills.rs"),
                    }
                }
                else if choice == options[2] {
                    show_contact();
                }
                else if choice == options[3] {
                    println!("Bye! Have a great day!");
                    break;
                }
            },
            Err(_) => println!("You did not select a valid option"),
        }
    }
}
