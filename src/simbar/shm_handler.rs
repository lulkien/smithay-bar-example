use smithay_client_toolkit::{
    delegate_shm,
    shm::{Shm, ShmHandler},
};

use super::SimBar;

delegate_shm!(SimBar);

impl ShmHandler for SimBar {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}
