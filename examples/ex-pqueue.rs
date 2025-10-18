use std::hint::black_box;
use std::num::NonZeroUsize;
use std::time::Instant;
use test_pqueue::queue::{Neighbor, Queue};

pub fn main() {
  let data = generate_random_floats();
  let mut queue = Queue::with_capacity( NonZeroUsize::new(64).unwrap() );

  let start = Instant::now();
  for _ in 0..3_000_000 {
    queue.clear();
    for neighbor in &data {
      queue.insert(black_box( *neighbor ));
    }
    black_box( &queue );
  }
  println!( "done in {elapsed}ms", elapsed = start.elapsed().as_millis() );
}

fn generate_random_floats() -> Vec<Neighbor> {
  use rand::{
    SeedableRng,
    distr::{Distribution, Uniform},
    rngs::StdRng,
    seq::SliceRandom,
  };

  let seed = [ 42u8; 32 ];
  let mut rng = StdRng::from_seed( seed );
  let range = Uniform::new( 0.0f32, 1.0f32 ).unwrap();

  let mut neighbors = Vec::with_capacity( 100 );
  let mut ids = (0..100).collect::<Vec<u32>>();
  ids.shuffle( &mut rng );

  for id in ids {
      let dist = range.sample( &mut rng );
      let neighbor = Neighbor{ id, dist };
      neighbors.push( neighbor );
  }

  neighbors
}
