use yew::prelude::*;

pub struct PageHeader {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub empire_name: String,
    pub player_name: String,
}

impl Component for PageHeader {
    type Message = ();

    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        PageHeader { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        if self.props != _props {
            self.props = _props;
            true
        } else {
            false
        }
    }
    fn view(&self) -> Html {
        html! {
            <ul class="page_header">
                <li class="empire_name"> { format!("{}", &self.props.empire_name) } </li>
                <li class="player_name">{ { format!("({})", &self.props.player_name) } }</li>
            </ul>
        }
    }
}
