//! Modulo que contiene un enumerador que representa una arbol con dos hijos, 
//! y funciones para calcular si un arbol es max heap simetrico
#[derive(Debug)]
pub enum Arbol<T: Ord>
{
    Hoja(T),
    Rama(T, Box<Arbol<T>>, Box<Arbol<T>>)
}


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

impl<T: Ord> Eq for Arbol<T> {}

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

fn es_binario<T: Ord>(raiz: &Arbol<T>) -> bool
{
    match raiz {
        Arbol::Hoja(_) => true,
        Arbol::Rama(_, l, r) =>
        raiz >= l && raiz >= r && es_binario(l) && es_binario(r) 
    }
}


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