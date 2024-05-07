use std::collections::HashSet;


/// super fucking hacky. not prod ready lol
pub fn get_exports(input: String) -> HashSet<(String, String)> {
    let result = input
        .lines()
        .rev()
        .skip_while(|x| x != &")")
        .skip(1)
        .map_while(|x| {
            let x = x.trim();
            // (export "x")
            let export: String = x.chars().take(7).collect();
            if &export != "(export" {
                return None;
            }
            let name = x.chars().skip(9).take_while(|x| x != &'"').collect::<String>();
            let tp = x
                .chars()
                .skip(9)
                .skip_while(|x| x != &'(')
                .skip(1)
                .take_while(|x| !x.is_whitespace())
                .collect::<String>();
            Some((name, tp))
        }).collect();


    result
}
