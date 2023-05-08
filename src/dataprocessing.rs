// These three functions are a sequence of operations that reduce the sparsity of any given graph.

// Step 1: Combine all unique nodes and edges into one column and then sort them from least to greatest
pub fn merge_and_sort_columns(column1: Vec<usize>, column2: Vec<usize>) -> Vec<usize> {
    // Merge both columns into one vector
    let mut all_values = vec![];
    all_values.append(&mut column1.clone());
    all_values.append(&mut column2.clone());

    // Use a hash set to only keep unique numbers from both columns
    let mut unique_numbers = std::collections::HashSet::new();
    for number in all_values {
        unique_numbers.insert(number);
    }

    // Convert the hash set back to a vector and sort it
    let mut sorted_numbers = Vec::from_iter(unique_numbers);
    sorted_numbers.sort();

    sorted_numbers
}

// Step 2: Reassign every value in the original nodes and edges to their index in the sorted vector of all unique values

// ex: if 878 was the 37th smallest number, it will become 36 (because the smallest number becomes 0)
pub fn reassign_indexes(index_vec: &Vec<usize>, mut input_vec: Vec<usize>) -> Vec<usize> {
    for elem in &mut input_vec {
        if let Ok(i) = index_vec.binary_search(elem) {
            *elem = i as usize;
        }
    }
    input_vec
}

// Step 3: Combine the vectors with the replaced values back together so graph analysis can be performed.
pub fn combine_vectors(v1: Vec<usize>, v2: Vec<usize>) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(v1.len());
    for i in 0..v1.len() {
        result.push((v1[i], v2[i]));
    }
    result
}

// Note: If necessary, any original value of a reassigned number can be recovered by referring to the sorted numbers vector, 
// which is returned by the merge_and_sort_columns function.