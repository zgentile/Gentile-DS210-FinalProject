use std::collections::VecDeque;

use crate::combine_vectors;

//These are a series of four functions that analyze an undirected graph

// Step 1: Turn the list of nodes and edges into an undirected adjacency list
pub fn adj_list(n: usize, nodes: Vec<usize>, edges: Vec<usize>) -> Vec<Vec<usize>> {

    let connections = combine_vectors(nodes, edges);
    let mut adjacency_list : Vec<Vec<usize>> = vec![vec![];n];

    for (v,w) in connections.iter() {
        adjacency_list[*v as usize].push(*w);
        adjacency_list[*w as usize].push(*v);
    };
    
    adjacency_list

}

// Step 2: Find the shortest distance between any two given points, as long as connection between them exists. If there is no connection, None is returned.
fn find_distance(graph: &Vec<Vec<usize>>, start: usize, end: usize) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visits = vec![false; graph.len()]; // creates vector of visited points


    //setting the start point to be visited and in queue
    visits[start] = true;
    queue.push_back((start, 0));


    while let Some((node, distance)) = queue.pop_front() {
        if node == end {
            return Some(distance);
        }
        for &neighbor in &graph[node] {
            if !visits[neighbor] {
                visits[neighbor] = true;
                queue.push_back((neighbor, distance+1));
            }
        }       
    }
    None
}

// Step 3: Use the previous find_distance function on every n (number of unique nodes) choose 2 combinations of points.
/* This function returns four things.
        1. A vector of distances between every possible combination of points. (or none if there is no distance)
        2. A vector of vectors of tuples. There are n subvectors that represent degrees of separation. Inside those subvectors are the
    pairs of points whose distance between each other aligns with the index of the subvector. The subvector of index 0 is for invalid connections, which
    in the context of my graph (euro road connections), are self connections and pairs who are not connected.
        3. The number of total pairs
        4. The number of invalid connections between pairs */

// Note: If you would like to see every connection (n choose 2) distance, uncomment the print statements in this function.
pub fn calculate_all_distances(adjacency_list: &Vec<Vec<usize>>) -> (Vec<usize>, Vec<Vec<(usize, usize)>>, usize, usize)  {
    let mut index = 0;
    let mut invalid_connections = 0;
    let length = adjacency_list.len();
    let mut num_connections = 0;
    let mut distances = vec![0 ; (adjacency_list.len() * (adjacency_list.len() + 1)) / 2];
    let mut organized_by_dist : Vec<Vec<(usize, usize)>> = vec![Vec::new(); adjacency_list.len()];

    for start in 0..length {
        for end in start..length {
            // Is there a connection between the 2 points?
            if let Some(x) = find_distance(&adjacency_list, start, end) {
                distances[index] = x;
                if distances[index] != 0 {
                num_connections += 1;
                //println!("The {}th distance is {}, between {} and {}.",  num_connections, distances[index], start, end);
                organized_by_dist[distances[index]].push((start, end));
                index += 1;
                }
            } else {
                organized_by_dist[0].push((start, end));
                index += 1;
                invalid_connections += 1;
                //println!("There is no path between {} and {}", start, end);
            }
        }
    }

    (distances, organized_by_dist, num_connections, invalid_connections)
}

// Step 4: Show what percentage of connections have n degrees of separations are invalid
/* Function can be used in various ways, output depends on how it is used.
   Intended to be used in a for loop to show the entire distribution, but can be used to show individual checks as well.
   If you want to disclude invalid connections in the distribution, start your for loop at 1 and use "valid connections" as input */
pub fn separation_distribution(sorted_distances: &Vec<Vec<(usize, usize)>>, i: usize, connections: usize) -> f32 {
    100 as f32 * sorted_distances[i].len() as f32/(1.0 + connections as f32)
}