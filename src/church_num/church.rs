#[derive(Clone, Debug)]
pub enum Church {
    Cero,
    Suc(Box<Church>)
}


pub fn num_church(a : i32) -> Church
{
    let mut c = Church::Cero;
    for _ in 0..a
    {
        c = sucesor(&c);
    }
    
    return c
}


pub fn sucesor(c : &Church) -> Church
{
    match c {
        Church::Cero => 
        Church::Suc(Box::new(Church::Cero)),
        
        Church::Suc(_) => 
        Church::Suc(Box::new(c.clone())),
    }
}


pub fn suma(a : &Church, b : &Church) -> Church
{
    match (a, b) {
        (Church::Cero, Church::Cero) => Church::Cero,
        (Church::Cero, Church::Suc(_)) => b.clone(),
        (Church::Suc(_), Church::Cero) => a.clone(),
        (Church::Suc(_), Church::Suc(i)) => sucesor(&suma(a, i)),
    }
}


pub fn multiplicacion(a : &Church, b : &Church) -> Church
{
    match (a, b) {
        (Church::Cero, Church::Cero) => Church::Cero,
        (Church::Cero, Church::Suc(_)) => Church::Cero,
        (Church::Suc(_), Church::Cero) => Church::Cero,
        (Church::Suc(_), Church::Suc(i)) => suma(a, &multiplicacion(a, i)),
    }
}
