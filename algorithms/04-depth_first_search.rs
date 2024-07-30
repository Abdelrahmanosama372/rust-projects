

#[derive(Debug, Clone)]
struct Vertex {
    name: String,
    links: Vec<u8>,
}

struct Graph {
    vertices: Vec<Vertex>,
    visited: Vec<bool>
}

impl Graph {
    fn new(vertices: Vec<Vertex>) -> Self {
        Self { 
            visited: vec![false;vertices.len()], 
            vertices, 
        }
    } 

    fn depth_first_search(&mut self) {
        self.visited[0] = true;
        self.dfs_recursive(0);
    }   

    fn dfs_recursive(&mut self, index: usize) {
        let vertex = self.vertices[index].clone();
        for &link in vertex.links.iter() {
            if  self.visited[link as usize] == false {
                println!("{} - {}", vertex.name, self.vertices[link as usize].name);
                self.visited[link as usize] = true;
                self.dfs_recursive(link as usize);
            }
        }
    }
}
fn main() {
    let vertices = vec![
        Vertex {
            name: "A".to_owned(),
            links: vec![1,2]
        },
        Vertex {
            name: "B".to_owned(),
            links: vec![0,3,4]
        },
        Vertex {
            name: "C".to_owned(),
            links: vec![0,3,5]
        },
        Vertex {
            name: "D".to_owned(),
            links: vec![1,2,4]
        },
        Vertex {
            name: "E".to_owned(),
            links: vec![1,5]
        },
        Vertex {
            name: "F".to_owned(),
            links: vec![2,3,4,7]
        },
        Vertex {
            name: "G".to_owned(),
            links: vec![7,8]
        },
        Vertex {
            name: "H".to_owned(),
            links: vec![5,6,8]
        },
        Vertex {
            name: "I".to_owned(),
            links: vec![6,7]
        },

    ];

    let mut graph = Graph::new(vertices);

    graph.depth_first_search();
}