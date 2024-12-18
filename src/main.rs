use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    key: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(key: i32) -> Self {
        TreeNode {
            key,
            left: None,
            right: None,
        }
    }
}

fn optimal_bst(keys: &[i32], freq: &[f64]) -> (f64, Option<Rc<RefCell<TreeNode>>>) {
    let n = keys.len();
    let mut cost = vec![vec![0.0; n]; n];
    let mut root = vec![vec![0; n]; n];
    let mut sum_freq = vec![vec![0.0; n]; n];

    // Fill `sum_freq` for cumulative frequencies
    // Fills the upper right portion on the matix above the diagonal
    for i in 0..n {
        sum_freq[i][i] = freq[i];
        for j in i + 1..n {
            sum_freq[i][j] = sum_freq[i][j - 1] + freq[j];
        }
    }

    // Initialize cost of single keys and set roots to self
    // Fills the matrix diagonal
    for i in 0..n {
        cost[i][i] = freq[i];
        root[i][i] = i;
    }

    // Build the cost and root tables for chains of length 2 to n
    // Subtrees of length 2
    for length in 2..=n {
        // i is the starting index of the subtree
        for i in 0..=n - length {
            let j = i + length - 1;
            cost[i][j] = f64::INFINITY;

            // Try each key in range (i..j) as root
            // j is the ending index of the subtree
            for r in i..=j {
                println!("l {length}, i {i}, j {j}, r {r}");
                let left_cost = if r > i { cost[i][r - 1] } else { 0.0 };
                let right_cost = if r < j { cost[r + 1][j] } else { 0.0 };
                let total_cost = left_cost + right_cost + sum_freq[i][j];
                println!("left {left_cost}, right {right_cost}, tot {total_cost}");

                // Update the minimum cost and root if this is the best configuration
                if total_cost < cost[i][j] {
                    cost[i][j] = total_cost;
                    root[i][j] = r;
                }
            }
        }
    }

    // Function to build the tree from the root table
    fn build_tree(
        keys: &[i32],
        root: &[Vec<usize>],
        i: usize,
        j: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if i > j {
            return None;
        }
        let r = root[i][j];
        let node = Rc::new(RefCell::new(TreeNode::new(keys[r])));
        
        // Recursively build left and right subtrees only if indices are valid
        node.borrow_mut().left = if r > 0 && r - 1 >= i {
            build_tree(keys, root, i, r - 1)
        } else {
            None
        };
        
        node.borrow_mut().right = if r + 1 <= j {
            build_tree(keys, root, r + 1, j)
        } else {
            None
        };
        
        Some(node)
    }

    // Build the tree and return the minimum cost and the root of the optimal tree
    let tree = build_tree(keys, &root, 0, n - 1);
    (cost[0][n - 1], tree)
}

fn main() {
    let keys = vec![10, 12, 20];
    let freq = vec![0.2, 0.5, 0.3];

    let (min_cost, root) = optimal_bst(&keys, &freq);
    println!("Minimum cost of the Optimal Binary Search Tree: {}", min_cost);
    println!("Root of the Optimal Binary Search Tree: {:?}", root);
}
