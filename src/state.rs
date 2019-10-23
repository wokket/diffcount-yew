use crate::channel::*;
use crate::clear_agent::*;
use serde_derive::{Deserialize, Serialize};
use yew::agent::*;
use yew::services::ConsoleService;
use yew::*;

pub struct State {
	//we put all the non-wasm things in here for testing, like seperating a model from a view
	pub total: i32,
	/// Trigger a small alarm every `alarm_count` total entries, so users know how many they're up to.
	pub alarm_count: i32,
}

/// Reflects the current running state of the application.
pub struct StateComponent {
	//TODO: It would be nice if these weren't public
	pub console: ConsoleService,
	pub state: State,
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
	pub fn on_incremented(&mut self) {
		self.total += 1;
	}

	pub fn on_cleared(&mut self) {
		self.total = 0;
	}
}

impl Component for StateComponent {
	type Message = StateMsg;
	type Properties = ();

	fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
		let callback = link.send_back(|_| StateMsg::Cleared);
		let clear_agent = ClearAgent::bridge(callback);

		StateComponent {
			console: ConsoleService::new(),
			clear_agent: clear_agent,
			state: State {
				alarm_count: 10,
				total: 0,
			},
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
				self.state.on_cleared();
			}
			StateMsg::Incremented => {
				self.state.on_incremented();
			}
		}
		true
	}
}

impl Renderable<Self> for StateComponent {
	fn view(&self) -> Html<Self> {
		let header = match self.state.is_alarm_triggered() {
			true => {
				//self.console.log("State: Alarm Triggered");
				html! {
					<div>
					//Call the beep() js function declared on the index.html
					<script>{ "window.beep();" }</script>
					<h2 class="alarmed">{ format!("Total Count: {}", self.state.total) } </h2>
					</div>
				}
			}
			false => {
				html! {<h2>{ format!("Total Count: {}", self.state.total) } </h2>}
			}
		};

		html! {
			<div>
				{header}
				<div class="pure-g">
				<ChannelComponent channel_num=1 total=self.state.total on_increment=|msg| msg/>
				<ChannelComponent channel_num=2 total=self.state.total on_increment=|msg| msg/>
				<ChannelComponent channel_num=3 total=self.state.total on_increment=|msg| msg/>
				<ChannelComponent channel_num=4 total=self.state.total on_increment=|msg| msg/>
				<ChannelComponent channel_num=5 total=self.state.total on_increment=|msg| msg/>
				<ChannelComponent channel_num=6 total=self.state.total on_increment=|msg| msg/>
				</div>
				<div class="pure-g">
				<ChannelComponent channel_num=7 total=self.state.total on_increment=|msg| msg/>
				<ChannelComponent channel_num=8 total=self.state.total on_increment=|msg| msg/>
				<ChannelComponent channel_num=9 total=self.state.total on_increment=|msg| msg/>
				<ChannelComponent channel_num=10 total=self.state.total on_increment=|msg| msg/>
				<ChannelComponent channel_num=11 total=self.state.total on_increment=|msg| msg/>
				<ChannelComponent channel_num=12 total=self.state.total on_increment=|msg| msg/>
				</div>
				<br /><br />

				<button class="pure-button button-clear" onclick=|msg| StateMsg::Clear>{"Clear"}</button>
			</div>
		}
	}
}
