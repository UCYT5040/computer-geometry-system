use crate::tree::{EquationTree, TreeItem};

#[allow(unused)]
pub fn add_sample_data(tree: &mut EquationTree) {
    let circle_id = tree.add_child(tree.root, TreeItem::new_category_with_name("Circle"));
    let polygon_id = tree.add_child(tree.root, TreeItem::new_category_with_name("Polygon"));
    let rectangle_id = tree.add_child(tree.root, TreeItem::new_category_with_name("Rectangle"));
    let square_id = tree.add_child(tree.root, TreeItem::new_category_with_name("Square"));
    let triangle_id = tree.add_child(tree.root, TreeItem::new_category_with_name("Triangle"));
    let ellipse_id = tree.add_child(tree.root, TreeItem::new_category_with_name("Ellipse"));

    // Add children to Circle
    let _radius_id = tree.add_child(circle_id, TreeItem::new_equation_with_name("Radius", "r"));
    let _area_id = tree.add_child(circle_id, TreeItem::new_equation_with_name("Area", "A = pi * r^2"));
    let _circumference_id =
        tree.add_child(circle_id, TreeItem::new_equation_with_name("Circumference", "C = 2 * pi * r"));

    // Add children to Polygon
    let _angles_id = tree.add_child(polygon_id, TreeItem::new_equation_with_name("Exterior Angles", "a = 360 / n"));

    // Add children to Rectangle
    let _width_id = tree.add_child(rectangle_id, TreeItem::new_equation_with_name("Width", "w"));
    let _height_id = tree.add_child(rectangle_id, TreeItem::new_equation_with_name("Height", "h"));
    let _rect_area_id = tree.add_child(rectangle_id, TreeItem::new_equation_with_name("Area", "A = w * h"));

    // Add children to Square
    let _side_id = tree.add_child(square_id, TreeItem::new_equation_with_name("Side Length", "s"));
    let _sq_area_id = tree.add_child(square_id, TreeItem::new_equation_with_name("Area", "A = s^2"));

    // Add children to Triangle
    let _base_id = tree.add_child(triangle_id, TreeItem::new_equation_with_name("Base", "b"));
    let _height_tri_id = tree.add_child(triangle_id, TreeItem::new_equation_with_name("Height", "h"));
    let _tri_area_id = tree.add_child(triangle_id, TreeItem::new_equation_with_name("Area", "A = (b * h) / 2"));

    // Add children to Ellipse
    let _major_axis_id = tree.add_child(ellipse_id, TreeItem::new_equation_with_name("Major Axis", "a"));
    let _minor_axis_id = tree.add_child(ellipse_id, TreeItem::new_equation_with_name("Minor Axis", "b"));
    let _ellipse_area_id = tree.add_child(ellipse_id, TreeItem::new_equation_with_name("Area", "A = pi * a * b"));
}
