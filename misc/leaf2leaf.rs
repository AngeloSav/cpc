struct Node {
    key: i32,
    left: Box<Option<Node>>,
    right: Box<Option<Node>>,
}

//best path, best solution
fn max_leaf_path(u: Option<Node>) -> (i32, i32) {
    if u.is_none() {
        return (0, 0);
    }

    let u = u.unwrap();
    let (bpl, bsl) = max_leaf_path(*u.left);
    let (bpr, bsr) = max_leaf_path(*u.right);

    let best_solution_here = bpr + u.key + bpl;

    let best_sol = if best_solution_here > bsl && best_solution_here > bsr {
        best_solution_here
    } else {
        bsl.max(bsr)
    };

    (best_sol, bsl.max(bsr) + u.key)
}

fn main() {
    println!("ok");
}
