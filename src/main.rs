use rand::seq::SliceRandom;
use rand::rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;
use std::io::{self, Write}; // <-- Importing Write trait

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Card {
    name: String,
    category: String,
    energy_cost: u32,
    stability: u32,
    effect: String,
    bonus: String,
}

#[derive(Debug)]
struct Player {
    name: String,
    hand: Vec<Card>,
    field: Vec<Card>,
    deck: Vec<Card>,
    energy: u32,
}

impl Player {
    fn new(name: &str, deck: Vec<Card>) -> Self {
        Self {
            name: name.to_string(),
            hand: Vec::new(),
            field: Vec::new(),
            deck,
            energy: 3,
        }
    }

    fn draw_card(&mut self) {
        if let Some(card) = self.deck.pop() {
            self.hand.push(card);
        } else {
            println!("{} has no more cards to draw.", self.name);
        }
    }

    fn play_card(&mut self, card_name: &str) -> String {
        if let Some(index) = self.hand.iter().position(|c| c.name.to_lowercase() == card_name.to_lowercase()) {
            let card = self.hand.remove(index);
            return if self.energy >= card.energy_cost {
                self.energy -= card.energy_cost;
                self.field.push(card.clone());
                format!("{} plays {}!", self.name, card.name)
            } else {
                self.hand.push(card);
                format!("Not enough energy to play {}!", card_name)
            }
        }
        format!("Card {} not found in hand!", card_name)
    }

    fn generate_energy(&mut self, turn: u32) {
        if turn < 5 {
            self.energy += 1;
        } else if turn == 5 {
            self.energy += 5;
        } else {
            self.energy += 5;
        }
    }

    fn total_stability(&self) -> u32 {
        self.field.iter().map(|c| c.stability).sum()
    }
    fn display_game_status(&self, opponent: &Player, turn: u32) {
        let name_width = self.name.len().max(opponent.name.len()).max(10) + 2; // Minimum width of 10
        let col_width = name_width.max(34); // Ensure it doesn't shrink below 34
        let line_width = col_width * 2 + 5; // Adjust total width dynamically

        println!("\n{}", "=".repeat(line_width));
        println!(
            " Chemistry Trading Card Game (CTCG)  |  Turn: {}  |  Max Rounds: 15",
            turn
        );
        println!("{}", "=".repeat(line_width));

        println!("\n=== GAME STATUS ===");
        println!(
            "+{:-<width$}+{:-<width$}+",
            "", "", width = col_width
        );
        println!(
            "| {:<width$} | {:<width$} |",
            self.name, opponent.name, width = col_width
        );
        println!(
            "|{:-<width$}|{:-<width$}|",
            "", "", width = col_width
        );
        println!(
            "| Energy: {:<width$} | Energy: {:<width$} |",
            self.energy, opponent.energy, width = col_width - 9
        );
        println!(
            "| Compounds in Field: {:<width$} | Compounds in Field: {:<width$} |",
            self.field.len(), opponent.field.len(), width = col_width - 20
        );
        println!(
            "| Total Stability: {:<width$} | Total Stability: {:<width$} |",
            self.total_stability(), opponent.total_stability(), width = col_width - 17
        );
        println!(
            "| Deck Remaining: {:<width$} | Deck Remaining: {:<width$} |",
            self.deck.len(), opponent.deck.len(), width = col_width - 15
        );
        println!(
            "+{:-<width$}+{:-<width$}+\n",
            "", "", width = col_width
        );
    }

    fn display_card_stats(&self, card_name: &str) {
        if let Some(card) = self.hand.iter().find(|c| c.name.to_lowercase() == card_name.to_lowercase()) {
            println!("\n+-------------------------------------------------+");
            println!("| {} |", card.name);
            println!("+-------------------------------------------------+");
            println!("| Category: {:<35} |", card.category);
            println!("| Energy Cost: {:<32} |", card.energy_cost);
            println!("| Stability: {:<34} |", card.stability);
            println!("| Effect: {:<36} |", card.effect);
            println!("| Bonus: {:<37} |", card.bonus);
            println!("+-------------------------------------------------+\n");
        } else {
            println!("Card '{}' not found in hand!", card_name);
        }
    }

