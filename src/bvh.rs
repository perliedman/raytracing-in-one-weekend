extern crate rand;
use rand::Rng;
use std::cmp::Ordering;

use ::aabb::{Aabb, surrounding_box};
use ::ray::Ray;
use ::material::HitRecord;
use ::hitable::Hitable;

#[derive(Debug)]
pub struct BvhTree<'a> {
  nodes: Vec<BvhNode<'a>>,
  root: NodeId
}

#[derive(Debug)]
struct BvhNode<'a> {
  left: Option<NodeId>,
  right: Option<NodeId>,
  aabb: Option<Aabb>,
  hitable: Option<&'a Box<Hitable>>
}

#[derive(Copy, Clone, Debug)]
pub struct NodeId {
  index: usize
}

impl<'a> BvhTree<'a> {
  fn hit(&self, id: NodeId, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let node = &self.nodes[id.index];

    if node.aabb.is_none() || node.aabb.is_some() && node.aabb.unwrap().hit(r, tmin, tmax) {
      match node.hitable {
        Some(ref hitable) => return hitable.hit(r, tmin, tmax),
        None => { }
      }

      let mut hit_left: Option<HitRecord> = None;
      let mut hit_right: Option<HitRecord> = None;

      if let Some(ref left_index) = node.left {
        hit_left = self.hit(*left_index, r, tmin, tmax);
      }

      if let Some(ref right_index) = node.right {
        hit_right = self.hit(*right_index, r, tmin, tmax);
      }

      match hit_left {
        Some(left) => {
          match hit_right {
            Some(right) => if left.t < right.t { return hit_left; } else { return hit_right; },
            None => return hit_left
          }
        },
        None => {}
      }

      match hit_right {
        Some(_right) => return hit_right,
        None => {}
      }
    }

    None
  }
}

impl<'a> Hitable for BvhTree<'a> {
  fn bounding_box(&self) -> Option<Aabb> {
    self.nodes[self.root.index].aabb
  }

  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    self.hit(self.root, r, tmin, tmax)
  }
}

impl<'a> BvhTree<'a> {
  pub fn new(l: &'a mut [Box<Hitable>]) -> BvhTree<'a> {
    let mut tree = BvhTree { nodes: Vec::new(), root: NodeId { index: 0 } };
    tree.root = tree.build(l);

    tree
  }

  fn build(&mut self, l: &'a mut [Box<Hitable>]) -> NodeId {
    let axis = rand::thread_rng().gen_range::<i32>(0, 3);

    match axis {
      0 => l.sort_by(|a, b| box_x_compare(a, b)),
      1 => l.sort_by(|a, b| box_y_compare(a, b)),
      2 => l.sort_by(|a, b| box_z_compare(a, b)),
      _ => panic!("Unexpected axis")
    }

    let left: NodeId;
    let right: NodeId;

    if l.len() == 1 {
      return self.new_leaf(&l[0]);
    } else if l.len() == 2 {
      left = self.new_leaf(&l[0]);
      right = self.new_leaf(&l[1]);
    } else {
      let half_len = l.len() / 2;
      let (left_hitables, right_hitables) = l.split_at_mut(half_len);

      left = self.build(left_hitables);
      right = self.build(right_hitables);
    }

    if let Some(left_box) = self.nodes[left.index].aabb {
      if let Some(right_box) = self.nodes[right.index].aabb {
        return self.new_node(surrounding_box(&left_box, &right_box), Some(left), Some(right))
      }
    }

    panic!("No bounding box in BvhNode::build");
  }

  fn new_leaf(&mut self, hitable: &'a Box<Hitable>) -> NodeId {
    let next_index = self.nodes.len();

    self.nodes.push(BvhNode {
      left: None,
      right: None,
      aabb: hitable.bounding_box(),
      hitable: Some(hitable)
    });

    return NodeId { index: next_index };
  }

  fn new_node(&mut self, aabb: Aabb, left: Option<NodeId>, right: Option<NodeId>) -> NodeId {
    let next_index = self.nodes.len();

    self.nodes.push(BvhNode {
      left,
      right,
      aabb: Some(aabb),
      hitable: None
    });

    return NodeId { index: next_index };
  }
}

fn box_x_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
  if let Some(box_left) = a.bounding_box() {
    if let Some(box_right) = b.bounding_box() {
      if let Some(cmp) = box_left.min.x().partial_cmp(&box_right.min.x()) {
        return cmp;
      } else {
        panic!("Can't compare");
      }
    }
  }

  panic!("No bounding box in BvhNode::new");
}

fn box_y_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
  if let Some(box_left) = a.bounding_box() {
    if let Some(box_right) = b.bounding_box() {
      if let Some(cmp) = box_left.min.y().partial_cmp(&box_right.min.y()) {
        return cmp;
      } else {
        panic!("Can't compare");
      }
    }
  }

  panic!("No bounding box in BvhNode::new");
}

fn box_z_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
  if let Some(box_left) = a.bounding_box() {
    if let Some(box_right) = b.bounding_box() {
      if let Some(cmp) = box_left.min.z().partial_cmp(&box_right.min.z()) {
        return cmp;
      } else {
        panic!("Can't compare");
      }
    }
  }

  panic!("No bounding box in BvhNode::new");
}
