use std::fs::File;
use std::io::{BufRead, BufReader};

mod dataprocessing;
use crate::dataprocessing::{merge_and_sort_columns, reassign_indexes, combine_vectors};

mod degrees_separation;
use crate::degrees_separation::{adj_list, calculate_all_distances, separation_distribution};

mod statistics;
use crate::statistics::{average, calculate_stdv};


fn main() {
    // Extract column of nodes and column of edges
    let tuple = readfile("euroroad.csv");
    let column1 = tuple.0;
    let column2 = tuple.1;

    // Create an index vector of all unique numbers sorted from least to greatest
    let sorted = merge_and_sort_columns(column1.clone(), column2.clone());

    // Reassign each value in the nodes and edges according to their index in the sorted vector in order to reduce sparsity
    let newcol1 = reassign_indexes(&sorted, column1);
    let newcol2 = reassign_indexes(&sorted, column2);

    // Create an adjacency list with the reassigned vectors
    let adj = adj_list(sorted.len(), newcol1.clone(), newcol2.clone());

    // If you would like to print the adjacency list, uncomment the line below
    // println!("{:?}", adj_list(sorted.len(), newcol1, newcol2));

    // Using graph analysis functions from "degrees_separation", collect all the distances, a sorted list of them, and the number of valid and invalid connections between pairs
    let dist_info = calculate_all_distances(&adj);
    let distances = dist_info.0;
    let sorted_distances = dist_info.1;
    let num_connections = dist_info.2;
    let invalid_connections = dist_info.3;
    let valid_connections = distances.len() - invalid_connections;

    // A implementation of the separation_distribution function. Here, it prints out the percentage of valid connections that each degree of separation from 1 to max has.
    // If you uncomment the first line, it will print out every pair that falls within each degree of separation
    /* If you use 0 as an index and total amount of pairs instead of valid connections, it will show what percentage of invalid connections there are compared to the total number of combinations of points.
       However, this pertains only to nodes that have no connections at all and the combinations of nodes to themselves */

    let mut connection_percentage: Vec<f32> = vec![0.0;*distances.iter().max().unwrap() + 1];
    for i in 1..=*distances.iter().max().unwrap() {
        //println!("The connections with {} degrees of separation are: {:?}", i, sorted_distances[i]);
        println!("The connections with {} degrees of separation are {}% of the valid connections.", i, separation_distribution(&sorted_distances, i, valid_connections));
        connection_percentage[i] = separation_distribution(&sorted_distances, i, valid_connections) + connection_percentage[i - 1]
    }

    // Prints out the number of valid and invalid connections, the mean and standard deviation of separation, the max distance, and what percentage of connected nodes can be reached within 6 and 20 degrees of separation.
    println!("----------------");
    println!("Invalid - (no connection or self connection): {}", invalid_connections);
    println!("Valid connections: {}", valid_connections);
    println!("Mean separation: {}", average(&distances, num_connections));
    println!("Max distance: {}", distances.iter().max().unwrap());
    println!("Standard deviation of separation: {}", calculate_stdv(&distances, num_connections));
    println!("The percentage of connected nodes that can be reached within six degrees of separation is {}%", connection_percentage[6]);
    println!("The percentage of connected nodes that can be reached within twenty degrees of separation is {}%", connection_percentage[20]);
    println!("----------------");
}

// Reads a graph file and returns it as usize columns of nodes and edges
fn readfile(path: &str) -> (Vec<usize>, Vec<usize>) {
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    let mut nodes: Vec<usize> = Vec::new();
    let mut edges: Vec<usize> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result;

        let values: Vec<Option<i32>> = line.expect("line could not be read").split(',').map(|s| s.trim().parse().ok()).collect();

        if let (Some(a), Some(b)) = (values[0], values[1]) {
            nodes.push(a as usize);
            edges.push(b as usize);
        }
    }
    return (nodes, edges)
}

//TESTS BELOW

#[test]
fn check_self_connection() {
    // Should return Some(0) if the same point is passed twice
    let adjacency_test : Vec<Vec<usize>> =  vec![vec![],vec![1], vec![3], vec![2]];
    assert_eq!(degrees(&adjacency_test, 1, 1), Some(0));
}

#[test]
fn check_one_degree() {
    // Should return Some(1) on a pair of points with one degree of separation
    let adjacency_test : Vec<Vec<usize>> =  vec![vec![],vec![1], vec![3], vec![2]];
    assert_eq!(degrees(&adjacency_test, 2, 3), Some(1));
}

#[test]
fn check_no_connections() {
    // Should return None on a point with no connections to the other point
    let adjacency_test : Vec<Vec<usize>> =  vec![vec![],vec![1], vec![3], vec![2]];
    assert_eq!(degrees(&adjacency_test, 0, 1), None);
}

#[test]
fn check_valid_connections_count() {
    // Total connections - valid connections = invalid connections
    let adjacency_test : Vec<Vec<usize>> =  vec![vec![],vec![1], vec![3], vec![2]];
    let dist_info = calculate_all_distances(&adjacency_test);
    let distances = dist_info.0;
    
    let num_connections = dist_info.2;
    let invalid_connections = dist_info.3;
    let valid_connections = distances.len() - invalid_connections;

    println!("Invalid - (no connection or self connection): {}", invalid_connections);
    println!("Connections check: {}", distances.len() - invalid_connections);
    println!("Valid connections: {}", valid_connections);

    assert_eq!(invalid_connections, distances.len() - valid_connections)
}