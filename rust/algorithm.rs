const FOO_RELATIVE_EQ_EPSILON_MULTIPLIER: DScalar = 2.0;

#[derive(Debug, Clone)]
struct FooCreationData {
    foos: Vec<Id<Foo>>,
    foos_s: Vec<DScalar>,
    position: DVec3,
}

pub fn generate_foos(bar: &mut Bar, config: &processing::Config) -> Result<()> {
    let mut temp_foos = generate_temp_foos(bar, config);

    split_foos(
        bar,
        &mut temp_foos,
        config.foos_relative_eq_epsilon,
    )?;

    let foos: Vec<Foo> =
        convert_temp_foos_to_foos(&temp_foos, bar)?;
    update_foos_in_foos(bar, &foos)?;

    for foo in foos {
        bar.insert_foo(foo)?;
    }

    merge_close_foos(bar, config)?;

    assign_bazs_to_foos(bar, config)?;
    bar.rebuild_curves_rtree();
    Ok(())
}

fn merge_close_foos(bar: &mut Bar, config: &processing::Config) -> Result<()> {
    let foos_to_squash = bar
        .foos_iter()
        .filter(|foo| {
            let curve = bar.curve_by_id(foo.curve);
            curve.length() <= config.distance_to_merge_foos && foo.foos.len() == 2
        })
        .map(|foo| foo.id)
        .collect_vec();

    for &squashed_foo_id in &foos_to_squash {
        let squashed_foo = bar.foo_by_id(squashed_foo_id).clone();
        let mut foo_a = bar
            .foo_by_id(squashed_foo.foos[0])
            .clone();
        let foo_b = bar
            .foo_by_id(squashed_foo.foos[1])
            .clone();
        let mut moved_foos = foo_b
            .foos
            .get()
            .iter()
            .filter(|jr| jr.foo_id != squashed_foo.id)
            .cloned()
            .collect_vec();
        for moved_foo in &moved_foos {
            let mut foo = bar.foo_by_id(moved_foo.foo_id).clone();
            let foo_b_index = foo
                .foos
                .iter()
                .position(|&j_id| j_id == foo_b.id)
                .unwrap();
            foo.foos[foo_b_index] = foo_a.id;
            bar.edit_foo(foo)?;
        }

        moved_foos.extend(
            foo_a
                .foos
                .get()
                .iter()
                .filter(|jr| jr.foo_id != squashed_foo.id)
                .cloned(),
        );
        foo_a.foos = FooFoos::new(moved_foos, bar);
        foo_a.position = (foo_a.position + foo_b.position) * 0.5;

        utils::remove_foo_with_dependencies(bar, squashed_foo_id)?;
        bar.edit_foo(foo_a)?;
        bar.remove_foo(foo_b.id)?;
    }

    Ok(())
}

fn generate_temp_foos(
    bar: &Bar,
    config: &processing::Config,
) -> Vec<FooCreationData> {
    let mut temp_foos = Vec::<FooCreationData>::new();
    let position_epsilon_sqr =
        config.foos_relative_eq_epsilon * config.foos_relative_eq_epsilon;

    for foo in bar.foos_iter() {
        let foo_curve = bar.curve_by_id(foo.curve);
        for node in foo_curve.nodes().iter() {
            try_generate_foo_in_point(
                bar,
                &mut temp_foos,
                node.position,
                position_epsilon_sqr,
            );
        }
    }

    temp_foos
}

fn try_generate_foo_in_point(
    bar: &Bar,
    foos: &mut Vec<FooCreationData>,
    position: DVec3,
    position_epsilon_sqr: DScalar,
) {
    if !is_foo_already_created(foos, position, position_epsilon_sqr) {
        if let Some(j) = search_common_nodes(bar, position, position_epsilon_sqr) {
            if j.foos.len() > 1 {
                foos.push(j);
            }
        }
    }
}

fn is_foo_already_created(
    foos: &[FooCreationData],
    position: DVec3,
    position_epsilon_sqr: DScalar,
) -> bool {
    foos
        .iter()
        .any(|j| j.position.distance2(position) <= position_epsilon_sqr)
}

