use std::cmp::Ordering;
use std::num::NonZeroUsize;

// ---------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Neighbor {
  pub id: u32,
  pub dist: f32,
}

// ---------------------------------------------------------------------------------------------------------------------------------

pub struct Queue {
  neighbors: Vec<Neighbor>,
  capacity: NonZeroUsize,
}

impl Queue {
  pub fn with_capacity( capacity: NonZeroUsize ) -> Self {
    let neighbors = Vec::with_capacity( capacity.get() );
    Self { neighbors, capacity }
  }

  pub fn as_slice( &self ) -> &[Neighbor] {
    &self.neighbors
  }
}

impl Queue {
  #[inline(never)]
  pub fn insert( &mut self, neighbor: Neighbor ) {
    // this compare function emits conditional jumps in opt-level=2
    // but conditional moves in opt-level=3
    let cmp = |other: &Neighbor| -> Ordering {
      if other.dist < neighbor.dist { Ordering::Less }
      else if other.dist == neighbor.dist { other.id.cmp(&neighbor.id) }
      else { Ordering::Greater }
    };

    // this compare function emits conditional moves in opt-level=2 and 3
    // let cmp = |other: &Neighbor| -> Ordering {
    //   match other.dist.total_cmp( &neighbor.dist ) {
    //     Ordering::Equal => other.id.cmp( &neighbor.id ),
    //     ordering => ordering,
    //   }
    // };

    if let Err( pos ) = self.neighbors.binary_search_by( cmp ) && pos < self.capacity.get() {
      if self.neighbors.len() == self.capacity.get() {
        _ = self.neighbors.pop();
      }
      unsafe { std::hint::assert_unchecked( self.neighbors.len() < self.neighbors.capacity() ) };
      self.neighbors.insert( pos, neighbor );
    }
  }

  pub fn clear( &mut self ) {
    self.neighbors.clear();
  }
}
