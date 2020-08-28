type Index<T> = FnvHashMap<Id<T>, usize>;
type FoodIndex<T> = FnvHashMap<Id<Foo>, Id<T>>;

#[derive(Default, Debug, Clone)]
pub struct Cache {
    pub foos: Index<Foo>,
    pub bars: Index<Bar>,
    pub bazs: Index<Baz>,
    pub quxs: Index<Qux>,
    pub quuxs: Index<Quux>,
    pub quxs_connectors: Index<QuxConnector>,
    pub corges: Index<Corge>,
    pub graults: Index<Grault>,
    pub graults_groups: Index<GraultGroup>,
    pub garplys: Index<Garply>,
    pub auxiliary_objects: Index<AuxiliaryObject>,
    pub foos_to_bazs: FoodIndex<Baz>,
    pub foos_to_quxs: FoodIndex<Qux>,
    pub foos_to_quxs_connectors: FoodIndex<QuxConnector>,
    pub foos_rtree: FoosRTree,
    pub entry_quxs: Vec<Id<Qux>>,
}

impl Cache {
    pub fn new(data: &BazSystemData) -> Self {
        Self {
            foos: Cache::build_index(&data.geometry.foos),
            bars: Cache::build_index(&data.geometry.bars),
            bazs: Cache::build_index(&data.bazs),
            quxs: Cache::build_index(&data.quxs),
            quuxs: Cache::build_index(&data.quuxs),
            quxs_connectors: Cache::build_index(&data.quxs_connectors),
            corges: Cache::build_index(&data.corges),
            graults: Cache::build_index(&data.graults),
            graults_groups: Cache::build_index(&data.graults_groups),
            garplys: Cache::build_index(&data.garplys),
            auxiliary_objects: Cache::build_index(&data.auxiliary_objects),
            foos_to_bazs: Cache::build_food_index(&data.bazs),
            foos_to_quxs: Cache::build_food_index(&data.quxs),
            foos_to_quxs_connectors: Cache::build_food_index(&data.quxs_connectors),
            foos_rtree: FoosRTree::new(&data.geometry.foos),
            entry_quxs: Cache::collect_entry_quxs(&data.quxs),
        }
    }

    fn build_index<T>(items: &[T]) -> Index<T>
    where
        T: Entity,
    {
        items
            .iter()
            .enumerate()
            .map(|(num, item)| (item.id(), num))
            .collect()
    }

    fn build_food_index<T: Food + Entity>(items: &[T]) -> FoodIndex<T> {
        items.iter().map(|item| (item.foo(), item.id())).collect()
    }

    fn collect_entry_quxs(quxs: &[Qux]) -> Vec<Id<Qux>> {
        quxs
            .iter()
            .filter(|l| l.from.is_empty() && l.qux_type == QuxType::Driving)
            .map(|l| l.id)
            .collect()
    }

    pub fn rebuild_foos_to_bazs_index(&mut self, bazs: &[Baz]) {
        self.foos_to_bazs = Cache::build_food_index(bazs);
    }

    pub fn rebuild_foos_to_quxs_index(&mut self, quxs: &[Qux]) {
        self.foos_to_quxs = Cache::build_food_index(quxs);
    }

    pub fn rebuild_foos_to_quxs_connectors_index(&mut self, quxs_connectors: &[QuxConnector]) {
        self.foos_to_quxs_connectors = Cache::build_food_index(quxs_connectors);
    }

    pub fn recollect_entry_quxs(&mut self, quxs: &[Qux]) {
        self.entry_quxs = Cache::collect_entry_quxs(quxs);
    }
}
