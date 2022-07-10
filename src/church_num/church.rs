//! Modulo que contiene un enumerador que representa numerales de church

/// Enumerador de numeral de church
///
/// Enumerador que representa un numerar de church.
/// 
/// Tiene una variante Cero representando el cero.
/// Tiene una variante Suc que representa el sucesor de un numero de church
#[derive(Clone, Debug)]
pub enum Church {
    Cero,
    Suc(Box<Church>)
}


/// Funcion para obtener un numero de church
/// 
/// Crea un numeral de sucesor conformado por 
/// a sucesiones a Cero
/// 
/// Argumentos:
/// 
/// -a: Numero de sucesiones
/// 
/// Retorna:
/// 
/// Numeral de church compuesto de a sucesiones
pub fn num_church(a : i32) -> Church
{
    let mut c = Church::Cero;
    for _ in 0..a
    {
        c = sucesor(&c);
    }
    
    return c
}


/// Funcion para obtener sucesor de un numeral de church
/// 
/// Crea el sucesor a un numeral de Church
/// 
/// Argumentos:
/// 
/// -c: Referencia de numeral de Church a suceder
/// 
/// Retorna:
/// 
/// Numeral de Church sucesor a el numeral c
pub fn sucesor(c : &Church) -> Church
{
    match c {
        Church::Cero => 
        Church::Suc(Box::new(Church::Cero)),
        
        Church::Suc(_) => 
        Church::Suc(Box::new(c.clone())),
    }
}


/// Funcion suma entre numerales de church
/// 
/// Calcula el numeral de Church resultado de sumar
/// dos numerales de Church
/// 
/// Argumentos:
/// 
/// -a: Referencia de numeral de Church a sumar
/// -b: Referencia a numeral de Church a sumar
/// 
/// Retorna:
/// 
/// Numeral de Church sucesor resultado de sumar a y b
pub fn suma(a : &Church, b : &Church) -> Church
{
    match (a, b) {
        (Church::Cero, Church::Cero) => Church::Cero,
        (Church::Cero, Church::Suc(_)) => b.clone(),
        (Church::Suc(_), Church::Cero) => a.clone(),
        (Church::Suc(_), Church::Suc(i)) => sucesor(&suma(a, i)),
    }
}


/// Funcion multiplicacion entre numerales de church
/// 
/// Calcula el numeral de Church resultado de multiplicar
/// dos numerales de Church
/// 
/// Argumentos:
/// 
/// -a: Referencia de numeral de Church a multiplicar
/// -b: Referencia a numeral de Church a multiplicar
/// 
/// Retorna:
/// 
/// Numeral de Church sucesor resultado de multiplicar a y b
pub fn multiplicacion(a : &Church, b : &Church) -> Church
{
    match (a, b) {
        (Church::Cero, Church::Cero) => Church::Cero,
        (Church::Cero, Church::Suc(_)) => Church::Cero,
        (Church::Suc(_), Church::Cero) => Church::Cero,
        (Church::Suc(_), Church::Suc(i)) => suma(a, &multiplicacion(a, i)),
    }
}
