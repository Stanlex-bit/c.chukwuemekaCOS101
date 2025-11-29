use std::io::Write;

struct Student {
    name: String,
    matric: String,
    department: String,
    level: String,
}

fn main() {
    // Vector containing all student details
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: "300".to_string(),
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: "100".to_string(),
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric: "CSC10328828".to_string(),
            department: "Computer".to_string(),
            level: "200".to_string(),
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric: "EEE11020202".to_string(),
            department: "Electrical".to_string(),
            level: "200".to_string(),
        },
        Student {
            name: "Blanca Edemoh".to_string(),
            matric: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: "100".to_string(),
        },
    ];

    // Display all student details
    println!("PAU SMIS:\n");
    for s in &students {
        println!(
            "Name: {}\nMatric no: {}\nDepartment: {}\nLevel: {}\n",
            s.name, s.matric, s.department, s.level
        );
    }

    // Save details to a file
    let mut file = std::fs::File::create("PAU SMIS.txt").expect("create failed");

    writeln!(file, "PAU SMIS\n").unwrap();
    writeln!(file, "Student Name | Matric Number | Department | Level").unwrap();
    writeln!(file, "------------------------------------------------------------------").unwrap();

    for s in &students {
        writeln!(
            file,
            "{} | {} | {} | {}",
            s.name, s.matric, s.department, s.level
        )
        .unwrap();
    }

    println!("\nData has been successfully saved to PAU SMIS.txt");
}

