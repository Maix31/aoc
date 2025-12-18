use std::collections::HashSet;

#[derive(Clone)]
enum Status {
    Off,
    On,
}

impl Status {
    fn toggle(self) -> Self {
        match self {
            Status::Off => Status::On,
            Status::On => Status::Off,
        }
    }
}

struct Machine {
    lights: Vec<Status>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

fn apply_button(mut lights: Vec<Status>, buttons: &Vec<usize>) -> Vec<Status> {
    for i in buttons {
        lights[*i] = lights[*i].clone().toggle();
    }
    lights
}

fn apply_button_to_joltage(mut joltage: Vec<i64>, buttons: &Vec<usize>) -> Vec<i64> {
    for &i in buttons {
        joltage[i] += 1;
    }
    joltage
}

fn is_solved(lights: &Vec<Status>) -> bool {
    lights.iter().all(|light| matches!(light, Status::Off))
}

impl Machine {}

struct Machines {
    machines: Vec<Machine>,
}

fn parse_input(input: &str) -> Machines {
    let machines = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert!(
                parts.len() >= 3,
                "expected: [lights] (buttons...) {{joltage}}"
            );

            let lights_str = parts[0];
            let joltage_str = parts[parts.len() - 1];
            let buttons_str = &parts[1..parts.len() - 1];

            let lights = lights_str
                .strip_prefix('[')
                .and_then(|s| s.strip_suffix(']'))
                .expect("lights must be in [brackets]")
                .chars()
                .map(|c| match c {
                    '.' => Status::Off,
                    '#' => Status::On,
                    _ => panic!("unexpected light char: {c}"),
                })
                .collect::<Vec<_>>();

            let buttons = buttons_str
                .iter()
                .map(|tok| {
                    tok.strip_prefix('(')
                        .and_then(|s| s.strip_suffix(')'))
                        .expect("button must be in (parentheses)")
                        .split(',')
                        .filter(|s| !s.is_empty())
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<_>>();

            let joltage = joltage_str
                .strip_prefix('{')
                .and_then(|s| s.strip_suffix('}'))
                .expect("joltage must be in {braces}")
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            Machine {
                lights,
                buttons,
                joltage,
            }
        })
        .collect();

    Machines { machines }
}

pub fn part_1(input: &str) -> u64 {
    let machines = parse_input(input);

    machines
        .machines
        .into_iter()
        .map(|m| {
            let mut a = vec![m.lights];
            let mut b = vec![];

            let mut moves = 1;

            loop {
                for old_light in a.iter() {
                    for buttons in m.buttons.iter() {
                        let new_lights = apply_button(old_light.clone(), &buttons);
                        if is_solved(&new_lights) {
                            return moves;
                        }
                        b.push(new_lights);
                    }
                }

                moves += 1;

                std::mem::swap(&mut a, &mut b);
                b.clear();
            }
        })
        .sum()
}

// pub fn part_2(input: &str) -> u64 {
//     let machines = parse_input(input);
//     let mut n = 0;
//     machines
//         .machines
//         .into_iter()
//         .map(|m| {
//             n += 1;
//             println!("{n}");
//             let mut a = vec![vec![0;m.joltage.len()]];
//             let mut b = vec![];
//
//             let mut s = HashSet::new();
//
//             let mut moves = 1;
//
//             loop {
//                 for old_joltage in a.iter() {
//                     for buttons in m.buttons.iter() {
//                         let new_joltage = apply_button_to_joltage(old_joltage.clone(), &buttons);
//                         if new_joltage == m.joltage {
//                             return moves;
//                         }
//
//                         if new_joltage.iter().zip(m.joltage.iter()).any(|(a,b)| a > b) {
//                             continue;
//                         }
//
//                         if (&mut s).insert(new_joltage.clone()) {
//                             b.push(new_joltage);
//                         }
//                     }
//                 }
//
//                 moves += 1;
//
//                 std::mem::swap(&mut a, &mut b);
//                 b.clear();
//             }
//         })
//         .sum()
// }

pub fn part_2(input: &str) -> u64 {
    let machines = parse_input(input);
    // (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

    // a * (0, 0, 0, 1) + // 3
    // b * (0, 1, 0, 1) + // 1, 3
    // c * (0, 0, 1, 0) + // 2
    // d * (0, 0, 1, 1) + // 2, 3
    // e * (1, 0, 1, 0) + // 0, 2
    // f * (1, 1, 0, 0)   // 0, 1
    // = (3, 5, 4, 7)

    a * (0, 0, 0, 1) +
    b * (0, 1, 0, 1) +
    c * (0, 0, 1, 0) +
    d * (0, 0, 1, 1) +
    e * (1, 0, 1, 0) +
    f * (1, 1, 0, 0)
    = (3, 5, 4, 7)

0 0 0 0 1 1 | 3
0 1 0 0 0 1 | 5
0 0 1 1 1 0 | 4
1 1 0 1 0 0 | 7

1 1 0 1 0 0 | 7
0 1 0 0 0 1 | 5
0 0 1 1 1 0 | 4
0 0 0 0 1 1 | 3



















        min (a + b + c + d + e + f)
    //
    // a,b,c,d,e,f part of Natural numbers
    //
    //     //      x
    //     // a * (0, 0, 0, 1) + // 3
    //     // b * (0, 1, 0, 1) + // 1, 3
    //     // c * (0, 0, 1, 0) + // 2
    //     // d * (0, 0, 1, 1) + // 2, 3
    //     // e * (1, 0, 1, 0) + // 0, 2
    //     // f * (1, 1, 0, 0)   // 0, 1
    //     // = (0, 1, 1, 0)

}

#[cfg(test)]
mod tests;
