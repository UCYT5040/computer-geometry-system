use crate::tree::{EquationTree, TreeItem};

pub fn add_sample_data(tree: &mut EquationTree) {
    let circle_id = tree.add_child(tree.root, TreeItem::new_category_with_name("Circle"));
    let polygon_id = tree.add_child(tree.root, TreeItem::new_category_with_name("Polygon"));
    let rectangle_id = tree.add_child(tree.root, TreeItem::new_category_with_name("Rectangle"));
    let square_id = tree.add_child(tree.root, TreeItem::new_category_with_name("Square"));
    let triangle_id = tree.add_child(tree.root, TreeItem::new_category_with_name("Triangle"));
    let ellipse_id = tree.add_child(tree.root, TreeItem::new_category_with_name("Ellipse"));

    // Add children to Circle
    let _radius_id = tree.add_child(circle_id, TreeItem::new_equation_with_name("Radius"));
    let _area_id = tree.add_child(circle_id, TreeItem::new_equation_with_name("Area"));
    let _circumference_id =
        tree.add_child(circle_id, TreeItem::new_equation_with_name("Circumference"));

    // Add children to Polygon
    let _sides_id = tree.add_child(
        polygon_id,
        TreeItem::new_equation_with_name("Number of Sides"),
    );
    let _perimeter_id = tree.add_child(polygon_id, TreeItem::new_equation_with_name("Perimeter"));

    // Add children to Rectangle
    let _width_id = tree.add_child(rectangle_id, TreeItem::new_equation_with_name("Width"));
    let _height_id = tree.add_child(rectangle_id, TreeItem::new_equation_with_name("Height"));
    let _rect_area_id = tree.add_child(rectangle_id, TreeItem::new_equation_with_name("Area"));

    // Add children to Square
    let _side_id = tree.add_child(square_id, TreeItem::new_equation_with_name("Side Length"));
    let _sq_area_id = tree.add_child(square_id, TreeItem::new_equation_with_name("Area"));

    // Add children to Triangle
    let _base_id = tree.add_child(triangle_id, TreeItem::new_equation_with_name("Base"));
    let _height_tri_id = tree.add_child(triangle_id, TreeItem::new_equation_with_name("Height"));
    let _tri_area_id = tree.add_child(triangle_id, TreeItem::new_equation_with_name("Area"));

    // Add children to Ellipse
    let _major_axis_id = tree.add_child(ellipse_id, TreeItem::new_equation_with_name("Major Axis"));
    let _minor_axis_id = tree.add_child(ellipse_id, TreeItem::new_equation_with_name("Minor Axis"));
    let _ellipse_area_id = tree.add_child(ellipse_id, TreeItem::new_equation_with_name("Area"));
}
