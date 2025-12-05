mod ingredient {
    use std::ops;

    enum IngredientCondition {
        Fresh,
        Spoiled,
    }

    struct IngredientId(usize);
    impl IngredientId {
        fn as_usize(self) -> usize {
            self.0
        }
    }

    struct Ingredient {
        id: IngredientId,
        condition: IngredientCondition,
    }
    struct FreshIngredientIDRange(ops::RangeInclusive<IngredientId>);
    impl FreshIngredientIDRange {
        fn as_usize(self) -> ops::RangeInclusive<usize> {
            self.0.start().as_usize()..self.0.end()
        }
    }

    fn sprawdzam_range() {
        let id1 = IngredientId(1);
        let id2 = IngredientId(2);
        let r = id1..=id2;
        let fresh_range = FreshIngredientIDRange(r);
    }
}
