// lifetime subtyping
pub struct Context<'s>(&'s str);

pub struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    pub fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

#[allow(dead_code)]
fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

// lifetime bounds
#[allow(dead_code)]
struct Ref<'a, T: 'a>(&'a T);

#[allow(dead_code)]
struct StaticRef<T: 'static>(&'static T);

// trait object lifetimes
trait Foo {}

#[allow(dead_code)]
struct Bar<'a> {
    x: &'a i32,
}

impl<'a> Foo for Bar<'a> {}

fn main() {
    let num = 5;

    #[allow(unused_variables)]
    let obj = Box::new(Bar { x: &num }) as Box<Foo>;
}
