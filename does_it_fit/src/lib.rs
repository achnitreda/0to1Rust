mod areas_volumes;
use areas_volumes::{square_area, triangle_area, circle_area, rectangle_area, parallelepiped_volume, cone_volume, cube_volume, sphere_volume, triangular_pyramid_volume};
pub type GeometricalVolumes = areas_volumes::GeometricalVolumes;
pub type GeometricalShapes = areas_volumes::GeometricalShapes;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool { 
    let rectangular_area = rectangle_area(x, y);
    let shape_area = match kind {
        GeometricalShapes::Square => square_area(a),
        GeometricalShapes::Triangle => triangle_area(a, b) as usize,
        GeometricalShapes::Circle => circle_area(a) as usize,
        GeometricalShapes::Rectangle => rectangle_area(a, b),
    };
    
    if shape_area*times <= rectangular_area {
        return true
    }
    false
}


pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let para_volume = parallelepiped_volume(x, y, z);
    let shape_area = match kind {
        GeometricalVolumes::Cube => cube_volume(a),
        GeometricalVolumes::Sphere => sphere_volume(a) as usize,
        GeometricalVolumes::Cone => cone_volume(a, b) as usize,
        GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(a as f64,b) as usize,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a,b,c),
    };

    if shape_area*times <= para_volume {
        return true
    }
    false
}