use crate::current_loop::{calculate_magnetic_field, CurrentLoop};
use yew::prelude::*;

use stdweb::unstable::TryInto;

pub struct Model {
    link: ComponentLink<Self>,
    view_width: f64,
    x_range: (f64, f64, usize),
    z_range: (f64, f64, usize),
    current_loops: Vec<CurrentLoop>,
    magnetic_field: Vec<Vec<(f64, f64, f64)>>,
    base_length: f64,
    arrow_box_length: f64,
    selected_coil: Option<usize>
}

pub enum Msg {
    UpdateView,
    DragStart(usize),
    DragMouseMove(MouseMoveEvent),
    DragMouseLeave(MouseLeaveEvent),
    DragStop(),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let current_loops = vec![
            CurrentLoop::new(0.0, 0.0, 0.0, 5.0, 1.0)
        ];
        let x_range = (-30.0, 30.0, 20);
        let z_range = (-30.0, 30.0, 20);
        let magnetic_field = calculate_magnetic_field(&current_loops, &x_range, &z_range);
        let mut base_length = 0.0;
        let mut num = 0;
        for row in magnetic_field.iter() {
            for b_field in row.iter() {
                let length = (b_field.0.powi(2) + b_field.1.powi(2) + b_field.2.powi(2)).sqrt();
                base_length += length;
                num += 1;
            }
        }
        base_length /= num as f64;
        let view_width = 300.0; // width in pixel for the display image
        let arrow_box_length = 100.0 / x_range.2 as f64; // edge of square containing 1 arrow
        Self {
            link,
            view_width,
            x_range,
            z_range,
            current_loops,
            magnetic_field,
            base_length,
            arrow_box_length,
            selected_coil: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateView => {
                self.magnetic_field =
                    calculate_magnetic_field(&self.current_loops, &self.x_range, &self.z_range);
            }
            Msg::DragStart(idx) => {
                self.selected_coil = Some(idx);
            }
            Msg::DragMouseMove(evt) => {
                self.update_current_loop_position(evt.client_x() as f64, evt.client_y() as f64);
            }
            Msg::DragMouseLeave(evt) => {
                self.update_current_loop_position(evt.client_x() as f64, evt.client_y() as f64);
            }
            Msg::DragStop() => {
                self.selected_coil = None;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <section class="app-container">
                <header>
                    <h1>{"Coil Calculator"}</h1>
                </header>
                <main style="display: flex; flex-direction: column;">
                    <div style="border: solid 1px black; background-color: #FAFAFA;">
                        <svg id="arrow_plot" viewBox="-10 -10 120 120" width=format!("{}", self.view_width)>
                            { self.display_magnetic_field() }
                            { self.display_current_loops() }
                        </svg>
                    </div>
                    <div style="display: flex; justify-content: flex-end;">
                        <a href="https://github.com/DomiDre/coilcalc/" target="_blank" rel="noopener">
                            <img style="width: 30px; cursor: pointer;" src="github.svg"/>
                        </a>
                        <a href="https://twitter.com/DomiDre" target="_blank" rel="noopener">
                            <img style="width: 30px; cursor: pointer;" src="twitter.svg"/>
                        </a>
                    </div>
                </main>
            </section>
        }
    }
}

impl Model {
    fn display_magnetic_field(&self) -> Html {
        html! {
            <>
                { for self.magnetic_field.iter().enumerate().map(|(i, row)| self.display_row(i, row)) }
            </>
        }
    }

    fn display_row(&self, idx_row: usize, row: &Vec<(f64, f64, f64)>) -> Html {
        html! {
            <>
                { for row.iter().enumerate().map(|(idx_col, b_field)| self.draw_unit_arrow(idx_row, idx_col, b_field)) }
            </>
        }
    }

