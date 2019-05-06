use crate::helpers::{ColorScheme, ID};
use crate::render::{should_draw_blinkers, DrawCtx, DrawOptions, Renderable};
use ezgui::{Color, Drawable, GfxCtx, Prerender};
use geom::{Circle, Distance, PolyLine, Polygon};
use map_model::{Map, LANE_THICKNESS};
use sim::{DrawPedestrianInput, PedestrianID};

pub struct DrawPedestrian {
    pub id: PedestrianID,
    circle: Circle,
    turn_arrow: Option<Vec<Polygon>>,
    zorder: isize,

    draw_default: Drawable,
}

impl DrawPedestrian {
    pub fn new(
        input: DrawPedestrianInput,
        map: &Map,
        prerender: &Prerender,
        cs: &ColorScheme,
    ) -> DrawPedestrian {
        // TODO Slight issues with rendering small pedestrians:
        // - route visualization is thick
        // - there are little skips when making turns
        // - front paths are too skinny
        let radius = LANE_THICKNESS / 4.0;

        let turn_arrow = if let Some(t) = input.waiting_for_turn {
            let angle = map.get_t(t).angle();
            Some(
                PolyLine::new(vec![
                    input.pos.project_away(radius / 2.0, angle.opposite()),
                    input.pos.project_away(radius / 2.0, angle),
                ])
                .make_arrow(Distance::meters(0.25))
                .unwrap(),
            )
        } else {
            None
        };

        let circle = Circle::new(input.pos, radius);

        let draw_default = prerender.upload(vec![(
            if input.preparing_bike {
                cs.get_def("pedestrian preparing bike", Color::rgb(255, 0, 144))
                    .shift(input.id.0)
            } else {
                cs.get_def("pedestrian", Color::rgb_f(0.2, 0.7, 0.7))
                    .shift(input.id.0)
            },
            circle.to_polygon(),
        )]);

        DrawPedestrian {
            id: input.id,
            circle,
            turn_arrow,
            zorder: input.on.get_zorder(map),
            draw_default,
        }
    }
}

impl Renderable for DrawPedestrian {
    fn get_id(&self) -> ID {
        ID::Pedestrian(self.id)
    }

    fn draw(&self, g: &mut GfxCtx, opts: &DrawOptions, ctx: &DrawCtx) {
        if let Some(color) = opts.color(self.get_id()) {
            g.draw_circle(color, &self.circle);
        } else {
            g.redraw(&self.draw_default);
        }

        if let Some(ref a) = self.turn_arrow {
            if should_draw_blinkers() {
                g.draw_polygons(ctx.cs.get("blinker on"), a);
            }
        }
    }

    fn get_outline(&self, _: &Map) -> Polygon {
        self.circle.to_polygon()
    }

    fn get_zorder(&self) -> isize {
        self.zorder
    }
}
