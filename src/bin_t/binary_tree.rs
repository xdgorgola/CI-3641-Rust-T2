//! Modulo que contiene un enumerador que representa una arbol con dos hijos, 
//! y funciones para calcular si un arbol es max heap simetrico

/// Enumerador de arbol binario
///
/// Enumerador que representa un arbol binario.
/// Tiene una variante Hoja que solo incluye un valor y 
/// tiene una variante rama que adicionalmente contiene dos
/// enumeradores tipo Arbol (hijos izquierdo y derecho)
#[derive(Debug)]
pub enum Arbol<T: Ord>
{
    Hoja(T),
    Rama(T, Box<Arbol<T>>, Box<Arbol<T>>)
}


/// Implementacion de Trait igualdad parcial para los
/// enumeradores de tipo arbol
impl<T: Ord> PartialEq for Arbol<T>
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            Arbol::Hoja(v) =>
            {
                match other {
                    Arbol::Hoja(vh) => v == vh,
                    Arbol::Rama(vh, _, _) => v == vh
                }
            },
            Arbol::Rama(v, _, _) =>
            {
                match other {
                    Arbol::Hoja(vh) => v == vh,
                    Arbol::Rama(vh, _, _) => v == vh
                }
            }
        }
    }
}

/// Implementacion de Trait igualdad para los
/// enumeradores de tipo arbol
impl<T: Ord> Eq for Arbol<T> {}

/// Implementacion de Trait orden parcial para los
/// enumeradores de tipo arbol
impl<T: Ord> PartialOrd for Arbol<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Arbol::Hoja(v), Arbol::Hoja(v2)) => Some(v.cmp(v2)),
            (Arbol::Hoja(v), Arbol::Rama(v2, _, _)) => Some(v.cmp(v2)),
            (Arbol::Rama(v, _, _), Arbol::Hoja(v2)) => Some(v.cmp(v2)),
            (Arbol::Rama(v, _, _), Arbol::Rama(v2, _, _)) => Some(v.cmp(v2)),
        }
    }
}

/// Implementacion de Trait orden total para los
/// enumeradores de tipo arbol
impl<T: Ord> Ord for Arbol<T> {
    
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Arbol::Hoja(v), Arbol::Hoja(v2)) => v.cmp(v2),
            (Arbol::Hoja(v), Arbol::Rama(v2, _, _)) => v.cmp(v2),
            (Arbol::Rama(v, _, _), Arbol::Hoja(v2)) => v.cmp(v2),
            (Arbol::Rama(v, _, _), Arbol::Rama(v2, _, _)) => v.cmp(v2),
        }
        
    }
}

/// Funcion que calcula si un arbol es binario
/// 
/// Calcula si para cada nodo rama de un arbol, el valor
/// de la rama es mayor que la de sus nodos hijos.
/// 
/// Argumentos:
/// 
/// -raiz: Referencia a la raiz del arbol
/// 
/// Retorna:
/// 
/// true si el arbol es binario, false en caso contrario
fn es_binario<T: Ord>(raiz: &Arbol<T>) -> bool
{
    match raiz {
        Arbol::Hoja(_) => true,
        Arbol::Rama(_, l, r) =>
        raiz >= l && raiz >= r && es_binario(l) && es_binario(r) 
    }
}


/// Funcion que devuelve la secuencia preorder de un arbol
/// 
/// Calcula la secuencia de recorrer un arbol en orden preorder.
/// 
/// Argumentos:
/// 
/// -raiz: Referencia a la raiz del arbol
/// -seq: Vector mutable donde se pondran los elementos de la secuencia
/// 
/// Retorna:
/// 
/// Vector con referencia a los elementos de la secuencia preorder del arbol
fn preorder<'a, T>(raiz: &'a Arbol<T>, mut seq: Vec<&'a T>) -> Vec<&'a T>
where
T: Ord
{
    match raiz
    {
        Arbol::Hoja(v) =>
        {
            seq.push(v);
            return seq;
        },
        Arbol::Rama(v, l, r) => 
        {
            seq.push(v);
            seq = preorder(l, seq);
            seq = preorder(r, seq);
            return seq;
        },
    }
}


/// Funcion que devuelve la secuencia postorder de un arbol
/// 
/// Calcula la secuencia de recorrer un arbol en orden postorder.
/// 
/// Argumentos:
/// 
/// -raiz: Referencia a la raiz del arbol
/// -seq: Vector mutable donde se pondran los elementos de la secuencia
/// 
/// Retorna:
/// 
/// Vector con referencia a los elementos de la secuencia postorder del arbol
fn postorder<'a, T>(raiz: &'a Arbol<T>, mut seq: Vec<&'a T>) -> Vec<&'a T>
where
T: Ord
{
    match raiz
    {
        Arbol::Hoja(v) =>
        {
            seq.push(v);
            return seq;
        },
        Arbol::Rama(v, l, r) => 
        {
            seq = postorder(l, seq);
            seq = postorder(r, seq);
            seq.push(v);
            return seq;
        },
    }
}


/// Funcion que calcula si un arbol es max heap simetrico
/// 
/// Calcula si el arbol es max heap simetrico.
/// 
/// Argumentos:
/// 
/// -raiz: Referencia a la raiz del arbol
/// 
/// Retorna:
/// 
/// true si el arbol es un max heap simetrico, false en caso contrario
pub fn es_max_heap_simetrico<T: Ord>(a: &Arbol<T>) -> bool
{
    if !es_binario(a)
    {
        return false;
    }
    
    let mut va = vec![];
    let mut vb = vec![];
    
    va = preorder(a, va);
    vb = postorder(a, vb);
    
    for n in 0..va.len()
    {
        if va[n] != vb[n]
        {
            return false;
        }
    }
    
    return true;
}