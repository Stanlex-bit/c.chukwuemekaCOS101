fn get_aps_level(role: &str, title: &str) -> Option<&'static str> {

    // Data directly as vectors of tuples
    let data = vec![
        ("Office Administrator", "APS 1-2", vec!["Intern"]),
        ("Office Administrator", "APS 3-5", vec!["Administrator"]),
        ("Office Administrator", "APS 5-8", vec!["Senior Administrator"]),
        ("Office Administrator", "EL1 8-10", vec!["Office Manager"]),
        ("Office Administrator", "EL2 10-13", vec!["Director"]),
        ("Office Administrator", "SES", vec!["CEO"]),

        ("Academic", "APS 3-5", vec!["Research Assistant"]),
        ("Academic", "APS 5-8", vec!["PhD Candidate"]),
        ("Academic", "EL1 8-10", vec!["Post-Doc Researcher"]),
        ("Academic", "EL2 10-13", vec!["Senior Lecturer"]),
        ("Academic", "SES", vec!["Dean"]),

        ("Lawyer", "APS 1-2", vec!["Paralegal"]),
        ("Lawyer", "APS 3-5", vec!["Junior Associate"]),
        ("Lawyer", "APS 5-8", vec!["Associate"]),
        ("Lawyer", "EL1 8-10", vec!["Senior Associate 1-2"]),
        ("Lawyer", "EL2 10-13", vec!["Senior Associate 3-4"]),
        ("Lawyer", "SES", vec!["Partner"]),

        ("Teacher", "APS 1-2", vec!["Placement"]),
        ("Teacher", "APS 3-5", vec!["Classroom Teacher"]),
        ("Teacher", "APS 5-8", vec!["Snr Teacher"]),
        ("Teacher", "EL1 8-10", vec!["Leading Teacher"]),
        ("Teacher", "EL2 10-13", vec!["Deputy Principal"]),
        ("Teacher", "SES", vec!["Principal"]),
    ];

    // Simple loop to find match
    for (r, range, titles) in data {
        if r == role && titles.contains(&title) {
            return Some(range);
        }
    }

    None
}

fn main() {
    let role = "Lawyer";
    let title = "Associate";

    match get_aps_level(role, title) {
        Some(level) => println!("{} - {} → {}", role, title, level),
        None => println!("No APS level found."),
    }
}

