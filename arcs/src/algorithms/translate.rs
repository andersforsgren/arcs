use crate::{
    algorithms::AffineTransformable,
    components::{BoundingBox, Viewport},
    Arc, Transform, Vector,
};

/// Something which can be moved around "rigidly" in *Drawing Space*.
pub trait Translate {
    fn translate(&mut self, displacement: Vector);

    fn translated(&self, displacement: Vector) -> Self
    where
        Self: Sized + Clone,
    {
        let mut clone = self.clone();
        clone.translate(displacement);

        clone
    }
}

impl<A: AffineTransformable> Translate for A {
    fn translate(&mut self, displacement: Vector) {
        self.transform(Transform::create_translation(
            displacement.x,
            displacement.y,
        ));
    }
}

impl Translate for Arc {
    fn translate(&mut self, displacement: Vector) {
        *self = Arc::from_centre_radius(
            self.centre().translated(displacement),
            self.radius(),
            self.start_angle(),
            self.sweep_angle(),
        );
    }
}

impl Translate for BoundingBox {
    fn translate(&mut self, displacement: Vector) {
        *self = BoundingBox::new_unchecked(
            self.bottom_left().translated(displacement),
            self.top_right().translated(displacement),
        );
    }
}

impl Translate for Viewport {
    fn translate(&mut self, displacement: Vector) {
        self.centre.translate(displacement);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point;

    #[test]
    fn translate_point() {
        let original = Point::new(3.0, 4.0);
        let delta = Vector::new(-5.0, 2.5);

        let got = original.translated(delta);

        assert_eq!(got, original + delta);
    }

    #[test]
    fn translate_bounding_box() {
        let first = Point::new(-2.0, 1.5);
        let second = Point::new(4.0, 3.7);
        let displacement = Vector::new(1.0, -1.0);
        let original = BoundingBox::new(first, second);

        let expected = BoundingBox::new(
            Point::new(-2.0 + 1.0, 1.5 + -1.0),
            Point::new(4.0 + 1.0, 3.7 + -1.0),
        );
        let actual = original.translated(displacement);

        assert_eq!(actual, expected);
    }
}
