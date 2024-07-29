
use queue::Queue;
#[derive(Debug, Clone)]
struct Vertex {
    name: String,
    visited: bool,
    links: Vec<u8>,
}

impl Vertex {
    fn set_vistted(&mut self) {
        self.visited = true;
    }
}
struct Graph {
    vertices: Vec<Vertex>
}

impl Graph {
    fn new(vertices: Vec<Vertex>) -> Self {
        Self { vertices }
    } 

    fn breadth_first_search(&mut self) {
        let mut v_queue = Queue::new();
        self.vertices[0].set_vistted();
        let _ = v_queue.queue(self.vertices[0].clone());

        while v_queue.len() > 0 {
            let vertex = v_queue.dequeue().unwrap();
            for link in vertex.links.iter() {
                if self.vertices[*link as usize].visited == false {
                    self.vertices[*link as usize].set_vistted();
                    let _= v_queue.queue(self.vertices[*link as usize].clone());
                    println!("{} - {}", vertex.name, self.vertices[*link as usize].name);
                }
            }
        }
    }
}
fn main() {
    let vertices = vec![
        Vertex {
            name: "A".to_owned(),
            visited: false,
            links: vec![1,2]
        },
        Vertex {
            name: "B".to_owned(),
            visited: false,
            links: vec![0,3,4]
        },
        Vertex {
            name: "C".to_owned(),
            visited: false,
            links: vec![0,3,5]
        },
        Vertex {
            name: "D".to_owned(),
            visited: false,
            links: vec![1,2,4]
        },
        Vertex {
            name: "E".to_owned(),
            visited: false,
            links: vec![1,5]
        },
        Vertex {
            name: "F".to_owned(),
            visited: false,
            links: vec![2,3,4,7]
        },
        Vertex {
            name: "G".to_owned(),
            visited: false,
            links: vec![7,8]
        },
        Vertex {
            name: "H".to_owned(),
            visited: false,
            links: vec![5,6,8]
        },
        Vertex {
            name: "I".to_owned(),
            visited: false,
            links: vec![6,7]
        },

    ];

    let mut graph = Graph::new(vertices);

    graph.breadth_first_search();
}