fn search_common_nodes(
    bar: &Bar,
    searched: DVec3,
    position_epsilon_sqr: DScalar,
) -> Option<FooCreationData> {
    let mut foo_foos: Vec<Id<Foo>> = vec![];
    let mut foos_s = vec![];
    let radius: DScalar = position_epsilon_sqr * FOO_RELATIVE_EQ_EPSILON_MULTIPLIER;

    let curves = bar.find_curves_in_area(radius, searched.truncate());
    if !curves.is_empty() {
        for curve_id in curves {
            let foo_curve = bar.curve_by_id(curve_id);
            let foo = bar.foo_by_main_curve_id(curve_id);

            if let Some(foo) = foo {
                for node in foo_curve.nodes().iter() {
                    if node.position.distance2(searched) < position_epsilon_sqr {
                        foo_foos.push(foo.id());
                        foos_s.push(foo_curve.query_closest_s_to_point(searched));
                    }
                }
            }
        }
    }

    if foo_foos.len() > 1 {
        Some(FooCreationData {
            foos: foo_foos,
            foos_s,
            position: searched,
        })
    } else {
        None
    }
}

fn split_foos(
    bar: &mut Bar,
    jcd: &mut Vec<FooCreationData>,
    epsilon: DScalar,
) -> Result<()> {
    let mut jcd_to_remove: Vec<usize> = vec![];

    for i in 0..jcd.len() {
        for j in 0..jcd[i].foos.len() {
            let foo_id = jcd[i].foos[j];
            let foo = bar.foo_by_id(foo_id).clone();
            let foo_curve = bar.curve_by_id(foo.curve).clone();
            let foo_position = jcd[i].position;
            let split_s = jcd[i].foos_s[j];

            if split_s > epsilon && split_s < foo_curve.length() - epsilon {
                let splitted_foos_result =
                    split::split_foo_at_point(foo_id, foo_position, bar);
                if let Err(err) = splitted_foos_result {
                    warn!(
                        "Unable to split foo with id {:?}. Skipping. Cause:\n {:?}",
                        foo_id,
                        err.display_chain().to_string()
                    );
                    jcd_to_remove.push(i);
                    continue;
                }

                let (foo_a, foo_b) = splitted_foos_result.unwrap();
                update_current_foo(jcd, i, j, foo_a, foo_b, split_s);
                update_rest_of_foos(jcd, foo_id, foo_a, foo_b, split_s, (i, j));
                update_bazs(bar, foo_id, foo_a, foo_b)?;
            }
        }
    }

    jcd_to_remove.sort();
    for i in jcd_to_remove.into_iter().rev() {
        jcd.swap_remove(i);
    }
    Ok(())
}

fn update_current_foo(
    jcd: &mut Vec<FooCreationData>,
    i: usize,
    j: usize,
    foo_a: Id<Foo>,
    foo_b: Id<Foo>,
    split_s: DScalar,
) {
    jcd[i].foos[j] = foo_a;
    jcd[i].foos_s[j] = split_s;
    jcd[i].foos.push(foo_b);
    jcd[i].foos_s.push(0.0);
}

fn update_rest_of_foos(
    jcds: &mut Vec<FooCreationData>,
    removed_foo: Id<Foo>,
    foo_a: Id<Foo>,
    foo_b: Id<Foo>,
    split_s: DScalar,
    current_indices: (usize, usize),
) {
    for (i, jcd) in jcds.iter_mut().enumerate() {
        for j in 0..jcd.foos.len() {
            if (i, j) == current_indices {
                continue;
            }

            if jcd.foos[j] == removed_foo {
                if jcd.foos_s[j] < split_s {
                    jcd.foos[j] = foo_a;
                } else if jcd.foos_s[j] > split_s {
                    jcd.foos[j] = foo_b;
                    jcd.foos_s[j] -= split_s
                }
            }
        }
    }
}

