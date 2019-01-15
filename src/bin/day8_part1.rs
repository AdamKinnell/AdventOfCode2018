#[macro_use] mod common;
use self::common::*;

// Types //////////////////////////////////////////////////////////////////////

struct NodeInfo {
    size: usize,
    metadata: i32,
}

// Functions //////////////////////////////////////////////////////////////////

/*
 Calculate the metadata for a node.
 Will recurse for child nodes as necessary.
*/
fn calculate_metadata(node: &[i32]) -> NodeInfo {
    let mut size = 2; // Header
    let mut metadata = 0;

    // Process child nodes
    let child_nodes = node[0];
    for _ in 0..child_nodes {
        let child = &node[size..];
        let child_info = calculate_metadata(&child);
        size += child_info.size;
        metadata += child_info.metadata;
    }

    // Process metadata
    let metadata_nodes = node[1];
    for _ in 0..metadata_nodes {
        metadata += node[size];
        size += 1
    }

    NodeInfo { size:size, metadata:metadata }
}

/*
 Find the sum of all metadata entries.
*/
fn solve(tree: &String) -> i32 {
    let node = tree
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    calculate_metadata(&node).metadata
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~7.5ms
    RELEASE: ~237us
*/
run! {
    input = "day8",
    run = |input: &Input| {
        let metadata_sum = solve(&input.raw());
        assert_eq!(metadata_sum, 40848);
        println!("Metadata sum: {}", metadata_sum);
    },
    bench = |input: &Input| {
        solve(&input.raw());
    }
}