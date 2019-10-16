/// Yew Agent to route requests to Clear from the App State to all channels (1:M message routing)
use crate::channel::*;
use crate::state::*;
use std::collections::HashSet;
use yew::agent::*;
use yew::services::ConsoleService;

pub struct ClearAgent {
	console: ConsoleService,
	link: AgentLink<ClearAgent>,
	listeners: HashSet<HandlerId>,
}

impl Transferable for ChannelMsg {}
impl Transferable for StateMsg {}

impl Agent for ClearAgent {
	type Reach = Context; //spawn one agent only on the main thread, all components will use the same router.
	type Input = StateMsg; // State calls us on Clear Requested
	type Output = ChannelMsg; // We call channels asking them to clear.
	type Message = ();

	fn create(link: AgentLink<Self>) -> Self {
		let mut console = ConsoleService::new();
		console.debug("ClearAgent: Created...");

		ClearAgent {
			console,
			link,
			listeners: HashSet::new(),
		}
	}

	/// This method called on every update message.
	fn update(&mut self, _msg: Self::Message) { /* Nothing to do here */
	}

	/// This method called on when a new bridge created.
	fn connected(&mut self, id: HandlerId) {
		self.console.debug("ClearAgent: New Child connected...");
		self.listeners.insert(id); //store this component away for later
	}

	// Handle incoming messages from components of other agents.
	fn handle(&mut self, msg: Self::Input, _who_sent_it: HandlerId) {
		match msg {
			StateMsg::Clear => {
				// broadcast it to all listening components
				self.console.debug(
					format!("ClearAgent: Requesting subscribers clear themselves.").as_str(),
				);

				for sub in self.listeners.iter() {
					self.link.response(*sub, ChannelMsg::Clear);
				}
			}
			_ => self
				.console
				.warn("ClearAgent: Unexpected message received!"), //ignore all other state messages
		}
	}
}
