use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Reverse;
use float_cmp::approx_eq;

#[derive(Debug, Clone)]
struct Node {
	pub id: usize,
	pub out: HashMap<usize, OrderedFloat<f32>>

}

impl Node {
	fn add_edge(&mut self, edge: Edge) {
		self.out.insert(edge.to_id, edge.weight);
	}
}

#[derive(Hash, Eq, PartialEq)]
struct LookupEdge {
	pub from: usize,
	pub to: usize
}

struct Edge {
	pub weight: OrderedFloat<f32>,
	pub to_id: usize
}

#[derive(Debug, Clone)]
struct Graph {
	pub nodes: Vec<Node>
}

fn shortest_path(start_node: &Node, end_node: &Node, graph: &Graph, exclude_nodes: HashSet<usize>, exclude_edges: HashSet<LookupEdge>, quiet: bool) -> (Vec<usize>, OrderedFloat<f32>) {
	if !quiet {
		println!("searching dijkstra's, start_id={}, end_id={}", start_node.id, end_node.id);
	}

	assert!(!exclude_nodes.contains(&start_node.id) && !exclude_nodes.contains(&end_node.id));

	let mut pq = PriorityQueue::new();

	let num_nodes = graph.nodes.len();

	let mut visited: Vec<bool> = vec![false; num_nodes];

	let mut path: Vec<usize> = vec![num_nodes + 1; num_nodes];

	pq.push((start_node.id, 0), Reverse(OrderedFloat(0.0)));

	while !pq.is_empty() {
		let ((node_id, prev_id), node_cost) = match pq.pop() {
			Some(x) => x,
			None => panic!("No value found in pq")
		};

		//println!("{} -> {}", prev_id, node_id);

		let node = &graph.nodes[node_id];

		if visited[node_id] {
			continue;
		}

		visited[node_id] = true;

		assert!(path[node_id] == num_nodes + 1); // assert hasn't been set in path before
		path[node_id] = prev_id;

		if node_id == end_node.id {
			// build path in reverse then flip
			let mut result_path = Vec::new();

			result_path.push(node_id);
			result_path.push(prev_id);

			let mut curr_id = prev_id;

			while curr_id != start_node.id {
				let backward = path[curr_id];

				result_path.push(backward);

				curr_id = backward;
			}

			result_path.reverse();

			if !quiet {
				println!("dijkstra's result: {:?}", result_path);
			}

			return (result_path, node_cost.0);
		}

		for (to_id, weight) in &node.out {

			if !exclude_nodes.contains(to_id) && !exclude_edges.contains(&LookupEdge {
				from: node_id,
				to: *to_id
			}){
				//Reverse.0 gets original value of reverse
				pq.push((*to_id, node_id), Reverse(node_cost.0 + weight));

				if !quiet {
					println!("dijkstra's path edge: {} -> {}", node_id, to_id);
				}	
			}
		}
	}

	(Vec::new(), OrderedFloat(0.0))
}

// https://en.wikipedia.org/wiki/Yen%27s_algorithm
fn top_k_shortest_paths(start_node: &Node, end_node: &Node, graph: &Graph, k: usize, quiet: bool) -> (Vec<Vec<usize>>, Vec<OrderedFloat<f32>>) {
	let mut shortest_paths = Vec::new();
	let mut shortest_costs = Vec::new();

	let mut added_paths = HashSet::new();

	let (p, c) = shortest_path(start_node, end_node, graph, HashSet::new(), HashSet::new(), quiet);
	added_paths.insert(p.clone());
	shortest_paths.push(p);
	shortest_costs.push(c);

	let mut potential_paths = PriorityQueue::new();

	for k_idx in 1..k {
		if !quiet {
			println!("trying k={}", k_idx);
		}
		for spur_idx in 0..(shortest_paths[k_idx - 1].len() - 1) {
			if !quiet {
				println!("trying spur={}", spur_idx);
			}

			let spur_node = shortest_paths[k_idx - 1][spur_idx];
			let root_path = &shortest_paths[k_idx - 1][..(spur_idx + 1)];

			//println!("root len {}", root_path.len());

			let mut deleted_edges = HashSet::new();
			for p in shortest_paths.iter() {
				if p.len() >= (spur_idx + 1) && root_path == &p[..(spur_idx + 1)] {
					// remove edge spur_idx to spur_idx + 1
					//graph.nodes[spur_node].out.remove(&shortest_paths[k_idx - 1][spur_idx + 1]);

					deleted_edges.insert(LookupEdge {
						from: spur_node,
						to: p[spur_idx + 1]
					});

					if !quiet {
						println!("deleting edge {}-{}", spur_node, p[spur_idx + 1]);
					}	
				}
			}

			let mut deleted_nodes = HashSet::new();
			for node_id in root_path[..spur_idx].iter() {
				//graph.nodes.remove(*node_id);
				deleted_nodes.insert(*node_id);

				if !quiet {
					println!("deleting node {}", node_id);
				}
			}

			let (spur_path, _) = shortest_path(&graph.nodes[spur_node], end_node, graph, deleted_nodes, deleted_edges, quiet);

			if spur_path.len() == 0 {
				if !quiet {
					println!("no path found, skipping");
				}
				continue;
			} else {
				if !quiet {
					println!("found path of size {}", spur_path.len());
				}
			}

			let result_path = [root_path, &spur_path[1..]].concat();

			let mut cost = OrderedFloat(0.0);

			for i in 0..(result_path.len() - 1) {
				let edge_cost = match graph.nodes[result_path[i]].out.get(&result_path[i + 1]) {
					Some(x) => x,
					None => panic!("Invalid edge when calculating cost")
				};
				cost += edge_cost;
			}

			//println!("result len {}", result_path.len());

			if !added_paths.contains(&result_path) {
				if !quiet {
					println!("adding to potential paths: {} {:?}", cost, result_path);
				}
				
				added_paths.insert(result_path.clone());
				potential_paths.push(result_path, Reverse(cost));
			} else {
				if !quiet {
					println!("duplicate paths: {} {:?}", cost, result_path);
				}
			}
		}

		if potential_paths.len() == 0 {
			break;
		} else {
			let (new_path, new_cost) = match potential_paths.pop() {
				Some(x) => x,
				None => panic!("No potential paths paths found")
			};
			shortest_paths.push(new_path);
			shortest_costs.push(new_cost.0); // Reverse.0 to get value from Reverse
		}
	}

	(shortest_paths, shortest_costs)
}

