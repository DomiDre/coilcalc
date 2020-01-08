use crate::current_loop::{calculate_magnetic_field, CurrentLoop};
use yew::prelude::*;
pub struct Model {
    link: ComponentLink<Self>,
    view_width: f64,
    x_range: (f64, f64, usize),
    z_range: (f64, f64, usize),
    current_loops: Vec<CurrentLoop>,
    magnetic_field: Vec<Vec<(f64, f64, f64)>>,
    base_length: f64
}

pub enum Msg {
    UpdateView,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let current_loops = vec![
            CurrentLoop::new(0.0, 0.0, -9.0, 5.0, 1.0),
            CurrentLoop::new(0.0, 0.0, 0.0, 5.0, 1.0),
            CurrentLoop::new(0.0, 0.0, 9.0, 5.0, 1.0),
        ];
        let x_range = (-30.0, 31.0, 40);
        let z_range = (-30.0, 31.0, 40);
        let magnetic_field = calculate_magnetic_field(&current_loops, &x_range, &z_range);
        let mut base_length = 0.0;
        let mut num = 0;
        for row in magnetic_field.iter() {
            for b_field in row.iter() {
                let length = (b_field.0.powi(2) + b_field.1.powi(2) + b_field.2.powi(2)).sqrt();
                if length > 1e6 {
                    continue
                }
                base_length += length;
                num += 1;
            }
        }
        base_length /= num as f64;
        Self {
            link,
            view_width: 400.0,
            x_range,
            z_range,
            current_loops,
            magnetic_field,
            base_length
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateView => {
                self.magnetic_field =
                    calculate_magnetic_field(&self.current_loops, &self.x_range, &self.z_range);
            }
        }
        true
    }

    fn view(&self) -> Html {
        // let input_text = self.link.callback(|i: InputData| Msg::SetText(i.value));
        html! {
            <section class="app-container">
                <header>
                    <h1>{"Coil Calculator"}</h1>
                </header>
                <main>
                </main>
                <output>
                    { self.display_magnetic_field() }
                </output>
            </section>
        }
    }
}

impl Model {
    fn display_magnetic_field(&self) -> Html {
        html! {
            <div style=format!("border: black solid 1px; width: {}px;", self.view_width)>
                { for self.magnetic_field.iter().rev().map(|row| self.display_row(row)) }
            </div>
        }
    }

    fn display_row(&self, row: &Vec<(f64, f64, f64)>) -> Html {
        html! {
            <div style="display:flex;">
                { for row.iter().map(|b_field| self.draw_unit_arrow(b_field)) }
            </div>
        }
    }

    fn draw_unit_arrow(&self, vector: &(f64, f64, f64)) -> Html {
        let length = (vector.0.powi(2) + vector.1.powi(2) + vector.2.powi(2)).sqrt();

        // display in x-z plane
        let angle = (vector.2).atan2(vector.0);
        let cos_phi = angle.cos();
        let sin_phi = angle.sin();
        let half_length = 20.0;
        let center = 50.0;
        let from = (
            center - half_length * cos_phi,
            center - half_length * sin_phi,
        );
        let to = (
            center + half_length * cos_phi,
            center + half_length * sin_phi,
        );
        self.draw_arrow(from, to, length/self.base_length)
    }

    fn draw_arrow(&self, from: (f64, f64), to: (f64, f64), magnitude: f64) -> Html {
        let view_limit = 100.0;
        let red = 0.0 + magnitude*255.0;
        let blue = 0.0;// + magnitude*255.0;
        let green = 0.0;// + magnitude*255.0;
        let color = format!("rgb({},{},{})", red, green, blue);
        let id = format!("arrow_{}", magnitude);
        html! {
            <svg viewBox=format!("0 0 {} {}", view_limit, view_limit)
                width=format!("{}", self.view_width/(self.x_range.2 as f64))>
                // arrow head
                <defs>
                    <marker id=id
                        markerWidth="10" markerHeight="10"
                        refX="2" refY="6" orient="auto">
                        <path d="M2,2 L2,10 L9,6 L2,2" style=format!("fill:{};", color)/>
                    </marker>
                </defs>

                // arrow tail
                <path d=format!("M{},{} L{},{}", from.0, view_limit-from.1, to.0, view_limit-to.1)
                    style=format!("stroke:{}; stroke-width: 4px; fill: none;
                            marker-end: url(#{});", color, id)
                />
            </svg>
        }
    }
}