    fn display_hand_table(&self) {
        let mut categorized_cards: HashMap<String, Vec<&Card>> = HashMap::new();

        let category_map = vec![
            ("Alkali Metals", "Elemental"), ("Alkaline Earth Metals", "Elemental"),
            ("Transition Metals", "Elemental"), ("Post-Transition Metals", "Elemental"),
            ("Metalloids", "Elemental"), ("Nonmetals", "Elemental"),
            ("Halogens", "Elemental"), ("Noble Gases", "Elemental"),
            ("Ionic Compound", "Compound"), ("Strong Acid", "Compound"),
            ("Organic Compound", "Compound"), ("Mineral Compound", "Compound"),
            ("Basic Compound", "Compound"), ("Metal Oxide", "Compound"),
            ("Elemental Crystal", "Compound"), ("Metal Salt", "Compound"),
            ("Peroxide", "Compound"), ("Salt", "Compound"),
            ("Network Solid", "Compound"), ("Acid", "Compound"),
            ("Biological Complex", "Compound"), ("Synthetic Polymer", "Compound"),
            ("Carbon Allotrope", "Compound"), ("Coordination Complex", "Compound"),
            ("Ceramic Compound", "Compound"),
            ("Basic Reaction", "Reaction"), ("Intermediate Reaction", "Reaction"),
            ("Advanced Reaction", "Reaction"), ("Environmental Reaction", "Reaction"),
            ("Specialized Reaction", "Reaction"), ("Expert Reaction", "Reaction"),
            ("Inorganic Catalysts", "Catalyst"), ("Biochemical Catalysts", "Catalyst"),
            ("Industrial Catalysts", "Catalyst"), ("Environmental Catalysts", "Catalyst"),
            ("Specialty Catalysts", "Catalyst"),
            ("Scientist Card", "Scientist"), ("State Card", "State"),
        ].into_iter().collect::<HashMap<_, _>>();

        for card in &self.hand {
            let normalized_category = category_map
                .get(card.category.as_str())
                .unwrap_or(&"Other")
                .to_string();
            categorized_cards
                .entry(normalized_category)
                .or_insert_with(Vec::new)
                .push(card);
        }

        let categories = ["Elemental", "Compound", "Reaction", "Catalyst", "Scientist", "State"];
        let max_width = 120;
        let col_width = max_width / categories.len();

        println!("\n{}'s Hand (Sorted by Type):", self.name);
        println!("+{}+", "-".repeat(max_width));
        print!("|");
        for category in categories.iter() {
            print!(" {:^width$} |", category, width = col_width - 2);
        }
        println!("\n+{}+", "-".repeat(max_width));

        let max_rows = categorized_cards.values().map(|v| v.len()).max().unwrap_or(0);

        for i in 0..max_rows {
            print!("|");
            for category in categories.iter() {
                if let Some(card_list) = categorized_cards.get(*category) {
                    if i < card_list.len() {
                        let card = card_list[i];
                        let card_info = format!("{} (Cost: {})", card.name, card.energy_cost);
                        let truncated_info = if card_info.len() > col_width - 2 {
                            format!("{}...", &card_info[..(col_width - 5)])
                        } else {
                            card_info
                        };
                        print!(" {:<width$} |", truncated_info, width = col_width - 2);
                    } else {
                        print!(" {:<width$} |", "", width = col_width - 2);
                    }
                } else {
                    print!(" {:<width$} |", "", width = col_width - 2);
                }
            }
            println!();
        }
        println!("+{}+", "-".repeat(max_width));
    }
    fn display_victory_status(&self, opponent: &Player) {
        // Aquí determinas las condiciones para vencer:
        // 1. Si el oponente no tiene más Compounds => Victoria inmediata
        // 2. Si tu oponente se queda sin cartas en el deck => Victoria
        // etc.

        // Muestra cuántas cartas de 'Compound' quedan en el campo o mano del oponente
        let opponent_compounds = opponent.field.iter()
            .filter(|c| c.category.to_lowercase().contains("compound"))
            .count();

        // Imprime un resumen
        println!("\n[INFORMACIÓN DE VICTORIA]");
        println!("Cartas 'Compound' del oponente en juego: {}", opponent_compounds);
        println!(
            "Cartas restantes en el mazo del oponente: {}",
            opponent.deck.len()
        );
        println!(
            "Energía actual tuya ({}): {}",
            self.name, self.energy
        );
        println!(
            "Energía actual de {}: {}",
            opponent.name, opponent.energy
        );

        // Aquí puedes añadir más detalles si quieres
        // (puntos de vida, estabilidad total, etc.)
        // Por ejemplo, mostrar la estabilidad total de ambos jugadores:
        println!("Estabilidad total de tus compuestos: {}", self.total_stability());
        println!("Estabilidad total de los compuestos del oponente: {}", opponent.total_stability());

        // Si tuvieras un campo de 'vida' o 'puntos de salud' en Player, podrías mostrarlo así:
        // println!("Tu vida ({}): {}", self.name, self.life_points);
        // println!("Vida del oponente ({}): {}", opponent.name, opponent.life_points);
    }
}


