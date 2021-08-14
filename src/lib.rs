//! `pulsectl` is a high level wrapper around the PulseAudio bindings supplied by
//! `libpulse-binding` to make application development easier. It provides simple access to sinks,
//! inputs, sources and outputs allowing one to write audio control programs with ease. This
//! library is only capable of modifying PulseAudio data (e.g., changing volume, routing
//! applications, and muting).
//!
//! This is a fork of [` JojiiOfficial/pulsectl-rust-fork`](https://github.com/JojiiOfficial/pulsectl)
//! which is then a fork of [`krruzic/pulsectl-rust`](https://github.com/krruzic/pulsectl).
//!
//! # Example
//!
//! List all currently connected playback devices
//!
//! ```no_run
//! use pulsectl::controllers::SinkController;
//! use pulsectl::controllers::DeviceControl;
//!
//! // create handler that calls functions on playback devices and apps
//! let mut handler = SinkController::create().unwrap();
//!
//! let devices = handler
//!     .list_devices()
//!     .expect("Could not get list of playback devices.");
//!
//! println!("Playback Devices: ");
//! for dev in devices.clone() {
//!     println!(
//!         "[{}] {}, Volume: {}",
//!         dev.index,
//!         dev.description.as_ref().unwrap(),
//!         dev.volume.print()
//!     );
//! }
//! ```
//!
//! For a more complete example, see `examples/change_device_vol.rs`.

pub mod controllers;
mod error;

use pulse::{
    context::{introspect::Introspector, Context},
    mainloop::standard::{IterateResult, Mainloop},
    operation::{Operation, State},
    proplist::Proplist,
};

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub use crate::controllers::error::ControllerError;
pub use crate::error::Error;

/// The main wrapper of libpulse-binding API.
pub struct Handler {
    /// Safe interface to the internal PA Mainloop.
    pub mainloop: Rc<RefCell<Mainloop>>,
    /// An opaque connection context to a daemon.
    pub context: Rc<RefCell<Context>>,
    /// A wrapper object providing introspection routines to a context.
    pub introspect: Introspector,
}

impl Handler {
    pub fn connect(name: &str) -> Result<Handler, Error> {
        let mut proplist = Proplist::new().unwrap();
        proplist
            .set_str(pulse::proplist::properties::APPLICATION_NAME, name)
            .unwrap();

        let mainloop;
        if let Some(m) = Mainloop::new() {
            mainloop = Rc::new(RefCell::new(m));
        } else {
            return Err(Error::Connect("Failed to create mainloop".to_string()));
        }

        let context;
        if let Some(c) =
            Context::new_with_proplist(mainloop.borrow().deref(), "MainConn", &proplist)
        {
            context = Rc::new(RefCell::new(c));
        } else {
            return Err(Error::Connect("Failed to create new context".to_string()));
        }

        context
            .borrow_mut()
            .connect(None, pulse::context::FlagSet::NOFLAGS, None)
            .map_err(|_| Error::Connect("Failed to connect context".to_string()))?;

        loop {
            match mainloop.borrow_mut().iterate(false) {
                IterateResult::Err(e) => {
                    eprintln!("iterate state was not success, quitting...");
                    return Err(e.into());
                }
                IterateResult::Success(_) => {}
                IterateResult::Quit(_) => {
                    eprintln!("iterate state was not success, quitting...");
                    return Err(Error::Connect(
                        "Iterate state quit without an error".to_string(),
                    ));
                }
            }

            match context.borrow().get_state() {
                pulse::context::State::Ready => break,
                pulse::context::State::Failed | pulse::context::State::Terminated => {
                    eprintln!("context state failed/terminated, quitting...");
                    return Err(Error::Connect(
                        "Context state failed/terminated without an error".to_string(),
                    ));
                }
                _ => {}
            }
        }

        let introspect = context.borrow_mut().introspect();
        Ok(Handler {
            mainloop,
            context,
            introspect,
        })
    }

    /// loops until the passed operation is completed
    pub fn wait_for_operation<G: ?Sized>(&mut self, op: Operation<G>) -> Result<(), Error> {
        loop {
            match self.mainloop.borrow_mut().iterate(false) {
                IterateResult::Err(e) => return Err(e.into()),
                IterateResult::Success(_) => {}
                IterateResult::Quit(_) => {
                    return Err(Error::Operation(
                        "Iterate state quit without an error".to_string(),
                    ));
                }
            }
            match op.get_state() {
                State::Done => {
                    break;
                }
                State::Running => {}
                State::Cancelled => {
                    return Err(Error::Operation(
                        "Operation cancelled without an error".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        self.context.borrow_mut().disconnect();
        self.mainloop.borrow_mut().quit(pulse::def::Retval(0));
    }
}
