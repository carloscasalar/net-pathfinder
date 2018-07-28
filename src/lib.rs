pub mod node {
    #[derive(Debug)]
    pub struct Node {
        pub id: String,
        pub name: String,
        pub connections: Vec<String>,
    }

    pub trait Connected {
        fn is_connected_to(&self, node: Node) -> bool;
    }

    impl Connected for Node {
        fn is_connected_to(&self, node: Node) -> bool {
            self.connections.contains(&node.id)
        }
    }

}


#[cfg(test)]
mod is_connected_to_test {
    use node::Node;
    use node::Connected;

    #[test]
    fn it_should_return_false_if_no_node_is_connected() {
        let node_a = Node {
            id: String::from("NodeA"),
            name: String::from("Node A"),
            connections: Vec::new(),
        };

        let node_b = Node {
            id: String::from("NodeB"),
            name: String::from("Node B"),
            connections: Vec::new(),
        };

        assert_eq!(node_a.is_connected_to(node_b), false);
    }
    #[test]
    fn it_should_return_true_if_is_connected_to_node() {
        let node_a = Node {
            id: String::from("NodeA"),
            name: String::from("Node A"),
            connections: vec![String::from("NodeB")],
        };

        let node_b = Node {
            id: String::from("NodeB"),
            name: String::from("Node B"),
            connections: vec![String::from("NodeA")],
        };

        assert_eq!(node_a.is_connected_to(node_b), true);
    }
}
