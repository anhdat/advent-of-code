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
                .map(|(x, y)| map[y][x] == '#')
                .filter(|&x| x)
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

fn print_map(map: &Vec<Vec<char>>) {
    for y in 0..(map.len()) {
        for x in 0..(map[0].len()) {
            print!("{}", map[y][x]);
        }
        print!("\n");
    }
}

fn part_1(input: &str) {
    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|l| l.chars().collect())
        .collect();
    // println!("{:?}", map);
    print_map(&map);
    let mut new_map: Vec<Vec<char>> = map.iter().cloned().collect();
    let mut last_occupied_count: usize = map
        .iter()
        .cloned()
        .map(|v| v.into_iter().filter(|&c| c == '#').count())
        .sum();
    let mut occupied_count: usize = 1;
    let mut iteration_count = 0;
    while iteration_count < 1000 && occupied_count != last_occupied_count {
        // println!("iteration: {}", iteration_count);
        last_occupied_count = occupied_count;
        for y in 0..(map.len()) {
            for x in 0..(map[0].len()) {
                new_map[y][x] = next_state(x, y, &map);
            }
        }
        occupied_count = new_map
            .iter()
            .cloned()
            .map(|v| v.into_iter().filter(|&c| c == '#').count())
            .sum();
        map = new_map.iter().cloned().collect();
        iteration_count += 1;
    }
    println!("count: {}", occupied_count);
}

fn main() {
    let input = include_str!("../input");
    // let input = include_str!("../example");
    part_1(&input);
    // part_2(&input);
}
