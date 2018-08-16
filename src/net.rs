use node::Node;
use node::Point;
use path::PathBuilder;
use path::Path;

#[derive(Debug)]
pub struct Net<T: Point> {
    pub nodes: Vec<Node<T>>
}

impl<'a, T: Point> Net<T> {
    pub fn find_paths(&self, from: &'a T, to: &'a T) -> Result<Vec<Path<T>>, NetErrors> {
        let node_from = self.find_node_or_throws(from)?;

        match PathBuilder::new().point(from).build() {
            Ok(beginning_path) => self.find_paths_not_crossing_previous_path(&node_from, &to, &beginning_path),
            Err(message) => Err(NetErrors::PathCannotBeBuilt(message))
        }
    }

    fn find_paths_not_crossing_previous_path(&self, from: &Node<T>, to: &T, previous_path: &Path<T>) -> Result<Vec<Path<T>>, NetErrors> {
        if previous_path.ends_with(to) {
            let current_path = previous_path.clone();
            return Ok(vec![current_path]);
        }

        match from.connected_points_not_in_path(previous_path) {
            None => Err(NetErrors::NoPathFound),
            Some(followable_points) => {
                let mut paths: Vec<Path<T>> = Vec::new();
                followable_points.into_iter().for_each(|point|
                    self.push_all_paths_following(&mut paths, point, &to, previous_path)
                );

                if paths.is_empty() {
                    Err(NetErrors::NoPathFound)
                } else {
                    Ok(paths)
                }
            }
        }
    }

    fn push_all_paths_following(&self, paths: &mut Vec<Path<T>>, starting_point: &T, destination_point: &&T, previous_path: &Path<T>) -> () {
        let origin_node = self.find_node_or_panic(starting_point);
        let trying_path = previous_path.with_point_at_the_end(starting_point);
        let path_search = self.find_paths_not_crossing_previous_path(origin_node, &destination_point, &trying_path);
        match path_search {
            Ok(paths_found) => paths_found.into_iter().for_each(|path_found| paths.push(path_found)),
            Err(err) => Self::panic_if_error_is_not_path_not_found(err)
        }
    }

    fn panic_if_error_is_not_path_not_found(err: NetErrors) -> () {
        match err {
            NetErrors::NoPathFound => (),
            _ => panic!(err)
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

    fn find_node_or_panic(&self, point: &T) -> &Node<T> {
        match self.find_node_or_throws(point) {
            Ok(ref node) => node,
            Err(err) => panic!(err)
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
        PathCannotBeBuilt(path_error: String) {
            description("Path cannot be built")
            display(r#"Path cannot be built: {}"#, path_error)
        }
    }
}


#[cfg(test)]
mod test {
    use net::*;
    use node::Point;
    use node::Node;
    use node::Connection;
    use path::Path;

    const A: char = 'A';
    const B: char = 'B';
    const C: char = 'C';
    const D: char = 'D';

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

        let paths = a_b_net.find_paths(&point_a, &point_b)
            .expect("Unexpected error while finding path");

        assert_eq!(format_list_of_paths(paths), "A-B", "Found path should be A-B");
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
                    _ => panic!("NoPathFound exception expected")
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

        let paths = a_b_c_net.find_paths(&point_a, &point_c)
            .expect(&format!("should not throw exception finding path a to c in net {:?}", a_b_c_net).into_boxed_str());

        assert_eq!("A-B-C", format_list_of_paths(paths), "found path should be A-B-C");
    }

    // Given this net of points:
    // A - B - C
    //  \     /
    //   \   /
    //     D
    #[test]
    fn in_triangle_net_should_find_two_paths_from_a_to_c() {
        let point_a = simple_point(A);
        let point_b = simple_point(B);
        let point_c = simple_point(C);
        let point_d = simple_point(D);

        let node_a = node_connected_to(point_a, vec![point_b, point_d]);
        let node_b = node_connected_to(point_b, vec![point_a, point_c]);
        let node_c = node_connected_to(point_c, vec![point_b, point_d]);
        let node_d = node_connected_to(point_d, vec![point_a, point_c]);

        let triangle_net: Net<SimplePoint> = Net {
            nodes: vec![node_a, node_b, node_c, node_d]
        };

        let paths = triangle_net.find_paths(&point_a, &point_c)
            .expect(&format!("should not throw exception finding path a to c in net {:?}", triangle_net).into_boxed_str());

        let formatted_paths = format_list_of_paths(paths);

        assert_eq!(formatted_paths, "A-B-C + A-D-C", "should find A-B-C and A-D-C paths");
    }

    // Given this net of points:
    // A - B - C
    //  \  |  /
    //   \ | /
    //     D
    #[test]
    fn should_find_all_feasible_paths_from_a_to_c() {
        let point_a = simple_point(A);
        let point_b = simple_point(B);
        let point_c = simple_point(C);
        let point_d = simple_point(D);

        let node_a = node_connected_to(point_a, vec![point_b, point_d]);
        let node_b = node_connected_to(point_b, vec![point_a, point_c, point_d]);
        let node_c = node_connected_to(point_c, vec![point_b, point_d]);
        let node_d = node_connected_to(point_d, vec![point_a, point_c, point_b]);

        let triangle_net: Net<SimplePoint> = Net {
            nodes: vec![node_a, node_b, node_c, node_d]
        };

        let paths = triangle_net.find_paths(&point_a, &point_c)
            .expect(&format!("should not throw exception finding path a to c in net {:?}", triangle_net).into_boxed_str());

        let formatted_paths = format_list_of_paths(paths);

        assert_eq!(formatted_paths, "A-B-C + A-B-D-C + A-D-B-C + A-D-C", "should find the four feasible paths");
    }


    fn format_path_kebab(path: &Path<SimplePoint>) -> String {
        return format!("{}", path);
    }

    fn format_list_of_paths(paths: Vec<Path<SimplePoint>>) -> String {
        let mut formatted_and_ordered_paths: Vec<String> = paths.iter()
            .map(|path| format_path_kebab(path))
            .collect();

        formatted_and_ordered_paths.sort();

        formatted_and_ordered_paths[..].join(" + ")
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