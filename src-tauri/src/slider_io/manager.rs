use log::info;
use std::{
  sync::{Arc, Mutex},
  thread::{self, JoinHandle},
};
use tokio::{
  select,
  sync::{mpsc, oneshot},
};

use crate::slider_io::{config::Config, context::Context};

use super::controller_state::FullState;

pub struct Manager {
  state: Arc<Mutex<Option<FullState>>>,
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
        .worker_threads(1)
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
                  let mut context_handle = context_cloned.lock().unwrap();
                  context_handle.take();

                  let new_context = Context::new(config);
                  let new_state = new_context.clone_state();
                  context_handle.replace(new_context);

                  let mut state_handle = state_cloned.lock().unwrap();
                  state_handle.replace(new_state);
                },
                None => {
                  let mut context_handle = context_cloned.lock().unwrap();
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
      join_handle: Some(join_handle),
      tx_config,
      tx_stop: Some(tx_stop),
    }
  }

  pub fn update_config(&self, config: Config) {
    self.tx_config.send(config).unwrap();
  }

  pub fn try_get_state(&self) -> Option<FullState> {
    let state_handle = self.state.lock().unwrap();
    state_handle.as_ref().map(|x| x.clone())
  }
}

impl Drop for Manager {
  fn drop(&mut self) {
    self.tx_stop.take().unwrap().send(()).unwrap();
    self.join_handle.take().unwrap().join().unwrap();
  }
}
