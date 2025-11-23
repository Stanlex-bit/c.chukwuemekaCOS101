fn main() {
    // List of developers (Name, Years of Experience)
    let developers = vec![
        ("Chimaka", 3),
        ("Korede", 7),
        ("David", 5),
        ("Stanley", 10),
        ("Tenski", 6),
    ];

    // Track the max
    let mut highest = developers[0];

    for dev in developers {
        if dev.1 > highest.1 {
            highest = dev;
        }
    }

    println!(
        "Developer with highest experience:\nName: {}\nExperience: {} years",
        highest.0, highest.1
    );
}

