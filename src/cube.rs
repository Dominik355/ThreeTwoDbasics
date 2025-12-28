// every negative 'z' means the object is "behind us"
pub const VERTICES: [(f32, f32, f32); 8] = [
    (1., 1., -1.),
    (-1., 1., -1.),
    (-1., -1., -1.),
    (1., -1., -1.),
    (1., 1., 1.),
    (-1., 1., 1.),
    (-1., -1., 1.),
    (1., -1., 1.),
];

// lines between vertices (indexing vertices array)
pub const LINE_INDEXES: [[usize; 4]; 4] = [[0, 1, 2, 3], [4, 5, 6, 7], [0, 4, 1, 5], [2, 6, 3, 7]];
