use std::collections::HashMap;

#[derive(Default)]
struct TeamStats {
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, TeamStats> = HashMap::new();

    for line in match_results.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 3 {
            continue;
        }
        let team1 = parts[0].to_string();
        let team2 = parts[1].to_string();
        let result = parts[2];

        teams.entry(team1.clone()).or_default();
        teams.entry(team2.clone()).or_default();

        match result {
            "win" => {
                let t1 = teams.get_mut(&team1).unwrap();
                t1.w += 1;
                t1.mp += 1;
                t1.p += 3;

                let t2 = teams.get_mut(&team2).unwrap();
                t2.l += 1;
                t2.mp += 1;
            }
            "loss" => {
                let t1 = teams.get_mut(&team1).unwrap();
                t1.l += 1;
                t1.mp += 1;

                let t2 = teams.get_mut(&team2).unwrap();
                t2.w += 1;
                t2.mp += 1;
                t2.p += 3;
            }
            "draw" => {
                let t1 = teams.get_mut(&team1).unwrap();
                t1.d += 1;
                t1.mp += 1;
                t1.p += 1;

                let t2 = teams.get_mut(&team2).unwrap();
                t2.d += 1;
                t2.mp += 1;
                t2.p += 1;
            }
            _ => {}
        }
    }

    let mut teams_vec: Vec<_> = teams.into_iter().collect();

    // Sort: points descending, then name ascending
    teams_vec.sort_by(|a, b| {
        b.1.p.cmp(&a.1.p).then(a.0.cmp(&b.0))
    });

    // Build the output string
    let mut output = String::new();
    output.push_str("Team                           | MP |  W |  D |  L |  P\n");
    for (name, stats) in teams_vec {
        output.push_str(&format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}\n",
            name, stats.mp, stats.w, stats.d, stats.l, stats.p
        ));
    }

    output.trim_end().to_string()
}