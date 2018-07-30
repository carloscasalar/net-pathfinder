use node::Node;
use node::Point;

pub struct Net<T: Point> {
    pub nodes: Vec<Node<T>>
}

impl<T: Point> Net<T> {

    fn assert_exists(&self, point: &T)->Result<(),NetErrors>{
       match self.nodes.iter().find(|p| p.point.is(point)) {
           Some(_) => Ok(()),
           None => Err(NetErrors::PointNotFound(point.id().to_string()))
       }
    }

    pub fn find_paths(&self, from: &T, to: &T) -> Result<Vec<Vec<T>>, NetErrors> {
        self.assert_exists(from)?;
        self.assert_exists(to)?;

        Ok(Vec::new())
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

    #[derive(Copy,Clone)]
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

        let path = a_b_net.find_paths(&point_c, &point_a);

        assert!(path.is_err(), "Should not be able to find the path from a point that does not exists in the net");
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

        let path = a_b_net.find_paths(&point_a, &point_c);

        assert!(path.is_err(), "Should not be able to find the path to a point that does not exists in the net");
    }

    fn simple_point(name: char) -> SimplePoint {
        SimplePoint { name: name.clone() }
    }

    fn node(from: SimplePoint, to: SimplePoint) -> Node<SimplePoint>{
        Node {
            point: from.clone(),
            connections: vec![Connection{ to: to.clone()}]
        }
    }
}