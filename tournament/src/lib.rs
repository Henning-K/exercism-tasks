use std::collections::HashMap;

pub fn tally(inp: &str) -> String {
    // (wins, draws, losses, points)
    let mut team_stats: HashMap<&str, (u32, u32, u32, u32)> = HashMap::new();
    for line in inp.lines() {
        let mut iter = line.split(";");
        let (team1, team2, status) = (iter.next(), iter.next(), iter.next());
        if team1.is_none() || team2.is_none() || status.is_none() {
            continue;
        }
        let (team1, team2) = (team1.unwrap(), team2.unwrap());
        match status {
            Some("win") => {
                {
                    let temp1 = team_stats.entry(team1).or_insert((0, 0, 0, 0));
                    temp1.0 += 1;
                    temp1.3 += 3;
                }
                {
                    let temp2 = team_stats.entry(team2).or_insert((0, 0, 0, 0));
                    temp2.2 += 1;
                }
            }
            Some("draw") => {
                {
                    let temp1 = team_stats.entry(team1).or_insert((0, 0, 0, 0));
                    temp1.1 += 1;
                    temp1.3 += 1;
                }
                {
                    let temp2 = team_stats.entry(team2).or_insert((0, 0, 0, 0));
                    temp2.1 += 1;
                    temp2.3 += 1;
                }
            }
            Some("loss") => {
                {
                    let temp1 = team_stats.entry(team1).or_insert((0, 0, 0, 0));
                    temp1.2 += 1;
                }
                {
                    let temp2 = team_stats.entry(team2).or_insert((0, 0, 0, 0));
                    temp2.0 += 1;
                    temp2.3 += 3;
                }
            }
            _ => {
                // Ill-formed line, ignoring.
            }
        }
    }
    let mut table_: Vec<(String, u32, u32, u32, u32, u32)> = Vec::new();
    for (key, val) in &team_stats {
        table_.push(((*key).to_string(), val.0 + val.1 + val.2, val.0, val.1, val.2, val.3));
    }
    let mut table = table_.as_mut_slice();
    table.sort_by(|a, b| a.0.cmp(&b.0));
    table.sort_by(|a, b| b.5.cmp(&a.5));
    let mut result = String::new();
    result = format!("{}Team                           | MP |  W |  D |  L |  P",
                     result);
    for line in table {
        result = format!("{}\n{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
                         result,
                         line.0,
                         line.1,
                         line.2,
                         line.3,
                         line.4,
                         line.5);
    }
    result
}