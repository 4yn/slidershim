use log::info;
use parking_lot::Mutex;
use std::{
  sync::Arc,
  thread::{self, JoinHandle},
};
use tokio::{
  select,
  sync::{mpsc, oneshot},
};

use crate::{config::Config, context::Context, state::SliderState};

pub struct Manager {
  state: Arc<Mutex<Option<SliderState>>>,
  context: Arc<Mutex<Option<Context>>>,
  join_handle: Option<JoinHandle<()>>,
  tx_config: mpsc::UnboundedSender<Config>,
  tx_stop: Option<oneshot::Sender<()>>,
}

impl Manager {
  pub fn new() -> Self {
    let state = Arc::new(Mutex::new(None));
    let (tx_config, mut rx_config) = mpsc::unbounded_channel::<Config>();
    let (tx_stop, rx_stop) = oneshot::channel::<()>();

    let context: Arc<Mutex<Option<Context>>> = Arc::new(Mutex::new(None));

    let state_cloned = Arc::clone(&state);
    let context_cloned = Arc::clone(&context);

    let join_handle = thread::spawn(move || {
      info!("Manager thread started");
      let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();
      runtime.block_on(async move {
        info!("Manager runtime started");

        select! {
          _ = async {
            loop {
              match rx_config.recv().await {
                Some(config) => {
                  info!("Rebuilding context");
                  let mut context_handle = context_cloned.lock();
                  context_handle.take();

                  let new_context = Context::new(config);
                  let new_state = new_context.clone_state();
                  context_handle.replace(new_context);

                  let mut state_handle = state_cloned.lock();
                  state_handle.replace(new_state);
                },
                None => {
                  let mut context_handle = context_cloned.lock();
                  context_handle.take();
                }
              }
            }
          } => {},
          _ = rx_stop => {}
        }
      });
    });

    Self {
      state,
      context,
      join_handle: Some(join_handle),
      tx_config,
      tx_stop: Some(tx_stop),
    }
  }

  pub fn update_config(&self, config: Config) {
    self.tx_config.send(config).unwrap();
  }

  pub fn try_get_state(&self) -> Option<SliderState> {
    let state_handle = self.state.lock();
    state_handle.as_ref().map(|x| x.clone())
  }

  pub fn get_timer_state(&self) -> String {
    let context_handle = self.context.lock();
    context_handle
      .as_ref()
      .map(|context| context.timer_state())
      .unwrap_or("".to_string())
  }
}

impl Drop for Manager {
  fn drop(&mut self) {
    if let Some(tx_stop) = self.tx_stop.take() {
      tx_stop.send(()).ok();
    }
    if let Some(join_handle) = self.join_handle.take() {
      join_handle.join().ok();
    }
  }
}
