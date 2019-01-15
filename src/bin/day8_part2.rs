#[macro_use] mod common;
use self::common::*;

// Types //////////////////////////////////////////////////////////////////////

struct NodeInfo {
    size: usize,
    value: usize,
}

// Functions //////////////////////////////////////////////////////////////////

/*
 Calculate the metadata for a node.
 Will recurse for child nodes as necessary.
*/
fn calculate_metadata(node: &[usize]) -> NodeInfo {
    let mut size = 2; // Header

    // Process child nodes
    let num_child_nodes = node[0];
    let mut child_nodes: Vec<NodeInfo> = Vec::new();
    for _ in 0..num_child_nodes {
        let child = &node[size..];
        let child_info = calculate_metadata(&child);
        size += child_info.size;
        child_nodes.push(child_info);
    }

    // Calculate value from metadata
    let value;
    let num_metadata_entries = node[1];
    if num_child_nodes == 0 {
        // Sum metadata entries
        value = node[size..].iter().take(num_metadata_entries).sum();
    } else {
        // Sum values of referenced child nodes
        value = node[size..].iter().take(num_metadata_entries).map(|m| {
            child_nodes.get(*m - 1).map_or(0, |c| c.value)
        }).sum()
    }
    size += num_metadata_entries;

    NodeInfo { size:size, value:value }
}

/*
 Find the value of the root node.
*/
fn solve(tree: &String) -> usize {
    let node = tree
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    calculate_metadata(&node).value
}

// Entry Point ////////////////////////////////////////////////////////////////

/*
 Timings:
    DEBUG: ~9.3ms
    RELEASE: ~391us
*/
run! {
    input = "day8",
    run = |input: &Input| {
        let root_value = solve(&input.raw());
        assert_eq!(root_value, 34466);
        println!("Value of root: {}", root_value);
    },
    bench = |input: &Input| {
        solve(&input.raw());
    }
}