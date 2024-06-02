use leptos_use::core::Position;
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BoxNode {
    pub id: String,
    pub pos: Position,
    pub disp: Position,
    pub degree: usize,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<BoxNode>,
    edges: Vec<(String, String)>,
    node_indices: HashMap<String, usize>,
}

impl Graph {
    pub fn new(nodes: Vec<String>, edges: Vec<(String, String)>) -> Self {
        let mut rng = rand::thread_rng();
        let mut node_map: HashMap<String, BoxNode> = nodes
            .into_iter()
            .map(|id| {
                (
                    id.clone(),
                    BoxNode {
                        id,
                        pos: Position {
                            x: rng.gen(),
                            y: rng.gen(),
                        },
                        disp: Position { x: 0.0, y: 0.0 },
                        degree: 0,
                    },
                )
            })
            .collect();

        for (id1, id2) in &edges {
            node_map.get_mut(id1).unwrap().degree += 1;
        }

        for (id1, id2) in &edges {
            node_map.get_mut(id1).unwrap().degree += node_map.get_mut(id2).unwrap().degree;
        }

        let nodes: Vec<BoxNode> = node_map.values().cloned().collect();
        let node_indices = node_map
            .into_iter()
            .map(|(id, node)| (id, node.id))
            .enumerate()
            .map(|(i, (id, _))| (id, i))
            .collect();
        Graph {
            nodes,
            edges,
            node_indices,
        }
    }

    pub fn fruchterman_reingold(
        &mut self,
        iterations: usize,
        area: f64,
        gravity: f64,
        speed: f64,
        tolerance: f64,
    ) {
        let num_nodes = self.nodes.len();
        let k = (area / num_nodes as f64).sqrt();

        fn repulsive_force(dist: f64, k: f64) -> f64 {
            k * k / dist
        }

        fn attractive_force(dist: f64, k: f64) -> f64 {
            dist * dist / k
        }

        for _ in 0..iterations {
            for node in &mut self.nodes {
                node.disp = Position { x: 0.0, y: 0.0 };
            }

            for i in 0..num_nodes {
                for j in 0..num_nodes {
                    if i != j {
                        let delta = Position {
                            x: self.nodes[i].pos.x - self.nodes[j].pos.x,
                            y: self.nodes[i].pos.y - self.nodes[j].pos.y,
                        };
                        let dist = (delta.x * delta.x + delta.y * delta.y).sqrt();
                        if dist > 0.0 {
                            let force = repulsive_force(dist, k);
                            self.nodes[i].disp.x += (delta.x / dist) * force;
                            self.nodes[i].disp.y += (delta.y / dist) * force;
                        }
                    }
                }
            }

            for &(ref id1, ref id2) in &self.edges {
                let index1 = self.node_indices[id1];
                let index2 = self.node_indices[id2];
                let [node1, node2] = self
                    .nodes
                    .get_many_mut([index1, index2])
                    .expect("out of bounds or overlapping indices");
                let delta = Position {
                    x: node1.pos.x - node2.pos.x,
                    y: node1.pos.y - node2.pos.y,
                };
                let dist = (delta.x * delta.x + delta.y * delta.y).sqrt();
                if dist > 0.0 {
                    let force = attractive_force(dist, k);
                    let displacement = Position {
                        x: (delta.x / dist) * force,
                        y: (delta.y / dist) * force,
                    };
                    node1.disp.x -= displacement.x;
                    node1.disp.y -= displacement.y;
                    node2.disp.x += displacement.x;
                    node2.disp.y += displacement.y;
                }
            }

            let mut max_disp = 0.0;
            for node in &mut self.nodes {
                let dist = (node.disp.x * node.disp.x + node.disp.y * node.disp.y).sqrt();
                if dist > 0.0 {
                    node.pos.x += (node.disp.x / dist) * dist.min(speed);
                    node.pos.y += (node.disp.y / dist) * dist.min(speed);
                    if (dist > max_disp) {
                        max_disp = dist;
                    }
                }
                node.pos.x += (0.0 - node.pos.x) * gravity;
                node.pos.y += (0.0 - node.pos.y) * gravity;
            }

            if max_disp < tolerance {
                break;
            }
        }
    }
}
