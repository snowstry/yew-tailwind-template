use yew::{html, Component, Context, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub text: String,
}

pub struct Text;

impl Component for Text {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1 class="grid place-items-center h-screen text-gray-300">{format!("{}", ctx.props().text)}</h1>
        }
    }
}
