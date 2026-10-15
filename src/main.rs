use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    // 1. Отримуємо шлях до файлу:
    // або з аргументу командного рядка (наприклад: cargo run -- my_file.txt),
    // або за замовчуванням беремо "text.txt", або запитуємо у користувача.
    let file_path = get_file_path();

    println!("Спроба відкрити файл: \"{}\"...", file_path);

    // 2. Зчитуємо вміст файлу
    let content = match fs::read_to_string(&file_path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("\n[ПОМИЛКА] Не вдалося відкрити або прочитати файл \"{}\": {}", file_path, err);
            eprintln!("Переконайтеся, що файл існує у відповідній директорії.");
            return;
        }
    };

    // 3. Розбиваємо текст на слова:
    // приводимо до нижнього регістру, відкидаємо пунктуацію
    let words: Vec<String> = content
        .split(|c: char| !c.is_alphanumeric() && c != '\'' && c != '-')
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .collect();

    println!("Успішно зчитано слів: {}", words.len());

    // Перевірка умови щодо обсягу (близько або понад 1000 слів)
    if words.len() < 1000 {
        println!(
            "[УВАГА] За умовою очікується текст приблизно на 1000 слів, але знайдено лише {}.",
            words.len()
        );
    }

    if words.len() < 3 {
        eprintln!("[ПОМИЛКА] У тексті менше ніж 3 слова — неможливо утворити триграми.");
        return;
    }

    // 4. Підрахунок послідовностей з 3 слів (триграм)
    let mut trigram_counts: HashMap<String, usize> = HashMap::new();

    // Метод .windows(3) робить ковзне вікно по 3 слова: [w1, w2, w3], [w2, w3, w4]...
    for window in words.windows(3) {
        let trigram = format!("{} {} {}", window[0], window[1], window[2]);
        *trigram_counts.entry(trigram).or_insert(0) += 1;
    }

    // 5. Сортуємо результати за спаданням частоти
    let mut sorted_trigrams: Vec<(String, usize)> = trigram_counts.into_iter().collect();
    sorted_trigrams.sort_by(|a, b| b.1.cmp(&a.1));

    // 6. Виводимо топ-5
    println!("\n================ ТОП-5 ПОСЛІДОВНОСТЕЙ З 3 СЛІВ ================");
    println!("{:<4} | {:<45} | {:<10}", "№", "Послідовність (триграма)", "Кількість");
    println!("{:-<65}", "");

    for (i, (trigram, count)) in sorted_trigrams.iter().take(5).enumerate() {
        println!("{:<4} | {:<45} | {:<10}", i + 1, trigram, count);
    }
    println!("{:-<65}", "");
}

/// Допоміжна функція для визначення шляху до файлу:
/// перевіряє передані аргументи CLI, якщо їх немає — перевіряє "text.txt",
/// якщо і його немає — запитує шлях з консолі.
fn get_file_path() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return args[1].clone();
    }

    let default_name = "text.txt";
    if Path::new(default_name).exists() {
        return default_name.to_string();
    }

    print!("Файл за замовчуванням \"text.txt\" не знайдено. Введіть назву/шлях до файлу: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}