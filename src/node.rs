use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    pub point: Point,
    pub children: HashMap<Direction, (u32, u32)>,
}

impl Node {
    pub(crate) fn at(x: u32, y: u32) -> Self {
        Self {
            point: Point::at(x, y),
            children: HashMap::new(),
        }
    }
}

#[derive(Debug, Default, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    fn at(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

#[cfg(test)]
mod test {

    // #[test]
    // fn iteration() {
    //     // xxoxx
    //     // xooox
    //     // xoxxx
    //     // xooox
    //     // xxxox
    //     let maze = vec![
    //         vec![255, 0, 0, 0, 255],
    //         vec![255, 0, 255, 255, 255],
    //         vec![255, 0, 255, 255, 255],
    //         vec![255, 0, 0, 0, 255],
    //         vec![255, 255, 255, 0, 255],
    //     ];

    //     let root = Node::new(Point::new(0, 2));

    //     for (x, row) in maze.iter().enumerate() {
    //         let mut prev = false;
    //         let mut cur = false;
    //         let mut next = row
    //             .get(x + 1)
    //             .map_or(false, |n| if *n == 255 { true } else { false });
    //         let mut left_node = None;
    //         for (y, value) in row.iter().enumerate() {
    //             let mut prev = cur;
    //             let mut cur = next;
    //             let mut next = row
    //                 .get(y + 1)
    //                 .map_or(false, |n| if *n == 255 { true } else { false });

    //             let above = maze
    //                 .get(x - 1)
    //                 .map_or(false, |row| if row[y] == 255 { true } else { false });
    //             let below = maze
    //                 .get(x + 1)
    //                 .map_or(false, |row| if row[y] == 255 { true } else { false });

    //             let mut n = None;

    //             if cur == false {
    //                 continue;
    //             }

    //             // if prev is not a wall
    //             if prev == true {
    //                 if next == true {
    //                     // PATH PATH PATH
    //                     // // only create if path above or below
    //                     if above || below {
    //                         let node = Rc::new(RefCell::new(Node::new(Point::new(x, y))));
    //                         n = Some(node);
    //                     }
    //                 } else {
    //                     // PATH PATH WALL
    //                     // create path at end of corridor
    //                     let mut node = Node::new(Point::new(x, y));
    //                     node.west = left_node;

    //                     left_node.unwrap().as_ref().borrow_mut().east =
    //                         Some(Rc::new(RefCell::new(node)));
    //                     left_node = None;
    //                 }
    //             } else if next == true {
    //                 // WALL PATH PATH
    //                 // create node at start of corridore
    //                 let node = Rc::new(RefCell::new(Node::new(Point::new(x, y))));
    //                 n = Some(node.clone());
    //                 left_node = Some(node.clone())
    //             } else {
    //                 // WALL PATH WALL
    //                 // Only create node in dead end
    //                 if !above || !below {
    //                     n = Some(Rc::new(RefCell::new(Node::new(Point::new(x, y)))));
    //                 }
    //             }
    //         }
    //     }
    // }
}