fn main() {
	let mut array = Vec::new();

	//array.push(vec![0.2, 0.1, 0.9, 0.2, 0.1, 0.8]);
	//array.push(vec![0.3, 0.2, 1.0, 0.2, 0.2, 0.9]);
	//array.push(vec![0.4, 0.5, 1.0, 0.3, 0.3, 1.0]);
	//array.push(vec![0.5, 0.6, 1.0, 0.4, 0.4, 1.0]);
	//array.push(vec![0.6, 0.8, 1.0, 0.5, 0.5, 1.0]);

	let mut check_length = 0;
	
	let mut line_all = String::new();
	std::io::stdin().read_line(&mut line_all).unwrap();

	let mut top_k = 0;

	for (line_idx, line) in line_all.split("|").enumerate() {
		if line_idx == 0 {
			top_k = line.trim().parse::<usize>().unwrap();
			continue;
		}

		let mut new_nums = Vec::new();

		for num_str in line.split(" ") {
			new_nums.push(num_str.trim().parse::<f32>().unwrap());
		}

		assert!(new_nums.len() != 0);

		if check_length == 0 {
			check_length = new_nums.len();
		} else {
			assert!(new_nums.len() == check_length);
		}

		array.push(new_nums);
	}

	let vocab_size = array.len();
	let sequence_length = array[0].len();
	let num_nodes = sequence_length * vocab_size + 2;

	let mut g = Graph {
		nodes: Vec::new()
	};

	for i in 0..num_nodes {
		g.nodes.push(Node {
			id: i,
			out: HashMap::new()
		});
	}

	// starting node to first column
	for i in 0..vocab_size {
		g.nodes[0].add_edge(Edge {
			weight: OrderedFloat(array[i][0]),
			to_id: i + 1
		});
	}

	// middle
	for c in 0..(sequence_length-1) {
		for r in 0..(vocab_size) {
			let idx = c * vocab_size + r + 1;
			for next_r in 0..(vocab_size) {
				assert!(((c + 1) * vocab_size + next_r + 1) < num_nodes - 1);
				g.nodes[idx].add_edge(Edge {
					weight: OrderedFloat(array[next_r][c + 1]),
					to_id: (c + 1) * vocab_size + next_r + 1
				});
			}
		}
	}

	// ending node
	for i in 0..vocab_size {
		g.nodes[(sequence_length - 1) * vocab_size + i + 1].add_edge(Edge {
			weight: OrderedFloat(0.0),
			to_id: num_nodes - 1
		});
	}

	//let (d_path, d_cost) = shortest_path(&g.nodes[0], &g.nodes[num_nodes - 1], &g, HashSet::new(), HashSet::new(), true);
	//println!("{} {:?}", d_cost, d_path);

	let (paths, costs) = top_k_shortest_paths(&g.nodes[0], &g.nodes[num_nodes - 1], &g, top_k, true);

	for (p, c) in paths.iter().zip(costs.iter()) {
		//print!("{} | START -> ", c);
		let mut cost_test = 0.0;

		for (iter_idx, idx) in p[1..p.len() - 1].iter().enumerate() {
			let col = (idx - 1) / vocab_size;
			let row = (idx - 1) - col * vocab_size;

			//print!("({}, {}, w={}) -> ", col, row, array[row][col]);
			print!("{} ", row);

			assert!(iter_idx == col);

			cost_test += array[row][col];
		}
		println!("c={}", c);

		assert!(approx_eq!(f32, **c, cost_test));

		//println!("END");
	}
}
