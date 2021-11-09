use collisions;
use collisions::Disk;
use draw;
use draw::render;
use draw::SvgRenderer;

use itertools::Itertools;

fn main() {
    let mut canvas = Canvas::new(100, 100);


    let disks = vec![
        Disk::new(),
        Disk::new(),
        Disk::new(),
        Disk::new(),
        Disk::new(),
        Disk::new(),
        Disk::new(),
        Disk::new(),
        Disk::new(),
    ];

    for disk in disks{
        disk.draw(&mut canvas);
    }
render::save(
    &canvas,
    "tests/svg/basic_end_to_end.svg",
    SvgRenderer::new(),
)
.expect("Failed to save")

    let mut it = disks
        .iter()
        .enumerate()
        .tuple_combinations()
        .filter(|((_, x), (_, y))| collisions::intersect(&x, &y))
        .map(|((i, _), (j, _))| (i, j));

    while let Some(x) = it.next() {
        println!("{:?}", x);
    }
}
