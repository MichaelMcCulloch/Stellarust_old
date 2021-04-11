use super::header::PageHeader;

use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    clicked: bool,
}

pub enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            clicked: false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="main"> 
               <PageHeader empire_name="THE GREAT KHANATE" player_name="George"/>
            </div>
        }
    }
}