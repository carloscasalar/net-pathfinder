use path::Path;

pub trait Point: Clone {
    type Identifier: PartialEq + ToString;

    fn id(&self) -> Self::Identifier;

    fn is(&self, other_point: &Self) -> bool {
        &self.id() == &other_point.id()
    }
}

#[derive(Debug)]
struct Connection<T: Point> {
    pub to: T
}

impl<T: Point> Connection<T> {
    pub fn is_connected_to(&self, point: &T) -> bool {
        self.to.is(point)
    }
}

impl<T: Point> PartialEq for Connection<T> {
    fn eq(&self, other_connection: &Connection<T>) -> bool {
        self.to.is(&other_connection.to)
    }
}

#[derive(Debug)]
pub struct Node<T: Point> {
    point: T,
    connections: Vec<Connection<T>>,
}

impl<T: Point> Node<T> {
    pub fn point_is(&self, point: &T) -> bool {
        self.point.is(point)
    }

    pub fn is_connected_to(&self, point: &T) -> bool {
        self.connections.iter()
            .any(|conn| conn.is_connected_to(point))
    }

    pub fn connected_points_not_in_path(&self, path: &Path<T>) -> Option<Vec<&T>> {
        let points: Vec<&T> = self.connections.iter()
            .filter(|connection| path.do_not_contains(&connection.to))
            .map(|c| &c.to)
            .collect();

        if points.is_empty() {
            None
        } else {
            Some(points)
        }
    }
}

impl<T: Point> PartialEq for Node<T> {
    fn eq(&self, other_node: &Node<T>) -> bool {
        if !self.point.is(&other_node.point) {
            return false;
        }

        self.connections == other_node.connections
    }
}

#[derive(Debug)]
pub struct NodeBuilder<T: Point> {
    point: Option<T>,
    connected_points: Option<Vec<T>>,
}

impl<T: Point> NodeBuilder<T> {
    pub fn new() -> NodeBuilder<T> {
        let point = None;
        let connections = None;
        NodeBuilder { point, connected_points: connections }
    }

    pub fn point(&mut self, point: &T) -> &mut Self {
        let point_to_add = point.clone();
        self.point = Some(point_to_add);

        self
    }

    pub fn connected_point(&mut self, point: &T) -> &mut Self {
        if self.node_is_connected_to(point) {
            return self;
        }

        let point_connected = point.clone();
        match self.connected_points {
            Some(ref mut c) => c.push(point_connected),
            None => self.connected_points = Some(vec![point_connected])
        }

        self
    }

    pub fn connected_points(&mut self, connected_points: &Vec<T>) -> &mut Self {
        connected_points.iter()
            .for_each(|connected_to| {
                self.connected_point(connected_to);
            });

        self
    }

    pub fn build(&self) -> Result<Node<T>, String> {
        if self.point.is_none() {
            return Err(String::from("Should specify a point"));
        }

        if self.node_is_connected_to(self.point.as_ref().unwrap()) {
            return Err(String::from("Point cannot be connected to itself"));
        }

        let point = self.point
            .as_ref()
            .unwrap()
            .clone();

        let to_connection = |connected_point: &T| Connection {
            to: connected_point.clone()
        };

        let connections = self.connected_points
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .map(to_connection)
            .collect();

        Ok(Node {
            point,
            connections,
        })
    }

    fn node_is_connected_to(&self, point: &T) -> bool {
        match self.connected_points {
            None => false,
            Some(ref connections) => connections.iter()
                .any(|connected_point| connected_point.is(point))
        }
    }
}

#[cfg(test)]
mod test {
    use node::*;

    #[derive(Clone, Debug)]
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
    const FRANCE: &str = "France";
    const ICELAND: &str = "Iceland";
    const AUSTRIA: &str = "Austria";

    #[test]
    fn node_is_connected_should_return_false_if_no_node_is_connected() {
        let iceland = get_country(ICELAND);
        let austria = get_country(AUSTRIA);

        let iceland_node = Node {
            point: iceland,
            connections: Vec::new(),
        };

        assert_eq!(iceland_node.is_connected_to(&austria), false);
    }

    #[test]
    fn node_is_connected_should_return_true_if_is_connected_to_node() {
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

    #[test]
    fn two_nodes_of_the_same_point_with_same_connections_should_be_equal() {
        let portugal = get_country(PORTUGAL);
        let spain = get_country(SPAIN);

        let portugal_node = Node {
            point: portugal.clone(),
            connections: vec![Connection {
                to: spain.clone()
            }],
        };

        let other_portugal_node = Node {
            point: portugal.clone(),
            connections: vec![Connection {
                to: spain.clone()
            }],
        };

        assert_eq!(portugal_node, other_portugal_node);
    }

    #[test]
    fn builder_should_build_a_connected_node() {
        let portugal = get_country(PORTUGAL);
        let spain = get_country(SPAIN);

        let portugal_node = NodeBuilder::new()
            .point(&portugal)
            .connected_point(&spain)
            .build()
            .expect("should build portugal node");

        let expected_portugal_node = Node {
            point: portugal,
            connections: vec![Connection {
                to: spain.clone()
            }],
        };

        assert_eq!(portugal_node, expected_portugal_node);
    }

    #[test]
    fn builder_should_add_each_connection_only_once() {
        let portugal = get_country(PORTUGAL);
        let spain = get_country(SPAIN);
        let france = get_country(FRANCE);

        let spain_node = NodeBuilder::new()
            .point(&spain)
            .connected_point(&portugal)
            .connected_point(&portugal)
            .connected_point(&france)
            .build()
            .expect("should build portugal node");

        let expected_spain_node = Node {
            point: spain,
            connections: vec![
                Connection {
                    to: portugal.clone()
                },
                Connection {
                    to: france.clone()
                }
            ],
        };

        print!("pero qué me estás contando {:?}", spain_node);
        assert_eq!(spain_node, expected_spain_node, "Spain should be connected once to Portugal and France");
    }

    #[test]
    fn builder_should_fail_if_there_is_no_point() {
        let country_node_builder: NodeBuilder<Country> = NodeBuilder::new();

        assert_eq!(country_node_builder.build(), Err(String::from("Should specify a point")));
    }

    #[test]
    fn builder_should_fail_if_point_is_connected_to_itself() {
        let iceland = get_country(ICELAND);

        let mut builder = NodeBuilder::new();
        let builder = builder.point(&iceland);
        let builder = builder.connected_point(&iceland);

        assert_eq!(builder.build(), Err(String::from("Point cannot be connected to itself")));
    }

    fn get_country(name: &str) -> Country {
        Country {
            name: String::from(name)
        }
    }
}
