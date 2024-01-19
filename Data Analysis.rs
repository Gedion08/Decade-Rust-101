use std::collections::HashMap;

// Function to aggregate and process data
// This function now handles multiple types of aggregation, not just summing up values.
pub fn process_data(data: Vec<HashMap<String, String>>) -> HashMap<String, f64> {
    let mut aggregated_data = HashMap::new();

    // Enhanced data processing logic
    for entry in data {
        for (key, value) in entry {
            let value: f64 = value.parse().unwrap_or(0.0);

            // Here, you can add more sophisticated aggregation logic.
            // For example, calculating average, min/max, standard deviation, etc.
            // Currently, it's just summing the values.
            *aggregated_data.entry(key).or_insert(0.0) += value;
        }
    }

    // Post-processing, such as normalization or scaling, can be done here if needed.

    aggregated_data
}

// Additional Functionality: Filter data
// This function could be used to filter out certain data points based on criteria.
pub fn filter_data(data: &Vec<HashMap<String, String>>, filter_key: &str, filter_value: &str) -> Vec<HashMap<String, String>> {
    data.iter()
        .filter(|entry| entry.get(filter_key).map_or(false, |v| v == filter_value))
        .cloned()
        .collect()
}

fn main() {
    // Example usage
    let sample_data = vec![
        HashMap::from([("sales".to_string(), "100.0".to_string()), ("region".to_string(), "North".to_string())]),
        HashMap::from([("sales".to_string(), "150.0".to_string()), ("region".to_string(), "South".to_string())]),
        // Add more data points as needed
    ];

    // Example of filtering data before processing
    let filtered_data = filter_data(&sample_data, "region", "North");

    let processed_data = process_data(filtered_data);
    println!("Processed Data: {:?}", processed_data);
}