    fn draw_unit_arrow(&self, idx_row: usize, idx_col: usize, vector: &(f64, f64, f64)) -> Html {
        let length = (vector.0.powi(2) + vector.1.powi(2) + vector.2.powi(2)).sqrt();

        // display in x-z plane
        let angle = (vector.2).atan2(vector.0);
        let cos_phi = angle.cos();
        let sin_phi = angle.sin();
        let half_length = self.arrow_box_length/4.0;
        let center_x = (idx_col as f64 + 0.5)*self.arrow_box_length;
        let center_y = (idx_row as f64 + 0.5)*self.arrow_box_length;
        let from = (
            center_x - half_length * cos_phi,
            center_y - half_length * sin_phi,
        );
        let to = (
            center_x + half_length * cos_phi,
            center_y + half_length * sin_phi,
        );
        self.draw_arrow(from, to, length/self.base_length)
    }

    fn draw_arrow(&self, from: (f64, f64), to: (f64, f64), magnitude: f64) -> Html {
        let red = 0.0 + magnitude*255.0;
        let blue = 0.0;// + magnitude*255.0;
        let green = 0.0;// + magnitude*255.0;
        let color = format!("rgb({},{},{})", red, green, blue);
        let id = format!("arrow_{}", magnitude);
        let stroke = self.arrow_box_length/100.0*4.0;
        html! {
            <>
                // arrow head
                <defs>
                    <marker id=id
                        markerWidth="10" markerHeight="10"
                        refX="2" refY="6" orient="auto">
                        <path d="M2,2 L2,10 L9,6 L2,2" style=format!("fill:{};", color)/>
                    </marker>
                </defs>

                // arrow tail
                <path d=format!("M{},{} L{},{}", from.0, from.1, to.0, to.1)
                    style=format!("stroke:{}; stroke-width: {}px; fill: none;
                            marker-end: url(#{});", color, stroke, id)
                />
            </>
        }
    }

    fn display_current_loops(&self) -> Html {
        html! {
            <>
                { for self.current_loops.iter().enumerate().map(|(idx, current_loop)| self.draw_circle(idx, current_loop)) }
            </>
        }
    }

    fn draw_circle(&self, idx: usize, current_loop: &CurrentLoop ) -> Html {
        let on_start_drag = self.link.callback(move |_| Msg::DragStart(idx));
        let on_drag = self.link.callback(move |evt: MouseMoveEvent| Msg::DragMouseMove(evt));
        let on_mouse_leave = self.link.callback(|evt: MouseLeaveEvent| Msg::DragMouseLeave(evt));
        let on_stop_drag = self.link.callback(|_| Msg::DragStop());
        html! {
            <>
                <circle
                cx={(current_loop.r_center.0-current_loop.radius-self.x_range.0)*100.0/(self.x_range.1-self.x_range.0)}
                cy={(current_loop.r_center.2-self.z_range.0)*100.0/(self.z_range.1-self.z_range.0)}
                r="5" fill="#ff8300"
                draggable="true" style="cursor: move"
                onmousedown=on_start_drag onmousemove=on_drag onmouseup=on_stop_drag onmouseleave=on_mouse_leave
                />
                <circle
                cx={(current_loop.r_center.0+current_loop.radius-self.x_range.0)*100.0/(self.x_range.1-self.x_range.0)}
                cy={(current_loop.r_center.2-self.z_range.0)*100.0/(self.z_range.1-self.z_range.0)}
                r="5" fill="#ff8300"
                />
            </>
        }
    }

    fn update_current_loop_position(&mut self, client_x: f64, client_y: f64) {
        if let Some(idx) = self.selected_coil {
            let screen_ctm = js! {
                const screenCTM = document.getElementById("arrow_plot").getScreenCTM();
                return [screenCTM.a, screenCTM.b, screenCTM.c, screenCTM.d, screenCTM.e, screenCTM.f];
            };
            let screen_ctm: Vec<f64> = screen_ctm.try_into().unwrap();
            let mut cur_loop = self.current_loops.get_mut(idx).unwrap();
            cur_loop.r_center.0 = ((client_x - screen_ctm[4]) / screen_ctm[0])*(self.x_range.1 - self.x_range.0)/100.0 + self.x_range.0 + cur_loop.radius;
            cur_loop.r_center.2 = ((client_y - screen_ctm[5]) / screen_ctm[3])*(self.z_range.1 - self.z_range.0)/100.0 + self.z_range.0;
            self.magnetic_field =
                calculate_magnetic_field(&self.current_loops, &self.x_range, &self.z_range);
        }
    }
}
