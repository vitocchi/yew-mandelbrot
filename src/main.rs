mod mandelbrot;
use wasm_bindgen::JsCast;
use yew::events::ChangeData;
use yew::prelude::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};
use yew::services::ConsoleService;
use yew::web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

struct App {
    link: ComponentLink<Self>,
    canvas_ref: NodeRef,
    iteration_limit: usize,
}

enum Msg {
    ChangeIterationLimit(ChangeData),
    DrawMandelbrot,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            canvas_ref: NodeRef::default(),
            iteration_limit: 10,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeIterationLimit(event) => {
                if let ChangeData::Value(value) = event {
                    ConsoleService::info(format!("change data: {:}", value).as_str());
                    self.iteration_limit = value.parse().unwrap();
                    self.link.send_message(Msg::DrawMandelbrot);
                }
                true
            }
            Msg::DrawMandelbrot => {
                self.draw_canvas();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div>
                <input type="number" name="iteration_limit" value={self.iteration_limit.to_string()} onchange=self.link.callback(|e| Msg::ChangeIterationLimit(e))/>
                </div>
                <div>
                <canvas ref={self.canvas_ref.clone()} id="canvas" width="60" height="40"></canvas>
                </div>
            </>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.draw_canvas();
        }
    }
}

impl App {
    fn draw_canvas(&mut self) {
        ConsoleService::info("draw canvas");
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        canvas.set_width(600);
        canvas.set_height(600);
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        ctx.clear_rect(0.0, 0.0, 600.0, 600.0);
        let screen =
            mandelbrot::mandelbrot_screen(600, 600, -3.0, -2.0, 0.01, self.iteration_limit);
        let image_data = screen.image_data();
        ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }
}

fn main() {
    yew::start_app::<App>();
}
