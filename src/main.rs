// Dies Ist Ein Test ob die Gitfunktion in der Rust IDE funktioniert.

use std::io::{self, Write}; // Für die Benutzereingabe und Ausgabe

// Definition der Struktur `Frage`
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
            println!("Falsch. Die richtige Antwort war: {}\n", self.correct_Antwort);
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
            correct_Antwort: 2,
        },
        Frage {
            question: String::from("Was passiert, wenn du eine Variable, die den Besitz eines Wertes hat, an eine andere Funktion übergibst?"),
            options: vec![
                String::from("Der ursprüngliche Besitzer kann den Wert weiterhin verwenden."),
                String::from("Der ursprüngliche Besitzer verliert den Zugriff auf den Wert."),
                String::from("Der Wert wird dupliziert."),
                String::from("Rust gibt eine Fehlermeldung aus."),
            ],
            correct_Antwort: 2,
        },
        Frage {
            question: String::from("Welche der folgenden Funktionen wird verwendet, um Fehler in Rust sicher zu handhaben?"),
            options: vec![
                String::from("Option<T>"),
                String::from("Result<T, E>"),
                String::from("panic!()"),
                String::from("unwrap()"),
            ],
            correct_Antwort: 2,
        },
        // 20 weitere Fragen:
        Frage {
            question: String::from("Was ist der Zweck des 'Ownership'-Konzepts in Rust?"),
            options: vec![
                String::from("Vermeidung von Garbage Collection."),
                String::from("Speicher und Ressourcen sicher verwalten."),
                String::from("Den Compiler beschleunigen."),
                String::from("Die Performance von Code zu reduzieren."),
            ],
            correct_Antwort: 2,
        },
        Frage {
            question: String::from("Was bedeutet 'mutable borrowing' in Rust?"),
            options: vec![
                String::from("Ein exklusives Recht, eine Variable zu ändern."),
                String::from("Das Duplizieren einer Variablen."),
                String::from("Das Umwandeln von Werten in Referenzen."),
                String::from("Das Anlegen eines konstanten Wertes."),
            ],
            correct_Antwort: 1,
        },
        Frage {
            question: String::from("Welche der folgenden Typen bietet Rust für konkurrierende Programmierung?"),
            options: vec![
                String::from("Mutex<T>"),
                String::from("Option<T>"),
                String::from("Result<T, E>"),
                String::from("Vec<T>"),
            ],
            correct_Antwort: 1,
        },
        Frage {
            question: String::from("Welche der folgenden Funktionen wird verwendet, um einen Faden in Rust zu erzeugen?"),
            options: vec![
                String::from("thread::launch"),
                String::from("thread::spawn"),
                String::from("std::create_thread"),
                String::from("thread::new"),
            ],
            correct_Antwort: 2,
        },
        Frage {
            question: String::from("Wie nennt man das Feature, das es erlaubt, generischen Code in Rust zu schreiben?"),
            options: vec![
                String::from("Traits"),
                String::from("Generics"),
                String::from("Enums"),
                String::from("Modules"),
            ],
            correct_Antwort: 2,
        },
        Frage {
            question: String::from("Wie kannst du sicherstellen, dass eine Referenz in Rust nicht null ist?"),
            options: vec![
                String::from("Durch Verwendung des Option<T>-Typs."),
                String::from("Durch Verwendung des unsafe-Schlüsselworts."),
                String::from("Durch Deklaration als mutable."),
                String::from("Es gibt keine Möglichkeit."),
            ],
            correct_Antwort: 1,
        },
        Frage {
            question: String::from("Welches der folgenden Statements ist korrekt, wenn eine Variable 'moved' wird?"),
            options: vec![
                String::from("Der Wert wird dupliziert."),
                String::from("Die alte Variable kann nicht mehr verwendet werden."),
                String::from("Die alte Variable enthält eine Kopie des Werts."),
                String::from("Es entsteht ein Kompilierungsfehler."),
            ],
            correct_Antwort: 2,
        },
        Frage {
            question: String::from("Wie kannst du verhindern, dass der Rust-Compiler 'Unused variable'-Warnungen ausgibt?"),
            options: vec![
                String::from("Benutze das Schlüsselwort 'unused' vor der Variablen."),
                String::from("Setze einen Unterstrich (_) vor den Variablennamen."),
                String::from("Verwende die Option '-ignore-unused'."),
                String::from("Rust erlaubt keine ungenutzten Variablen."),
            ],
            correct_Antwort: 2,
        },
        Frage {
            question: String::from("Was bewirkt das Schlüsselwort 'mod' in Rust?"),
            options: vec![
                String::from("Es definiert ein neues Modul."),
                String::from("Es definiert einen Trait."),
                String::from("Es erzeugt eine neue Variable."),
                String::from("Es weist Speicher zu."),
            ],
            correct_Antwort: 1,
        },
        Frage {
            question: String::from("Welches der folgenden Schlüsselwörter wird verwendet, um bedingten Code in Rust auszuführen?"),
            options: vec![
                String::from("switch"),
                String::from("if"),
                String::from("match"),
                String::from("choose"),
            ],
            correct_Antwort: 2,
        },
        Frage {
            question: String::from("Welcher der folgenden Typen erlaubt dir, einen Wert zu 'borrowen' (auszuleihen)?"),
            options: vec![
                String::from("&T"),
                String::from("T"),
                String::from("String"),
                String::from("usize"),
            ],
            correct_Antwort: 1,
        },
        Frage {
            question: String::from("Welcher der folgenden Datentypen ist 'unsized' in Rust?"),
            options: vec![
                String::from("str"),
                String::from("usize"),
                String::from("i32"),
                String::from("f64"),
            ],
            correct_Antwort: 1,
        },
        Frage {
            question: String::from("Was macht das Schlüsselwort 'unsafe' in Rust?"),
            options: vec![
                String::from("Es erlaubt Operationen, die Rusts Sicherheitseinschränkungen umgehen."),
                String::from("Es erzwingt die Garbage Collection."),
                String::from("Es deklariert eine unsichere Variable."),
                String::from("Es zeigt eine Kompilierungswarnung an."),
            ],
            correct_Antwort: 1,
        },
        Frage {
            question: String::from("Welche Methode kann verwendet werden, um eine Zeichenkette (String) in einen Ganzzahlwert umzuwandeln?"),
            options: vec![
                String::from("parse::<T>()"),
                String::from("to_string()"),
                String::from("unwrap()"),
                String::from("clone()"),
            ],
            correct_Antwort: 1,
        },
        Frage {
            question: String::from("Welche Funktion erlaubt es, benutzerdefinierte Logik in Fehlerfällen bereitzustellen, anstatt eine panische Ausführung zu verursachen?"),
            options: vec![
                String::from("expect()"),
                String::from("unwrap()"),
                String::from("Result<T, E>::map_err()"),
                String::from("Option<T>::ok_or()"),
            ],
            correct_Antwort: 3,
        },
        Frage {
            question: String::from("Was ist der Zweck des ownership-based Typsystems in Rust?"),
            options: vec![
                String::from("Es vermeidet Speicherfehler durch explizites Management."),
                String::from("Es zwingt zur Garbage Collection."),
                String::from("Es erlaubt es, den Speicher manuell zu verwalten."),
                String::from("Es deklariert variable Größen."),
            ],
            correct_Antwort: 1,
        },
        Frage {
            question: String::from("Wie kannst du in Rust mehrere Werte aus einer Funktion zurückgeben?"),
            options: vec![
                String::from("Indem du einen Tupel-Typ verwendest."),
                String::from("Indem du mehrere return-Anweisungen nutzt."),
                String::from("Indem du mehrere Variablen deklarierst."),
                String::from("Es ist nicht möglich, mehrere Werte zurückzugeben."),
            ],
            correct_Antwort: 1,
        },
    ];

    let mut score = 0;

    println!("Willkommen beim erweiterten Rust-Quiz! Beantworte die Fragen, indem du die Nummer der richtigen Antwort eingibst.\n");

    for question in &quiz {
        if question.ask() {
            score += 1;
        }
    }

    println!(
        "Du hast {}/{} richtige Antworten gegeben!",
        score,
        quiz.len()
    );
}
