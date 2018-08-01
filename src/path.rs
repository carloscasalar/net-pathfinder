use node::Point;

#[derive(Debug)]
pub struct Path<T: Point> {
    points: Vec<T>
}

impl<T: Point> Path<T> {}

pub struct PathBuilder<T: Point> {
    points: Option<Vec<T>>
}

impl<T: Point> PathBuilder<T> {
    pub fn new() -> PathBuilder<T> {
        let points = None;
        PathBuilder { points }
    }

    pub fn points(&mut self, points: Vec<T>) -> &mut Self {
        match self.points {
            Some(_) => self.points = Some(points),
            None => self.points = Some(points)
        }
        self
    }

    pub fn point(&mut self, point: T) -> &mut Self {
        match self.points {
            Some(ref mut p) => p.push(point),
            None => self.points = Some(vec![point])
        }
        self
    }

    pub fn build(&self) -> Result<Path<T>, String> {
        let points = Clone::clone(
            self.points
                .as_ref()
                .ok_or("Should set at least one point for the path")?
        );
        Ok(Path { points })
    }
}

#[cfg(test)]
mod test {
    use path::PathBuilder;
    use node::Point;
    use path::Path;

    #[derive(Copy, Clone, PartialEq, Debug)]
    struct SimplePoint {
        id: u8
    }

    impl Point for SimplePoint {
        type Identifier = u8;

        fn id(&self) -> u8 {
            self.id
        }
    }

    impl SimplePoint {
        pub fn new(id: u8) -> SimplePoint {
            SimplePoint { id }
        }
    }

    #[test]
    fn builder_should_be_able_to_build_a_path_setting_points_one_by_one() {
        let path = PathBuilder::new()
            .point(SimplePoint::new(8))
            .point(SimplePoint::new(5))
            .build()
            .expect("Builder should not throw if all attributes are provided");

        assert_eq!(format_path_with_dashes_between_ids(path), "8-5", "Should build a path with all points in order");
    }

    #[test]
    fn builder_should_be_able_to_build_a_path_setting_points_as_vec() {
        let path = PathBuilder::new()
            .points(vec![SimplePoint::new(8), SimplePoint::new(5)])
            .build()
            .expect("Builder should not throw if all attributes are provided");

        assert_eq!(format_path_with_dashes_between_ids(path), "8-5", "Should build a path with all points in order");
    }

    #[test]
    fn builder_should_thrown_if_no_points_are_provided() {
        let builder: PathBuilder<SimplePoint> = PathBuilder::new();
        assert!(builder.build().is_err(), "Should throw an error if no point is provided");
    }

    fn format_path_with_dashes_between_ids(path: Path<SimplePoint>) -> String {
        let ids_as_string: Vec<String> = path.points
            .iter()
            .map(|point| format!("{}", point.id))
            .collect();

        ids_as_string[..].join("-")
    }
}