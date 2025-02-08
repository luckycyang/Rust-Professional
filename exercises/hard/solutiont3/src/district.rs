// src/district.rs
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;

/// Structure to deserialize the JSON data
type DistrictData = HashMap<String, HashMap<String, Vec<String>>>;

pub fn count_provinces() -> String {
    // Path to the district.json file
    let file_path = "district.json";
    let data = fs::read_to_string(file_path).expect("Failed to read file");

    // Parse the JSON data
    let json_data: Value = serde_json::from_str(&data).expect("Invalid JSON data");

    // Initialize a vector to hold the counts for each key
    let mut counts = Vec::new();

    // Define the order of keys to ensure the output is in the correct sequence
    let keys = ["1", "2", "3", "4", "5"];

    for key in &keys {
        if let Some(province_map) = json_data.get(*key) {
            // Build adjacency list
            let mut adj: HashMap<String, Vec<String>> = HashMap::new();

            // Populate the adjacency list
            if let Some(map) = province_map.as_object() {
                for (city, neighbors) in map {
                    let neighbors = neighbors.as_array().unwrap();
                    let neighbor_vec: Vec<_> = neighbors
                        .iter()
                        .map(|n| n.as_str().unwrap().to_string())
                        .collect();
                    adj.entry(city.clone())
                        .or_insert(Vec::new())
                        .extend(neighbor_vec.clone());

                    // Since the graph is undirected, add the reverse connections
                    for neighbor in neighbor_vec {
                        adj.entry(neighbor).or_insert(Vec::new()).push(city.clone());
                    }
                }
            }

            // Perform DFS to count connected components
            let mut visited = HashSet::new();
            let mut province_count = 0;

            for city in adj.keys() {
                if !visited.contains(city) {
                    province_count += 1;
                    dfs(city, &adj, &mut visited);
                }
            }

            counts.push(province_count);
        } else {
            // If the key does not exist, assume 0 provinces
            counts.push(0);
        }
    }

    // Convert the counts to a comma-separated string
    counts
        .iter()
        .map(|count| count.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

// Helper function for Depth-First Search
fn dfs(city: &str, adj: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>) {
    if visited.contains(city) {
        return;
    }
    visited.insert(city.to_string());
    if let Some(neighbors) = adj.get(city) {
        for neighbor in neighbors {
            dfs(neighbor, adj, visited);
        }
    }
}
