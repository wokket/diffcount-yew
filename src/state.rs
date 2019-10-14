use crate::channel::*;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

/// Reflects the current running state of the application.
pub struct State {
	pub total: i32,
	/// Trigger a small alarm every `alarm_count` total entries, so users know how many they're up to.
	pub alarm_count: i32,
}

pub enum StateMsg {
	/// Called from UI to request a reset to 0 counts
	Clear,
	/// Passed from a channel to indicate it's value has been bumped up
	Incremented,
}

impl State {}

impl Component for State {
	type Message = StateMsg;
	type Properties = ();

	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		State {
			alarm_count: 100,
			total: 0,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			StateMsg::Clear => {
				//TODO
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
		html! {
			<div>
				<div>{format!("Total Count: {}", self.total) }</div>
				<br />
				<Channel channel_num=1 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=2 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=3 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=4 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=5 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=6 total=self.total on_increment=|msg| msg/>
				<br />
				<Channel channel_num=7 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=8 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=9 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=10 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=11 total=self.total on_increment=|msg| msg/>
				<Channel channel_num=12 total=self.total on_increment=|msg| msg/>
			</div>
		}
	}
}
