use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufReader, BufRead };

fn main() {
    let file = File::open("/home/gab/sourceCode/fec-dataset/data/itcont.txt")
    // let file = File::open("/home/gab/sourceCode/fec-dataset/data/by_date/itcont_2018_20070823_20170529.txt")
        .expect("Failed to read file!");
    let mut reader = BufReader::new(file);

    let mut line_count = 0;
    let mut full_name_array: Vec<String> = Vec::new();
    let mut first_name_map: HashMap<String, u32> = HashMap::new();
    let mut date_map: HashMap<u32, u32> = HashMap::new();
    
    let mut buffer = String::new();
    while reader.read_line(&mut buffer).expect("Failed to read file line!") > 0 {
        {
            let line = buffer.trim_right();

            line_count += 1;

            let mut start = 0;
            let mut current_col = 1;
            for (i, c) in line.chars().enumerate() {
                if c == '|' {
                    current_col += 1;
                    
                    if current_col == 5 || current_col == 8 {
                        start = i + 1;
                    }
                    
                    if current_col == 6 {
                        let month = line.to_string()[(start + 4)..(start + 6)].parse().unwrap();
                        date_map.entry(month)
                            .and_modify(|e| { *e += 1 })
                            .or_insert(1);
                    }
                    if current_col == 9 {
                        if line_count == 433 || line_count == 43244 {
                            full_name_array.push(line[start..i].to_string());
                        }
                        let first_name = process_full_name(&line[start..i].to_string());
                        first_name_map.entry(first_name)
                            .and_modify(|e| { *e += 1 })
                            .or_insert(1);
                    }
                }
            }
        }

        buffer.clear();
    }

    println!("Number of lines: {}", line_count);
    println!("432nd name: {}, 43243rd name: {}", full_name_array[0], full_name_array[1]);
    print_date_map(&date_map);
    let max_first_name = most_common_name(&first_name_map);
    println!("Most common first name is {} with {} instances of it", max_first_name.0, max_first_name.1);
}

fn most_common_name(first_name: &HashMap<String, u32>) -> (&str, u32) {
    let mut max = ("", 0);
    for (key, val) in first_name.iter() {
        if val > &max.1 {
            max.0 = key;
            max.1 = *val;
        }
    }

    max
}

fn print_date_map(date_map: &HashMap<u32, u32>) {
    println!("Donations per month:");

    let mut total = 0;
    for (key, val) in date_map.iter() {
        println!("=> {}: {}", convert_month_num(key), val);
        total += val;
    }

    println!("For a total of {} donations!", total);
}

fn convert_month_num(num: &u32) -> &'static str {
    match num {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Invalid"
    }
}

fn process_full_name(name: &String) -> String {
    // println!("{}", name);
    let new_name = match name.split(',').nth(1) {
        Some(x) => x.to_string(),
        None => name.clone()
    };

    let new_name = new_name.trim();
    // println!("=> {:?}", new_name.split(' ').collect::<Vec<&str>>());

    let res_arr = new_name.split(' ').collect::<Vec<&str>>();
    if res_arr[0].ends_with('.') && res_arr.len() > 1 {
        // println!("=> {}", res_arr[1]);
        res_arr[1].to_string()
    } else {
        // println!("=> {}", res_arr[0]);
        res_arr[0].to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    
    #[test]
    fn test_first_name_processing() {
        let pre = ["PEREZ, JOHN A","DEEHAN, WILLIAM N","WATJEN, THOMAS R.","SABOURIN, JAMES","MAKER, SCOTT T.",
                   "PYNE, CHRISTOPHER W","JEROME, CHRISTOPHER","FOLEY, JOSEPH","MCGARRY, JOHN","SIMONDS, MICHAEL Q",
                   "BOUTIN, DONALD","GORDON, ANDREA","MCKENNEY, RICHARD","MCGUINNESS, MARTIN","ROYAL, JONATHAN",
                   "ZABEL, STEVEN","IGLESIAS, LISA","WHITE, THOMAS A.H.","ARNOLD, TIMOTHY G","JOSEPH, STEVEN",
                   "BARONE, MICHAEL","ABBOTT, JOSEPH","BAHL, TRACY","BAKER, SCOTT","BISACCIA, LISA","BORATTO, EVA",
                   "CHRISTAL, NANCY","DE NALE, CAROL","DENTON, DAVID","FOULKES, HELENA","ERWIN, GARY","FALKOWSKI, DAVID",
                   "FLUM, JOSHUA","FRENDO, JOSEPH","GILSON, THOMAS","GOLD, STEPHEN","GRIFFIN, MARK","JOYNER, JOHN",
                   "KENNEDY, JOHN","KNUDSON, JEFFREY","KRAFT, ROCKY","LEONARD, MATTHEW","LOTVIN, ALAN","MCINTOSH, COLLEEN",
                   "MERLO, LARRY","FRENDO, JOSEPH","PENBERTHY, SHANNON","MORIARTY, THOMAS","MURPHY, KEVIN"];
        
        let post = ["JOHN","WILLIAM","THOMAS","JAMES","SCOTT","CHRISTOPHER","CHRISTOPHER","JOSEPH","JOHN","MICHAEL",
                    "DONALD","ANDREA","RICHARD","MARTIN","JONATHAN","STEVEN","LISA","THOMAS","TIMOTHY","STEVEN",
                    "MICHAEL","JOSEPH","TRACY","SCOTT","LISA","EVA","NANCY","CAROL","DAVID","HELENA","GARY","DAVID",
                    "JOSHUA","JOSEPH","THOMAS","STEPHEN","MARK","JOHN","JOHN","JEFFREY","ROCKY","MATTHEW","ALAN","COLLEEN",
                    "LARRY","JOSEPH","SHANNON","THOMAS","KEVIN"];

        for (i, name) in pre.iter().enumerate() {
            let actual = process_full_name(&name.to_string());
            let expected = post[i].to_string();
            println!("Actual: {}, Expected: {}", actual, expected);
            assert_eq!(actual, expected);
        }
    }
}
