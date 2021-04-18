use yew::prelude::*;

pub struct Graph {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub what: u32,
}

impl Component for Graph {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        todo!()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        todo!()
    }
}
