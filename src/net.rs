use node::Node;
use node::Point;

pub struct Net<T: Point> {
    pub nodes: Vec<Node<T>>
}

impl<'a, T: Point> Net<T> {
    pub fn find_paths(&self, from: &'a T, to: &'a T) -> Result<Vec<Vec<&'a T>>, NetErrors> {
        self.find_node_or_throws(to)?;
        let node_from = self.find_node_or_throws(from)?;

        let mut paths = Vec::new();
        if node_from.is_connected_to(to){
            let path = vec!(from, to);
            paths.push(path);
        }

        Ok(paths)
    }

    fn find_node_or_throws(&self, point: &T) -> Result<&Node<T>, NetErrors> {
        let node_point = self.nodes.iter().find(|node| node.point.is(point));
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

        let paths = a_b_net.find_paths(&point_a, &point_b)
            .expect("Unexpected error while finding path");

        assert_eq!(paths.len(), 1, "Should find one path");

        let path_from_a_to_b = &paths[0];

        let first_point = &path_from_a_to_b[0];

        let last_point = &path_from_a_to_b[1];

        assert!(first_point.is(&point_a), "Path should begin with A point");
        assert!(&last_point.is(&point_b), "Path should begin with A point");
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
}