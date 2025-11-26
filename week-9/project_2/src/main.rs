use std::fs::File;
use std::io::Write;

fn main() {
    let names = [
        "Oluchi Mordi",
        "Adams Halimat",
        "Shania Bolade",
        "Adekule Gold",
        "Blanca Edemon",
    ];

    let matric = [
        "ACC10211111",
        "ECO1002001",
        "CSC10328828",
        "EEE11020202",
        "MEE10020001",
    ];

    let departments = [
        "Accounting",
        "Economics",
        "Computer",
        "Electrical",
        "Mechanical",
    ];

    let levels = ["300", "100", "200", "200", "100"];

    let mut file = File::create("students.txt").expect("Could not create file");

    let header = "PAU SMIS - Student Details\n\n";
    file.write_all(header.as_bytes()).unwrap();

    for i in 0..names.len() {
        let line = format!(
            "Name: {}\nMatric Number: {}\nDepartment: {}\nLevel: {}\n\n",
            names[i], matric[i], departments[i], levels[i]
        );

        println!("{}", line);

        file.write_all(line.as_bytes()).unwrap();
    }

    println!("Student details saved successfully!");
}