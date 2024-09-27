use std::io::{self, Write}; // Für die Benutzereingabe und Ausgabe

struct Frage {
    question: String,
    options: Vec<String>,
    correct_Antwort: usize,
}

impl Frage {
    fn ask(&self) -> bool {
        println!("{}", self.question);

        for (i, option) in self.options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        print!("Deine Antwort (Nummer): ");
        io::stdout().flush().unwrap(); // Ausgabe sofort anzeigen

        let mut Antwort = String::new();
        io::stdin().read_line(&mut Antwort).unwrap();
        let Antwort: usize = match Antwort.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ungültige Eingabe.");
                return false;
            }
        };

        if Antwort == self.correct_Antwort {
            println!("Richtig!\n");
            true
        } else {
            println!(
                "Falsch. Die richtige Antwort war: {}\n",
                self.correct_Antwort
            );
            false
        }
    }
}

fn main() {
    let quiz = vec![
        Frage {
            question: String::from("Welches der folgenden ist ein gültiger Funktionsaufruf in Rust?"),
            options: vec![
                String::from("printline!(\"Hello, World!\");"),
                String::from("println!(\"Hello, World!\");"),
                String::from("printf(\"Hello, World!\");"),
                String::from("print!(\"Hello, World!\");"),
            ],
            correct_Antwort: 2, // Die Antwort ist die zweite Option
        },
        Frage {
            question: String::from("Was passiert, wenn du eine Variable, die den Besitz eines Wertes hat, an eine andere Funktion übergibst?"),
            options: vec![
                String::from("Der ursprüngliche Besitzer kann den Wert weiterhin verwenden."),
                String::from("Der ursprüngliche Besitzer verliert den Zugriff auf den Wert."),
                String::from("Der Wert wird dupliziert."),
                String::from("Rust gibt eine Fehlermeldung aus."),
            ],
            correct_Antwort: 2, // Die richtige Antwort ist "Der ursprüngliche Besitzer verliert den Zugriff auf den Wert."
        },
        Frage {
            question: String::from("Welche der folgenden Funktionen wird verwendet, um Fehler in Rust sicher zu handhaben?"),
            options: vec![
                String::from("Option<T>"),
                String::from("Result<T, E>"),
                String::from("panic!()"),
                String::from("unwrap()"),
            ],
            correct_Antwort: 2, // Die richtige Antwort ist "Result<T, E>"
        },
    ];

    let mut score = 0;

    println!("Willkommen beim Rust-Quiz! Beantworte die Fragen, indem du die Nummer der richtigen Antwort eingibst.\n");

    for question in &quiz {
        // Iteriere über eine Referenz auf `quiz`, um es nicht zu verbrauchen
        if question.ask() {
            score += 1;
        }
    }

    println!(
        "Du hast {}/{} richtige Antworten gegeben!",
        score,
        quiz.len()
    ); // `quiz` kann jetzt noch verwendet werden
}
