pub mod node {
    pub trait Point {
        fn id(&self) -> &str;
    }

    #[derive(Debug)]
    pub struct Connection<'a> {
        pub to: &'a str
    }

    #[derive(Debug)]
    pub struct Node<'a, T> {
        pub point: T,
        pub connections: Vec<Connection<'a>>,
    }

    impl<'a, T: Point> Node<'a, T> {
        pub fn is_connected_to(&self, point_id: &str) -> bool {
            self.connections.iter().any(|conn| conn.to == point_id)
        }
    }
}

#[cfg(test)]
mod test {
    use node::*;

    struct Country {
        pub name: String
    }

    impl Point for Country {
        fn id(&self) -> &str {
            &self.name
        }
    }

    const PORTUGAL: &str = "Portugal";
    const SPAIN: &str = "Spain";
    const ICELAND: &str = "Iceland";
    const AUSTRIA: &str = "Austria";

    #[test]
    fn it_should_return_false_if_no_node_is_connected() {
        let iceland = get_country(ICELAND);

        let iceland_node = Node {
            point: iceland,
            connections: Vec::new(),
        };

        assert_eq!(iceland_node.is_connected_to(AUSTRIA), false);
    }

    #[test]
    fn it_should_return_true_if_is_connected_to_node() {
        let portugal = get_country(PORTUGAL);
        let spain = get_country(SPAIN);

        let portugal_node = Node {
            point: portugal,
            connections: vec![Connection {
                to: spain.id()
            }],
        };

        assert_eq!(portugal_node.is_connected_to(SPAIN), true);
    }

    fn get_country(name: &str) -> Country {
        Country {
            name: String::from(name)
        }
    }
}
