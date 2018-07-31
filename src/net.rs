use node::Node;
use node::Point;
use node::Connection;

#[derive(Debug)]
pub struct Net<T: Point> {
    pub nodes: Vec<Node<T>>
}

impl<'a, T: Point> Net<T> {
    pub fn find_paths(&self, from: &'a T, to: &'a T) -> Result<Vec<Vec<T>>, NetErrors> {
        let node_from = self.find_node_or_throws(from)?;

        let beginning_path = vec![from.clone()];
        self.find_paths_rec(&node_from, &to, &beginning_path)
    }

    fn find_paths_rec(&self, from: &Node<T>, to: &T, previous_path: &Vec<T>) -> Result<Vec<Vec<T>>, NetErrors> {
        let mut current_path = previous_path.to_vec();

        if from.is_connected_to(to) {
            let next_point= to.clone();
            current_path.push(next_point);
            return Ok(vec![current_path]);
        }

        let connection_not_used_in_previous_path = |connection: &&Connection<T>|
            !previous_path.iter().any(|point| point.is(&connection.to));

        let followable_points: Vec<&T> = from.connections.iter()
            .filter(connection_not_used_in_previous_path)
            .map(|c| &c.to)
            .collect();

        if followable_points.is_empty() {
            return Err(NetErrors::NoPathFound);
        }

        let mut paths: Vec<Vec<T>> = Vec::new();
        for point in followable_points.iter() {
            let origin_node = self.find_node_or_throws(&point)?;
            let mut trying_path = current_path.to_vec();
            trying_path.push(point.clone().clone());

            let path_search = self.find_paths_rec(origin_node, &to, &trying_path);
            match path_search {
                Ok(paths_found) => paths_found.iter()
                    .for_each(|path_found| paths.push(path_found.to_vec())),
                Err(err) => {
                    match err {
                        NetErrors::NoPathFound => (),
                        _ => panic!(err)
                    }
                }
            }
        }

        if paths.is_empty() {
            Err(NetErrors::NoPathFound)
        } else {
            Ok(paths)
        }
    }

    fn find_node_or_throws(&self, point: &T) -> Result<&Node<T>, NetErrors> {
        let node_point = self.nodes.iter()
            .find(|node| node.point.is(point));

        match node_point {
            Some(ref node) => Ok(node),
            None => Err(NetErrors::PointNotFound(point.id().to_string()))
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum NetErrors {
        PointNotFound(point_id: String) {
            description("Point does not exists in the net")
            display(r#"The point with id "{}" could not be found"#, point_id)
        }
        NoPathFound {
            description("No path found between points")
            display(r#"No path found between points"#)
        }
    }
}


#[cfg(test)]
mod test {
    use net::*;
    use node::Point;
    use node::Node;
    use node::Connection;

    const A: char = 'A';
    const B: char = 'B';
    const C: char = 'C';

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct SimplePoint {
        name: char
    }

    impl Point for SimplePoint {
        type Identifier = char;

        fn id(&self) -> char {
            self.name
        }
    }

    // Given this net:
    // A - B
    #[test]
    fn find_paths_from_a_point_not_in_the_net_should_throw_an_exception() {
        let point_a = simple_point(A);
        let point_b = simple_point(B);
        let point_c = simple_point(C);

        let node_a = node(point_a, point_b);
        let node_b = node(point_b, point_a);

        let a_b_net: Net<SimplePoint> = Net {
            nodes: vec![node_a, node_b]
        };

        let paths = a_b_net.find_paths(&point_c, &point_a);

        assert!(paths.is_err(), "Should not be able to find the path from a point that does not exists in the net");
    }

    // Given this net:
    // A - B
    #[test]
    fn find_paths_to_a_point_not_in_the_net_should_throw_an_exception() {
        let point_a = simple_point(A);
        let point_b = simple_point(B);
        let point_c = simple_point(C);

        let node_a = node(point_a, point_b);
        let node_b = node(point_b, point_a);

        let a_b_net: Net<SimplePoint> = Net {
            nodes: vec![node_a, node_b]
        };

        let paths = a_b_net.find_paths(&point_a, &point_c);

        assert!(paths.is_err(), "Should not be able to find the path to a point that does not exists in the net");
    }

    // Given this net:
    // A - B
    #[test]
    fn in_a_b_net_find_path_should_find_a_path_from_a_to_b() {
        let point_a = simple_point(A);
        let point_b = simple_point(B);

        let node_a = node(point_a, point_b);
        let node_b = node(point_b, point_a);

        let a_b_net: Net<SimplePoint> = Net {
            nodes: vec![node_a, node_b]
        };

        let mut paths = a_b_net.find_paths(&point_a, &point_b)
            .expect("Unexpected error while finding path");

        assert_eq!(paths.len(), 1, "Should find one path");

        let path_a_b = paths.pop().unwrap();

        assert_eq!(format_path_kebab(&path_a_b), "A-B", "Found path should be A-B");
    }

    // Given this net of non connected points:
    // A  B
    #[test]
    fn in_there_is_no_path_from_a_to_b_find_path_should_throw() {
        let point_a = simple_point(A);
        let point_b = simple_point(B);

        let node_a = non_connected_node(point_a);
        let node_b = non_connected_node(point_b);

        let a_b_net: Net<SimplePoint> = Net {
            nodes: vec![node_a, node_b]
        };

        let paths = a_b_net.find_paths(&point_a, &point_b);

        match paths {
            Ok(_) => panic!("should throw an error"),
            Err(ref err) => {
                match err {
                    NetErrors::NoPathFound => assert!(true),
                    _ => panic!("NoPathFound execption expected")
                }
            }
        }
    }

    // Given this net of points:
    // A - B - C
    #[test]
    fn in_an_a_b_c_net_should_find_a_path_from_a_to_c() {
        let point_a = simple_point(A);
        let point_b = simple_point(B);
        let point_c = simple_point(C);

        let node_a = node(point_a, point_b);
        let node_b = node_connected_to(point_b, vec![point_a, point_c]);
        let node_c = node(point_c, point_b);

        let a_b_c_net: Net<SimplePoint> = Net {
            nodes: vec![node_a, node_b, node_c]
        };

        let mut paths = a_b_c_net.find_paths(&point_a, &point_c)
            .expect(&format!("should not throw exception finding path a to c in net {:?}", a_b_c_net).into_boxed_str());

        assert_eq!(paths.len(), 1, "should find one path");

        let path_a_b_c = paths.pop().unwrap();

        assert_eq!("A-B-C", format_path_kebab(&path_a_b_c), "found path should be A-B-C");
    }

    fn format_path_kebab(path: &Vec<SimplePoint>) -> String {
        let points: Vec<String> =path.iter()
            .map( |point| point.id().to_string())
            .collect();

        points[..].join("-")
    }

    fn simple_point(name: char) -> SimplePoint {
        SimplePoint { name: name.clone() }
    }

    fn node(from: SimplePoint, to: SimplePoint) -> Node<SimplePoint> {
        Node {
            point: from.clone(),
            connections: vec![Connection { to: to.clone() }],
        }
    }

    fn node_connected_to(point: SimplePoint, point_connected: Vec<SimplePoint>) -> Node<SimplePoint> {
        let connections = point_connected.iter()
            .map(|point| Connection { to: point.clone() })
            .collect();
        Node { point, connections }
    }

    fn non_connected_node(point: SimplePoint) -> Node<SimplePoint> {
        let connections = Vec::new();
        Node { point, connections }
    }
}