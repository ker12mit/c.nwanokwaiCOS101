fn main() {
    let names = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieve",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut merged: Vec<(&str, &str, &str)> = Vec::new();

    for i in 0..names.len() {
        merged.push((names[i], ministries[i], zones[i]));
    }

    println!(
        "{: <4} {: <30} {: <18} {: <15}",
        "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEO. ZONE"
    );
    println!("{}", "-".repeat(75));

    for (index, (name, ministry, zone)) in merged.iter().enumerate() {
        println!(
            "{: <4} {: <30} {: <18} {: <15}",
            index + 1,
            name,
            ministry,
            zone
        );
    }
}