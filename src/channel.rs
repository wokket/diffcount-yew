use crate::clear_agent::*;
use serde_derive::{Deserialize, Serialize};
use yew::agent::*;
use yew::services::ConsoleService;
use yew::*;

pub struct Channel {
	pub value: i32,
	pub channel_num: u8,
	pub total: i32,
	// Callback back up to parent to let it know to increment global totals/ percentages etc
	pub on_increment: Option<Callback<crate::state::StateMsg>>,
}
pub struct ChannelComponent {
	channel: Channel,
	console: ConsoleService,
	/// We need to keep a reference to the bridge around, or we get https://github.com/yewstack/yew/issues/712
	_clear_agent_bridge: Box<dyn Bridge<ClearAgent>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChannelMsg {
	Increment,
	Clear,
}

#[derive(Properties)]
pub struct Props {
	pub channel_num: u8,
	pub total: i32,
	pub on_increment: Option<Callback<crate::state::StateMsg>>,
}

impl Default for Props {
	fn default() -> Self {
		Props {
			channel_num: 0,
			total: 0,
			on_increment: None,
		}
	}
}

impl Channel {
	pub fn on_incremented(&mut self) {
		self.value = self.value + 1;

		if let Some(ref mut callback) = self.on_increment {
			callback.emit(crate::state::StateMsg::Incremented);
		}
	}

	pub fn on_cleared(&mut self) {
		self.value = 0;
	}

	/// Simple helper function to calc a percentage.
	fn display_percentage(&self) -> f32 {
		if self.total == 0 {
			return 0.0;
		}

		return (self.value as f32 / self.total as f32) * 100 as f32;
	}
}

impl Component for ChannelComponent {
	type Message = ChannelMsg;
	type Properties = Props;

	fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
		let callback = link.send_back(|_| ChannelMsg::Clear);

		ChannelComponent {
			console: ConsoleService::new(),
			channel: Channel {
				value: 0,
				total: props.total,
				channel_num: props.channel_num,
				on_increment: props.on_increment,
			},
			_clear_agent_bridge: ClearAgent::bridge(callback),
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			ChannelMsg::Increment => {
				self.channel.on_incremented();
				self.console.log("Channel: Incremented...");
			}
			ChannelMsg::Clear => {
				self.channel.on_cleared();
				self.console.log("Channel: Cleared...");
			}
		}
		true
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.channel.total = props.total;
		true
	}
}

impl Renderable<Self> for ChannelComponent {
	fn view(&self) -> Html<Self> {
		html! {
		<div class="pure-u-1-6">
					<a class="pure-button button-channel" onclick=|_| ChannelMsg::Increment>
					{ format!("Channel {}", self.channel.channel_num) }
					<br />
					<strong>{ format!(" {} ", self.channel.value) }</strong>
						{ format!("({:.2}%)", self.channel.display_percentage()) }
					</a>

					</div>
				}
	}
}
