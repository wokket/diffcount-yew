use crate::channel::*;
use crate::clear_agent::*;
use serde_derive::{Deserialize, Serialize};
use yew::agent::*;
use yew::services::ConsoleService;
use yew::*;

/// Reflects the current running state of the application.
pub struct State {
	console: ConsoleService,
	pub total: i32,
	/// Trigger a small alarm every `alarm_count` total entries, so users know how many they're up to.
	pub alarm_count: i32,
	pub clear_agent: Box<dyn Bridge<ClearAgent>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StateMsg {
	/// Called from UI to request a reset to 0 counts
	Clear,
	/// Passed from a channel to indicate it's value has been bumped up
	Incremented,
	/// Called when each child channel has cleared it's values...
	Cleared,
}

impl State {
	/// return true iff the user alarm is currently triggered.
	pub fn is_alarm_triggered(&self) -> bool {
		self.total > 0 && self.total % self.alarm_count == 0
	}
}

impl Component for State {
	type Message = StateMsg;
	type Properties = ();

	fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
		let callback = link.send_back(|_| StateMsg::Cleared);
		let clear_agent = ClearAgent::bridge(callback);

		State {
			console: ConsoleService::new(),
			alarm_count: 10,
			total: 0,
			clear_agent,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			StateMsg::Clear => {
				self.console.info("Clear app requested..");
				self.clear_agent.send(msg); // call 1:M message router to request child channels to clear
			}
			StateMsg::Cleared => {
				// Children have now reset their internal counts (or may still be working, but the agent has done it's bit)
				self.console.log("Child Clear Complete");
				self.total = 0;
			}
			StateMsg::Incremented => {
				self.total += 1;
			}
		}
		true
	}
}

impl Renderable<Self> for State {
	fn view(&self) -> Html<Self> {
		let header = match self.is_alarm_triggered() {
			true => {
				ConsoleService::new().debug("State: Alarm Triggered");
				html! {<h2 class="alarmed">{ format!("Total Count: {}", self.total) } </h2> }
			}
			false => {
				html! {<h2>{ format!("Total Count: {}", self.total) } </h2>}
			}
		};

		html! {
			<div>
				{header}
				<div class="pure-g">
				<Channel channel_num=1 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=2 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=3 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=4 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=5 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=6 total=self.total on_increment=|msg| msg/>
				</div>
				<div class="pure-g">
				<Channel channel_num=7 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=8 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=9 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=10 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=11 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=12 total=self.total on_increment=|msg| msg/>
				</div>
				<br /><br />

				<button class="pure-button button-clear" onclick=|msg| StateMsg::Clear>{"Clear"}</button>
			</div>
		}
	}
}