fn load_deck() -> Vec<Card> {
    let json_files = vec![
        "data/elements.json", "data/compounds.json", "data/catalyst.json",
        "data/reactions.json", "data/scientist.json", "data/states.json"
    ];

    let mut all_cards = Vec::new();
    for file in json_files {
        match fs::read_to_string(file) {
            Ok(data) => {
                match serde_json::from_str::<Vec<Card>>(&data) {
                    Ok(mut cards) => all_cards.append(&mut cards),
                    Err(e) => println!("Error parsing JSON file '{}': {}", file, e),
                }
            }
            Err(e) => println!("Warning: Could not read file '{}': {}", file, e),
        }
    }

    // Mezcla el mazo
    all_cards.shuffle(&mut rng());

    // **Limitar a 60 cartas**
    if all_cards.len() > 60 {
        all_cards.truncate(60);
    }

    all_cards
}


fn game_loop() {
    let mut deck = load_deck();
    let mut player1 = Player::new("Alice", deck.split_off(deck.len() / 2));
    let mut player2 = Player::new("Bob", deck);

    // Both players draw 5 cards at the start
    for _ in 0..5 {
        player1.draw_card();
        player2.draw_card();
    }

    let mut turn = 1;
    while turn <= 15 {
        player1.display_game_status(&player2, turn);
        player1.display_hand_table();
        player1.display_victory_status(&player2);

        println!("(A) Play a Card  |  (P) Pass  |  (Q) Quit");
        println!("--------------------------------------------------------------------------------");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let action = input.trim().to_uppercase();

        match action.as_str() {
            "A" => {
                // Show selectable cards
                if player1.hand.is_empty() {
                    println!("Your hand is empty! No cards to play.");
                    continue;
                }

                println!("\n=== Select a Card to Play ===");
                for (index, card) in player1.hand.iter().enumerate() {
                    println!("[{}] {} (Energy: {}, Stability: {})", index + 1, card.name, card.energy_cost, card.stability);
                }
                print!("Enter the number of the card to play: ");
                io::stdout().flush().unwrap();

                let mut card_choice = String::new();
                io::stdin().read_line(&mut card_choice).expect("Failed to read line");

                if let Ok(choice) = card_choice.trim().parse::<usize>() {
                    if choice > 0 && choice <= player1.hand.len() {
                        let card_name = player1.hand[choice - 1].name.clone();
                        println!("{}", player1.play_card(&card_name));
                    } else {
                        println!("Invalid choice! Please enter a valid number.");
                    }
                } else {
                    println!("Invalid input! Please enter a number.");
                }
            }
            "P" => {
                println!("You pass your turn.");
                player1.generate_energy(turn);
            }
            "Q" => {
                println!("Game Over: Player Quit.");
                return;
            }
            _ => println!("Invalid choice, please try again."),
        }

        // Player generates energy after their turn
        player1.generate_energy(turn);

        // -------------------------
        // CPU TURN (Bob)
        // -------------------------

        // CPU draws a card
        player2.draw_card();

        println!("\nCPU Turn:");
        if let Some(index) = player2.hand.iter().position(|c| c.energy_cost <= player2.energy) {
            let card_name = player2.hand[index].name.clone();
            println!("CPU plays {}.", card_name);
            player2.play_card(&card_name);
        } else {
            println!("CPU passes.");
            player2.generate_energy(turn);
        }

        turn += 1;
    }

    println!("Game Over: Maximum turns reached.");
}


fn main() {
    game_loop();
}