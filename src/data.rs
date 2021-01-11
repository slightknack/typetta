pub enum Data {
    Unit,
    Natural(usize),
    Lambda,
}

pub enum Type {
    Unit,
    Natural,
    Lambda {
        argument:   Box<Type>,
        expression: Box<Type>,
    },

    Pre(usize),
    Var(usize),
}

impl Data {
    pub fn of_data(data: &Data) -> Type {
        match data {
            Data::Unit => {}
            Data::Natural(_) => {}
            Data::Lambda => {}
        }
    }
}

pub struct Typed<T> {
    item: T,
    r#type: Type,
}

impl<T> Typed<T> {
    pub fn new(item: T, r#type: Type) -> Typed<T> {
        Typed { item, r#type }
    }
}
