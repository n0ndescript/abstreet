// Copyright 2018 Google LLC, licensed under http://www.apache.org/licenses/LICENSE-2.0

use aabb_quadtree::geom::Rect;
use ezgui::GfxCtx;
use graphics;
use graphics::math::Vec2d;
use graphics::types::Color;
use map_model;
use map_model::geometry;
use map_model::{BuildingID, Map};
use std::f64;

#[derive(Debug)]
pub struct DrawBuilding {
    pub id: BuildingID,
    polygon: Vec<Vec2d>,
    front_path: Option<[f64; 4]>,
}

impl DrawBuilding {
    pub fn new(bldg: &map_model::Building) -> DrawBuilding {
        let pts: Vec<Vec2d> = bldg.points.iter().map(|pt| [pt.x(), pt.y()]).collect();
        DrawBuilding {
            id: bldg.id,
            // TODO ideally start the path on a side of the building
            front_path: bldg.front_path
                .map(|pair| [pair.0.x(), pair.0.y(), pair.1.x(), pair.1.y()]),
            polygon: pts,
        }
    }

    // TODO it'd be cool to draw a thick border. how to expand a polygon?
    pub fn draw(&self, g: &mut GfxCtx, color: Color) {
        if let Some(line) = self.front_path {
            let path = graphics::Line::new_round([0.0, 0.6, 0.0, 1.0], 1.0);
            path.draw(line, &g.ctx.draw_state, g.ctx.transform, g.gfx);
        }

        let poly = graphics::Polygon::new(color);
        poly.draw(&self.polygon, &g.ctx.draw_state, g.ctx.transform, g.gfx);
    }

    pub fn contains_pt(&self, x: f64, y: f64) -> bool {
        geometry::point_in_polygon(x, y, &self.polygon)
    }

    pub fn tooltip_lines(&self, map: &Map) -> Vec<String> {
        let b = map.get_b(self.id);
        let mut lines = vec![
            format!("Building #{:?} (from OSM way {})", self.id, b.osm_way_id),
        ];
        lines.extend(b.osm_tags.iter().cloned());
        lines
    }

    pub fn get_bbox(&self) -> Rect {
        geometry::get_bbox_for_polygons(&[self.polygon.clone()])
    }
}
