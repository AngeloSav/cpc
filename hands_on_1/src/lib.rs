pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    /// This method returns true if the tree is a BST
    pub fn is_bst(&self) -> bool {
        // call the helper method on the root of the tree and returns
        // the first element of the tuple, which is the predicate `is the tree a BST?`
        self.is_bst_rec(Some(0)).0
    }

    /// private method that recursively checks that the keys of a node and its chiledren
    /// respect the bst property and its subtrees do too
    /// It returns a triple (is_bst, min value in the tree, max value in the tree)
    fn is_bst_rec(&self, node_id: Option<usize>) -> (bool, u32, u32) {
        match node_id {
            Some(node_id) => {
                let cur_node = &self.nodes[node_id];
                let l_child_id = cur_node.id_left;
                let r_child_id = cur_node.id_right;

                let mut prop = true;

                // call is_bst_rec on the left and right subtrees
                let (lprop, lmin, lmax) = self.is_bst_rec(l_child_id);
                let (rprop, rmin, rmax) = self.is_bst_rec(r_child_id);

                // both the subtrees need to be BSTs
                prop &= lprop & rprop;

                // to be a bst, the current node key needs to be:
                // - bigger or equal than the biggest key in the left subtree
                // - less than the smallest key in the right subtree
                prop &= lmax <= cur_node.key;
                prop &= rmin > cur_node.key;

                (
                    prop,
                    *[lmin, rmin, cur_node.key].iter().max().unwrap(),
                    *[lmax, rmax, cur_node.key].iter().max().unwrap(),
                )
            }
            //base case
            None => (true, u32::MAX, u32::MIN),
        }
    }

    /// Returns the maximum path sum between to special nodes
    /// A special node is a node which is connected to exactly one different node.
    pub fn max_path_sum(&self) -> u32 {
        // Return the best solution we found,
        // we can safely unwrap because we know that the tree has always at least one node (the root)
        self.max_path_sum_rec(Some(0)).1.unwrap()
    }

    /// Private method that recursively calculates the maximum path sum in a subtree rooted in `node_id`
    /// It returns a pair (max path from leaf to root, best solution so far)
    fn max_path_sum_rec(&self, node_id: Option<usize>) -> (u32, Option<u32>) {
        match node_id {
            Some(node_id) => {
                let cur_node = &self.nodes[node_id];

                // calculate the best path from leaf to the children and the best solution for both the subtrees
                let (bpl, bsl) = self.max_path_sum_rec(cur_node.id_left);
                let (bpr, bsr) = self.max_path_sum_rec(cur_node.id_right);

                // the best candidate solution for this subtree is the best two paths
                // from leaf to each of the children + the current node key
                let best_solution_here = bpl + bpr + cur_node.key;

                // calculate the best solution for this subtree, comparing `best_solution_here`
                // with the previously found ones (if they exist) and taking the max.
                let best_sol = match (bsl, bsr) {
                    (None, None) => best_solution_here,
                    (Some(x), Some(y)) => *[best_solution_here, x, y].iter().max().unwrap(),
                    (None, Some(x)) => best_solution_here.max(x),
                    (Some(x), None) => best_solution_here.max(x),
                };

                //return the best path from a leaf up to the current node and the best solution we found so far
                (cur_node.key + bpl.max(bpr), Some(best_sol))
            }

            // base case, the best path from leaf to itself is 0,
            // there is no best solution in the empty tree
            None => (0, None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);
        assert_eq!(tree.is_bst(), true);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);
        assert_eq!(tree.is_bst(), true);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);
        assert_eq!(tree.is_bst(), true);
    }

    #[test]
    fn test_is_bst_root() {
        let tree = Tree::with_root(1);
        assert_eq!(tree.is_bst(), true);
    }

    #[test]
    fn test_is_bst() {
        let mut tree = Tree::with_root(50);
        assert_eq!(tree.is_bst(), true);

        tree.add_node(0, 15, true);
        assert_eq!(tree.is_bst(), true);

        tree.add_node(0, 90, false);
        assert_eq!(tree.is_bst(), true);

        tree.add_node(1, u32::MAX, false);
        assert_eq!(tree.is_bst(), false);
    }

    #[test]
    fn test_max_path_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.max_path_sum(), tree.sum())
    }

    #[test]
    fn test_max_path_sum_2() {
        let mut tree = Tree::with_root(3);

        tree.add_node(0, 4, true); // id 1
        tree.add_node(0, 5, false); // id 2

        tree.add_node(1, 2, true);
        tree.add_node(1, 4, false);

        // tree.add_node(2, 20, true);

        assert_eq!(tree.max_path_sum(), 16);
        assert_eq!(tree.is_bst(), false);
    }

    #[test]
    fn test_max_path_sum_3() {
        let mut tree = Tree::with_root(0);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 6, false); // id 2

        tree.add_node(1, 8, true); // 3
        tree.add_node(1, 1, false);

        tree.add_node(3, 2, true);
        tree.add_node(3, 3, false);

        tree.add_node(2, 3, true);
        tree.add_node(2, 9, false);

        tree.add_node(8, 0, false);

        tree.add_node(9, 4, true);
        tree.add_node(9, 1, false);

        tree.add_node(11, 10, true);

        assert_eq!(tree.max_path_sum(), 42);
        assert_eq!(tree.is_bst(), false);
    }

    #[test]
    fn test_max_path_sum_4() {
        let mut tree = Tree::with_root(2);

        tree.add_node(0, 2, true); // id 1

        tree.add_node(1, 1, false); // 2
        assert_eq!(tree.max_path_sum(), 5);

        tree.add_node(1, 3, true); // 3
        assert_eq!(tree.max_path_sum(), 7);

        tree.add_node(2, 6, false);
        assert_eq!(tree.max_path_sum(), 12);

        assert_eq!(tree.is_bst(), false);
    }
}
