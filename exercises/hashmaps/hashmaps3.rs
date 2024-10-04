// hashmaps3.rs
//
//给出了足球比赛的分数列表（每行一个）。每一行都是
//表单：“<team_1_name>，<team_2_name>
//示例：英格兰队，法国队，4,2（英格兰队进4球，法国队进2球）。
//你必须建立一个分数表，其中包含球队的名字、目标和
//该队得分，失球。一种建立分数的方法
//表使用Hashmap。该解决方案部分是为了使用
//Hashmap，完成它以通过测试。
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.


use std::collections::HashMap;

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        let t1 = scores.entry(team_1_name).or_insert(Team{goals_scored: 0,goals_conceded: 0});
        t1.goals_scored += team_1_score;
        t1.goals_conceded += team_2_score;
        let t2 = scores.entry(team_2_name).or_insert(Team{goals_scored: 0,goals_conceded: 0});
        t2.goals_scored += team_2_score;
        t2.goals_conceded += team_1_score;
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
