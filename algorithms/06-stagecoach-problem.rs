use std::u8;

#[derive(Copy, Clone, Debug)]
struct State {
    from: char,
    to: char,
    cost: u8,
}


fn main() {

    let labels = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

    let data = [
        [0, 2, 4, 3, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 7, 4, 6, 0, 0, 0],
        [0, 0, 0, 0, 3, 2, 4, 0, 0, 0],
        [0, 0, 0, 0, 4, 1, 5, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 4, 0],
        [0, 0, 0, 0, 0, 0, 0, 6, 3, 0],
        [0, 0, 0, 0, 0, 0, 0, 3, 3, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 4],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let n = data.len();

    let state = State {to: ' ', from: ' ', cost: 0};

    let mut states = [state; 10];


    for i in (0..=n-2).rev()
    {
        let mut min_cost = u8::MAX;
        for j in 0..n {
            if data[i][j] == 0 {
                continue;
            }
            if data[i][j] + states[j].cost < min_cost {
                min_cost = data[i][j] + states[j].cost;
                states[i].to = labels[j];
            }
        }
        states[i].from = labels[i];
        states[i].cost = min_cost;
    }

    println!("minimum cost: {}", states[0].cost);

    for i in states.iter() {
        println!("{:?}", i);
    }

    let mut path = vec!['A'];
    let mut curr_char = 'A';

    // get minimum path
    for state in states {
        if curr_char == state.from {
            curr_char = state.to;
            path.push(curr_char);
        }
    }

    println!("minimum path: {:?}", path);
}
