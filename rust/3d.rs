fn create_foo(bar: barHandle, baz: &Baz, window: &mut Window) -> FooMesh {
    let foo_type = bar.foo_type(baz);
    let mut group = window.add_group();
    let mut foo_mesh = group.add_cube(
        foo_type.width as f32,
        foo_type.height as f32,
        foo_type.length as f32,
    );

    if bar.foo_state(baz).has_beacon_active {
        group.set_color(1.0, 0.0, 0.0);
    } else if foo_type.emergency_type.is_some() {
        group.set_color(0.0, 0.0, 1.0);
    }

    foo_mesh.set_local_translation(na::Translation3::new(
        0.0,
        0.0,
        -foo_type.height as f32 * 0.5,
    ));

    FooMesh { bar, mesh: group }
}

pub fn clicked_foo(
    bar: &Bar,
    camera: &Camera,
    window: &Window,
) -> Option<FooHandle> {
    let max_distance_pixels_sqr = 10000.0;
    let cursor_f64 = window.cursor_pos().unwrap();
    let cursor = Point2::new(cursor_f64.0 as f32, cursor_f64.1 as f32);
    let size_u32 = window.size();
    let size = Vector2::new(size_u32.x as f32, size_u32.y as f32);

    bar
        .foos_iter()
        .map(|a| {
            let position = &a.baz_state(bar).transform.position;
            let position_on_screen = Point2::from(camera.project(&to_na_point3(position), &size));
            let distance_on_screen_sqr = (position_on_screen - cursor).norm_squared();
            (a, distance_on_screen_sqr)
        })
        .filter(|(_, distance_sqr)| *distance_sqr < max_distance_pixels_sqr)
        .min_by(|(_, distance_a), (_, distance_b)| distance_a.partial_cmp(distance_b).unwrap())
        .map(|(a, _)| a)
}

fn sample_foo(&self, s: DScalar) -> SampledPoint {
    let (position, tangent, offset) = self.curve.sample_at_distance(s);
    let binormal = binormal_from_tangent(tangent);

    SampledPoint {
        s,
        foo: position
            + binormal * (offset + self.offset_at_width)
            + DVec3::new(0.0, 0.0, self.z_offset),
        binormal,
    }
}

fn generate_foo(&mut self, points: &[SampledPoint]) {
    let half_width = 0.5 * self.width;

    for (sampled_a, sampled_b) in points.iter().tuple_windows() {
        let i_0 = self.mesh.vertex_buffer.len() as Index;

        self.mesh.vertex_buffer.push(Vertex {
            position: (sampled_a.point - sampled_a.binormal * half_width)
                .cast()
                .unwrap(),
            normal: Vec3::new(0.0, 0.0, 1.0),
            uv: Vec2::new(sampled_a.s as Scalar, 0.0),
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
        });

        self.mesh.vertex_buffer.push(Vertex {
            position: (sampled_a.point + sampled_a.binormal * half_width)
                .cast()
                .unwrap(),
            normal: Vec3::new(0.0, 0.0, 1.0),
            uv: Vec2::new(sampled_a.s as Scalar, 1.0),
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
        });

        self.mesh.vertex_buffer.push(Vertex {
            position: (sampled_b.point - sampled_b.binormal * half_width)
                .cast()
                .unwrap(),
            normal: Vec3::new(0.0, 0.0, 1.0),
            uv: Vec2::new(sampled_b.s as Scalar, 0.0),
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
        });

        self.mesh.vertex_buffer.push(Vertex {
            position: (sampled_b.point + sampled_b.binormal * half_width)
                .cast()
                .unwrap(),
            normal: Vec3::new(0.0, 0.0, 1.0),
            uv: Vec2::new(sampled_b.s as Scalar, 1.0),
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
        });

        self.mesh.index_buffer.push(i_0);
        self.mesh.index_buffer.push(i_0 + 1);
        self.mesh.index_buffer.push(i_0 + 2);
        self.mesh.index_buffer.push(i_0 + 1);
        self.mesh.index_buffer.push(i_0 + 3);
        self.mesh.index_buffer.push(i_0 + 2);
    }
}
