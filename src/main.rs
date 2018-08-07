#![feature(async_await, await_macro, futures_api, tokio_compat)]

#[macro_use]
extern crate futures;

use f1::{Future, Stream};
use futures::{
  compat::{Future01CompatExt, TokioDefaultExecutor},
  FutureExt, TryFutureExt,
};
use std::time::{Duration, Instant};
use tokio::timer::Interval;

fn main() {
  let future = async {
    let timer = Interval::new(Instant::now(), Duration::from_secs(1))
      .take(3)
      .for_each(|_| {
        println!("tick");
        Ok(())
      }).map_err(|e| panic!("timer error: {}", e));

    let join_handle = spawn_with_handle!(timer.compat()).unwrap();
    await!(join_handle);
  };

  let future_compat = future.boxed().unit_error().compat(TokioDefaultExecutor);

  tokio::run(future_compat);
}
