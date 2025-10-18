use std::hint::black_box;
use std::num::NonZeroUsize;
use std::time::Duration;
use criterion::{criterion_group, criterion_main, Criterion};
use test_pqueue::queue::{Neighbor, Queue};


fn bench_pqueue_insert( c: &mut Criterion ) {
  let neighbors = generate_random_neighbors();
  let mut group = c.benchmark_group( "pqueue-insert" );
  group.measurement_time( Duration::from_secs(5) ).sample_size( 10_000 );

  let mut queue = Queue::with_capacity( NonZeroUsize::new(64).unwrap() );
  group.bench_function( "Priority Queue Insert", |bencher| {
    bencher.iter( || {
      queue.clear();
      for neighbor in neighbors.iter() {
        queue.insert(black_box( *neighbor ));
      }
      black_box( &queue );
    });
  });
}

fn generate_random_neighbors() -> Vec<Neighbor> {
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

criterion_group!( benches, bench_pqueue_insert );
criterion_main!( benches );
