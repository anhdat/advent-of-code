fn print_map(map: &Vec<Vec<char>>) {
    for y in 0..(map.len()) {
        for x in 0..(map[0].len()) {
            print!("{}", map[y][x]);
        }
        print!("\n");
    }
}

fn get_adjacent_indices(x: usize, y: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let adjacent_indices: Vec<(usize, usize)> = directions
        .iter()
        .map(|d| (x as isize + d.0, y as isize + d.1))
        .filter(|(xx, yy)| !(*xx < 0 || *xx >= m as isize || *yy < 0 || *yy >= n as isize))
        .map(|(xx, yy)| (xx as usize, yy as usize))
        .collect();

    adjacent_indices
}

fn next_state(x: usize, y: usize, map: &Vec<Vec<char>>) -> char {
    let current_state = map[y][x];
    let neighbors: Vec<(usize, usize)> = get_adjacent_indices(x, y, map[0].len(), map.len());
    match current_state {
        'L' => {
            if neighbors.into_iter().any(|(x, y)| map[y][x] == '#') {
                current_state
            } else {
                '#'
            }
        }
        '#' => {
            let occupieds_count = neighbors
                .into_iter()
                .filter(|(x, y)| map[*y][*x] == '#')
                .count();
            if occupieds_count >= 4 {
                'L'
            } else {
                current_state
            }
        }
        _ => current_state,
    }
}

fn seats_in_directions(x: usize, y: usize, map: &Vec<Vec<char>>) -> Vec<char> {
    let m = map[0].len();
    let n = map.len();

    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let adjacent_seats: Vec<char> = directions
        .iter()
        .map(|d| {
            let mut xx = x as isize + d.0;
            let mut yy = y as isize + d.1;
            loop {
                if xx < 0 || xx >= m as isize || yy < 0 || yy >= n as isize {
                    return '.';
                }
                let current = map[yy as usize][xx as usize];
                if current == 'L' || current == '#' {
                    return current;
                }
                xx += d.0;
                yy += d.1;
            }
        })
        .collect();

    adjacent_seats
}

fn next_state_2(x: usize, y: usize, map: &Vec<Vec<char>>) -> char {
    let current_state = map[y][x];
    let neighbors: Vec<char> = seats_in_directions(x, y, &map);
    match current_state {
        'L' => {
            if neighbors.into_iter().any(|x| x == '#') {
                current_state
            } else {
                '#'
            }
        }
        '#' => {
            let occupieds_count = neighbors.into_iter().filter(|&x| x == '#').count();
            if occupieds_count >= 5 {
                'L'
            } else {
                current_state
            }
        }
        _ => current_state,
    }
}

fn calculate(input: &str, next_state_fn: &dyn Fn(usize, usize, &Vec<Vec<char>>) -> char) {
    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|l| l.chars().collect())
        .collect();
    let mut new_map: Vec<Vec<char>> = map.iter().cloned().collect();
    let mut last_occupied_count: usize = map
        .iter()
        .map(|v| v.iter().filter(|c| **c == '#').count())
        .sum();
    let mut occupied_count: usize = 1;
    let mut iteration_count = 0;
    while iteration_count < 1000 && occupied_count != last_occupied_count {
        // println!("iteration: {}", iteration_count);
        last_occupied_count = occupied_count;
        for y in 0..(map.len()) {
            for x in 0..(map[0].len()) {
                new_map[y][x] = next_state_fn(x, y, &map);
            }
        }
        occupied_count = new_map
            .iter()
            .map(|v| v.iter().filter(|c| **c == '#').count())
            .sum();
        map = new_map.iter().cloned().collect();
        println!("");
        print!("{}[2J", 27 as char);
        print_map(&map);
        iteration_count += 1;
    }
    println!("count: {}", occupied_count);
}

fn part_1(input: &str) {
    calculate(input, &next_state)
}

fn part_2(input: &str) {
    calculate(input, &next_state_2)
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    part_1(&input);
    part_2(&input);
}
