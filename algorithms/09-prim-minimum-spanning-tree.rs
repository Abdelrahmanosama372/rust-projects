use core::f32;

fn main() {
    let labels = ['1', '2', '3', '4', '5', '6'];

    let graph: [[f32; 6]; 6] = [
        [0.0, 6.7, 5.2, 2.8, 5.6, 3.6],
        [6.7, 0.0, 5.7, 7.3, 5.1, 3.2],
        [5.2, 5.7, 0.0, 3.4, 8.5, 4.0],
        [2.8, 7.3, 3.4, 0.0, 8.0, 4.4],
        [5.6, 5.1, 8.5, 8.0, 0.0, 4.6],
        [3.6, 3.2, 4.0, 4.4, 4.6, 0.0],
    ];

    let mut selected = [false; 6];
    let mut number_of_links = 0;

    selected[0] = true;
    while number_of_links < graph.len()-1 {
        let mut min_path = f32::MAX;
        let mut pos = (0,0);
        for i in 0..graph.len() {
            if selected[i] == true {
                for j in 0..graph.len() {
                    if graph[i][j] == 0 as f32 {
                        continue;
                    }
                    if selected[j] == false && graph[i][j] < min_path {
                        min_path = graph[i][j];
                        pos = (i,j);
                    }
                }
            }
        }

        selected[pos.1] = true;
        println!("{} -> {}", labels[pos.0], labels[pos.1]);
        number_of_links += 1;
    }
}
