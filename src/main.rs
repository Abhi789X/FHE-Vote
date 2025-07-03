use anyhow::Result;
use serde_json::{from_str, to_string};
use std::fs;
use std::io;
use tfhe::integer::{ClientKey, ServerKey};
use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2_COMPACT_PK;

struct VotingSystem {
    client_key: ClientKey,
    server_key: ServerKey,
    encrypted_votes: Vec<String>,
}

impl VotingSystem {
    fn new() -> Self {
        let client_key = ClientKey::new(PARAM_MESSAGE_2_CARRY_2_COMPACT_PK);
        let server_key = ServerKey::new(&client_key);
        VotingSystem {
            client_key,
            server_key,
            encrypted_votes: Vec::new(),
        }
    }

    fn encrypt_vote(&self, vote: u64) -> String {
        let encrypted = self.client_key.encrypt(vote);
        to_string(&encrypted).unwrap()
    }

    fn decrypt_result(&self, encrypted_total: &str) -> u64 {
        let encrypted_val: tfhe::integer::Ciphertext = from_str(encrypted_total).unwrap();
        self.client_key.decrypt(&encrypted_val)
    }

    fn add_votes(&self) -> Result<String> {
        let mut total = self.client_key.encrypt(0u64);
        
        for vote_str in &self.encrypted_votes {
            let vote: tfhe::integer::Ciphertext = from_str(vote_str)?;
            total = self.server_key.unchecked_add(&total, &vote);
        }
        
        Ok(to_string(&total)?)
    }

    fn save_votes(&self) -> Result<()> {
        fs::write("votes.json", to_string(&self.encrypted_votes)?)?;
        Ok(())
    }

    fn load_votes(&mut self) -> Result<()> {
        if let Ok(data) = fs::read_to_string("votes.json") {
            self.encrypted_votes = from_str(&data)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut system = VotingSystem::new();
    system.load_votes()?;

    loop {
        println!("\nDAO Vote Verifier");
        println!("1. Submit Vote (0=No, 1=Yes)");
        println!("2. Tally Votes");
        println!("3. Add Demo Votes");
        println!("4. Exit");
        println!("Select option: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        
        match choice.trim() {
            "1" => {
                println!("Enter vote (0 or 1): ");
                let mut vote = String::new();
                io::stdin().read_line(&mut vote)?;
                
                match vote.trim().parse::<u64>() {
                    Ok(0) | Ok(1) => {
                        let encrypted = system.encrypt_vote(vote.trim().parse()?);
                        system.encrypted_votes.push(encrypted);
                        system.save_votes()?;
                        println!("Vote encrypted and stored!");
                    }
                    _ => println!("Invalid vote! Must be 0 or 1."),
                }
            }
            "2" => {
                if system.encrypted_votes.is_empty() {
                    println!("No votes to tally!");
                    continue;
                }
                
                let encrypted_total = system.add_votes()?;
                let total_yes = system.decrypt_result(&encrypted_total);
                let total_no = system.encrypted_votes.len() as u64 - total_yes;
                
                println!("\nVOTE RESULTS");
                println!("------------");
                println!("Total votes: {}", system.encrypted_votes.len());
                println!("Yes votes: {}", total_yes);
                println!("No votes: {}", total_no);
                println!("------------");
            }
            "3" => {
                // Add demo votes: 3 Yes, 2 No
                for _ in 0..3 {
                    system.encrypted_votes.push(system.encrypt_vote(1));
                }
                for _ in 0..2 {
                    system.encrypted_votes.push(system.encrypt_vote(0));
                }
                system.save_votes()?;
                println!("Added 5 demo votes (3 Yes, 2 No)");
            }
            "4" => break,
            _ => println!("Invalid option!"),
        }
    }
    
    Ok(())
          }
