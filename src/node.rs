pub trait Point {
    type Identifier: PartialEq;

    fn id(&self) -> Self::Identifier;
}

pub struct Connection<T: Point> {
    pub to: T
}

impl<T: Point> Connection<T> {
    pub fn is_connected_to(&self, point: &T) -> bool {
        self.to.id() == point.id()
    }
}

pub struct Node<T: Point> {
    pub point: T,
    pub connections: Vec<Connection<T>>,
}

impl<T: Point> Node<T> {
    pub fn is_connected_to(&self, point: &T) -> bool{
        self.connections.iter()
            .any(|conn| conn.is_connected_to(point))
    }
}

#[cfg(test)]
mod test {
    use node::*;

    #[derive(Clone)]
    struct Country {
        pub name: String
    }

    impl Point for Country {
        type Identifier = String;

        fn id(&self) -> String {
            self.name.to_string()
        }
    }

    const PORTUGAL: &str = "Portugal";
    const SPAIN: &str = "Spain";
    const ICELAND: &str = "Iceland";
    const AUSTRIA: &str = "Austria";

    #[test]
    fn it_should_return_false_if_no_node_is_connected() {
        let iceland = get_country(ICELAND);
        let austria = get_country(AUSTRIA);

        let iceland_node = Node {
            point: iceland,
            connections: Vec::new(),
        };

        assert_eq!(iceland_node.is_connected_to(&austria), false);
    }

    #[test]
    fn it_should_return_true_if_is_connected_to_node() {
        let portugal = get_country(PORTUGAL);
        let spain = get_country(SPAIN);

        let portugal_node = Node {
            point: portugal,
            connections: vec![Connection {
                to: spain.clone()
            }],
        };

        assert_eq!(portugal_node.is_connected_to(&spain), true);
    }

    fn get_country(name: &str) -> Country {
        Country {
            name: String::from(name)
        }
    }
}
