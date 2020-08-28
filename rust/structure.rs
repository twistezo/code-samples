#[derive(Clone, Debug)]
pub struct Foo {
    pub id: Id<Foo>,
    pub bar: Bar,
    pub baz: Baz,
    pub quxulated: bool,
}

impl Foo {
    pub fn new(id: Id<Foo>, bar: Bar, baz: Baz) -> Self {
        Self {
            id,
            bar,
            baz,
            quxulated: true,
        }
    }
}

impl VisualDebug for Foo {
    fn add_debug_primitives(&self, out_primitives: &mut Vec<DebugPrimitive>) {
        self.baz.add_debug_primitives(out_primitives);
        self.bar.add_debug_primitives(out_primitives);
    }
}

#[derive(Debug, Default)]
pub struct FoosManager {
    foos: Vec<Foo>,
    foos_map: HashMap<Id<Foo>, usize>,
    recently_added: Vec<Id<Foo>>,
    recently_removed: Vec<Id<Foo>>,
}

impl FoosManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(qux: &mut QuxContainer, dt: DScalar) -> Result<()> {
        qux.foos.recently_added.clear();
        qux.foos.recently_removed.clear();
        qux.foos.foos = qux
            .foos
            .foos
            .par_iter()
            .map(|a| Self::update_foo(qux, a, dt))
            .collect::<Result<Vec<Foo>>>()?;

        let foos_to_remove = qux
            .foos
            .foos
            .iter()
            .filter(|a| a.baz.is_ready_to_remove())
            .map(|a| a.id)
            .collect_vec();

        for foo_id in foos_to_remove {
            qux.foos.remove(foo_id)?;
        }
        Ok(())
    }

    pub fn foos(&self) -> &[Foo] {
        &self.foos
    }

    pub fn foos_mut(&mut self) -> &mut [Foo] {
        &mut self.foos
    }

    pub fn find_by_id(&self, id: Id<Foo>) -> Option<&Foo> {
        self.foos_map.get(&id).map(|&index| &self.foos[index])
    }

    pub fn by_id(&self, id: Id<Foo>) -> &Foo {
        self.find_by_id(id).unwrap_or_else(|| {
            panic!(
                "FoosManger::by_id: foo with given id {:?} does not exist.\n",
                id
            )
        })
    }

    pub fn by_id_mut(&mut self, id: Id<Foo>) -> &mut Foo {
        let index = *self.foos_map.get(&id).unwrap_or_else(|| {
            panic!(
                "FoosManger::by_id: foo with given id {:?} does not exist.\n",
                id
            )
        });
        &mut self.foos[index]
    }

    pub fn add(&mut self, foo: Foo) -> Result<&Foo> {
        debug!("Adding foo {:?}", foo.id);

        if self.foos_map.contains_key(&foo.id) {
            bail!(ErrorKind::FooIdAlreadyInUse(foo.id));
        }

        self.recently_added.push(foo.id);
        self.foos_map.insert(foo.id, self.foos.len());
        self.foos.push(foo);
        Ok(self.foos.last().unwrap())
    }

    pub fn remove(&mut self, foo_id: Id<Foo>) -> Result<()> {
        debug!("Removing foo {:?}", foo_id);
        let index = *self
            .foos_map
            .get(&foo_id)
            .ok_or_else(|| Error::from(ErrorKind::FooNotFound(foo_id)))?;

        self.foos.swap_remove(index);
        self.foos_map
            .remove(&foo_id)
            .expect("Internal qux error: FoosManager is corrupted.\n");

        if index < self.foos_map.len() {
            let new_id_at_index = self.foos[index].id;

            self.foos_map
                .insert(new_id_at_index, index)
                .expect("Internal qux error: FoosManager is corrupted.\n");
        }

        self.recently_removed.push(foo_id);
        Ok(())
    }

    pub fn recently_added(&self) -> &[Id<Foo>] {
        &self.recently_added
    }

    pub fn recently_removed(&self) -> &[Id<Foo>] {
        &self.recently_removed
    }

    pub fn force_clear_recent_foo_changes(&mut self) {
        self.recently_added.clear();
        self.recently_removed.clear();
    }

    fn update_foo(
        qux: &QuxContainer,
        previous_foo_state: &Foo,
        dt: DScalar,
    ) -> Result<Foo> {
        let mut foo = previous_foo_state.clone();

        if !foo.quxulated {
            return Ok(foo);
        }

        Baz::update(&mut foo, qux, dt)?;
        foo.bar.update(dt, &qux.road_system)?;
        Ok(foo)
    }
}
