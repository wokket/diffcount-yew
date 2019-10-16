use crate::clear_agent::*;
use serde_derive::{Deserialize, Serialize};
use yew::agent::*;
use yew::services::ConsoleService;
use yew::*;

pub struct Channel {
	console: ConsoleService,
	value: i32,
	channel_num: u8,
	total: i32,
	// Callback back up to parent to let it know to increment global totals/ percentages etc
	on_increment: Option<Callback<crate::state::StateMsg>>,

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

impl Component for Channel {
	type Message = ChannelMsg;
	type Properties = Props;

	fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
		let callback = link.send_back(|_| ChannelMsg::Clear);

		Channel {
			console: ConsoleService::new(),
			value: 0,
			total: props.total,
			channel_num: props.channel_num,
			on_increment: props.on_increment,
			_clear_agent_bridge: ClearAgent::bridge(callback),
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			ChannelMsg::Increment => {
				self.value = self.value + 1;

				if let Some(ref mut callback) = self.on_increment {
					callback.emit(crate::state::StateMsg::Incremented);
				}
				self.console.log("Channel: Incremented...");
			}
			ChannelMsg::Clear => {
				self.value = 0;
				self.console.log("Channel: Cleared...");
			}
		}
		true
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		self.total = props.total;
		true
	}
}

impl Channel {
	/// Simple helper function to calc a percentage.
	fn display_percentage(&self) -> f32 {
		if self.total == 0 {
			return 0.0;
		}

		return (self.value as f32 / self.total as f32) * 100 as f32;
	}
}

impl Renderable<Self> for Channel {
	fn view(&self) -> Html<Self> {
		html! {

			<button style="width: 15%; height: 30px; "
				onclick=|_| ChannelMsg::Increment>
				{ format!("Channel {}: {} ({:.2}%)", self.channel_num, self.value, self.display_percentage()) }
			</button>
		}
	}
}
