use std::cmp::Ordering;

pub struct Node{
    pub data: char,
    pub cost: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node{
    pub fn new(data: char, cost: i32) -> Self{
        Node {
            data,
            cost,
            left: None,
            right: None
        }
        
    }
}

impl PartialEq for Node{
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for Node{
    
}

impl PartialOrd for Node{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering{
        if self.cost > other.cost{
            return Ordering::Less;
        } else if self.cost < other.cost {
            return  Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