fn convert_temp_foos_to_foos(
    foos_creation_data: &[FooCreationData],
    bar: &Bar,
) -> Result<Vec<Foo>> {
    Ok(foos_creation_data
        .iter()
        .map(|j| Foo {
            id: Id::new(),
            position: j.position,
            foos: FooFoos::new(
                j.foos
                    .iter()
                    .enumerate()
                    .map(|(i, r)| FooFoo {
                        foo_id: *r,
                        contact_point: get_node_position_on_foo(*r, j.foos_s[i], bar),
                        turn_directions: vec![],
                    })
                    .collect(),
                bar,
            ),
            lanes_connectors: Vec::new(),
            signal_group: None,
            area: Id::new(),
        })
        .collect())
}

fn get_node_position_on_foo(
    foo_id: Id<Foo>,
    foo_foo_s: DScalar,
    rs: &Bar,
) -> ContactPoint {
    let foo = rs.foo_by_id(foo_id);
    let curve = rs.curve_by_id(foo.curve);

    if foo_foo_s <= curve.length() * 0.5 {
        ContactPoint::Start
    } else {
        ContactPoint::End
    }
}

fn update_foos_in_foos(bar: &mut Bar, foos: &[Foo]) -> Result<()> {
    for foo in foos {
        for foo_foo in foo.foos.get() {
            let mut foo = bar.foo_by_id(foo_foo.foo_id).clone();
            foo.foos.push(foo.id);
            bar.edit_foo(foo)?;
        }
    }
    Ok(())
}

fn update_bazs(
    bar: &mut Bar,
    old_foo_id: Id<Foo>,
    foo_a_id: Id<Foo>,
    foo_b_id: Id<Foo>,
) -> Result<()> {
    let mut foo_a = bar.foo_by_id(foo_a_id).clone();
    let foo_a_curve_length = bar.curve_by_id(foo_a.curve).length();
    foo_a.bazs.clear();
    bar.edit_foo(foo_a)?;

    let mut foo_b = bar.foo_by_id(foo_b_id).clone();
    foo_b.bazs.clear();
    bar.edit_foo(foo_b)?;

    let old_foo_bazs = bar
        .bazs_iter()
        .filter(|c| c.foo == old_foo_id)
        .cloned()
        .collect_vec();

    old_foo_bazs.into_iter().try_for_each(|mut c| {
        if c.foo_s > foo_a_curve_length {
            c.foo_s -= foo_a_curve_length;
            c.foo = foo_b_id;
            let mut chagned_foo = bar.foo_by_id(foo_b_id).clone();
            chagned_foo.bazs.push(c.id);
            bar.edit_foo(chagned_foo)?;
        } else {
            c.foo = foo_a_id;
            let mut chagned_foo = bar.foo_by_id(foo_a_id).clone();
            chagned_foo.bazs.push(c.id);
            bar.edit_foo(chagned_foo)?;
        }
        bar.edit_baz(c)
    })
}

fn assign_bazs_to_foos(
    bar: &mut Bar,
    config: &processing::Config,
) -> Result<()> {
    let bazs = bar.bazs_iter().cloned().collect_vec();
    let min_baz_to_foo_distance_sq = config
        .distance_to_qualify_baz_as_part_of_foo
        * config.distance_to_qualify_baz_as_part_of_foo;

    bazs.into_iter().try_for_each(|mut c| {
        let foo = bar.foo_by_id(c.foo).clone();
        c.foo = foo
            .foos
            .iter()
            .filter_map(|j_id| {
                let distance_sq =
                    baz_to_foo_distance_squared(bar, &foo, *j_id, c.foo_s);
                let foo = bar.foo_by_id(*j_id);

                if foo.foos.get().len() <= 2 {
                    None
                } else if distance_sq < min_baz_to_foo_distance_sq {
                    Some((*j_id, distance_sq))
                } else {
                    None
                }
            })
            .min_by(|(_, distance_0), (_, distance_1)| distance_0.partial_cmp(distance_1).unwrap())
            .map(|(j_id, _)| j_id);

        bar.edit_baz(c)
    })
}

fn baz_to_foo_distance_squared(
    bar: &Bar,
    foo: &Foo,
    foo_id: Id<Foo>,
    baz_foo_s: DScalar,
) -> DScalar {
    let foo = bar.foo_by_id(foo_id);

    (bar
        .curve_by_id(foo.curve)
        .point_at_distance(baz_foo_s)
        - foo.position)
        .magnitude2()
}
