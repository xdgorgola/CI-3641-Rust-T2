# CI-3641-Rust-T2
Implementaciones de los numerales de Church y un árbol binario con ramas y hojas usando Rust.

### NOTA
La función **esMaxHeapSimétrico** se llama **es_max_heap_simetrico**.

## Archivos
### Numerales de Church
El módulo con la implementación de numerales de Church se encuentra en el archivo:

    ./src/church_num/church.rs
    
En este módulo se encuentran las siguientes definiciones:
  - enum Church: Enumerador numeral de Church.
  - fn num_church: Función que calcula un numeral de Church con un entero.
  - fn sucesor: Función que devuelve el sucesor de un numeral de Church.
  - fn suma: Función que calcula la suma de dos numerales de Church.
  - fn multiplicacion: Función que calcula la multiplicación de dos numerales de Church.
   
### Árboles binarios
El módulo con la implementación de árboles binarios se encuentra en el archivo:

    ./src/bin_t/binary_tree.rs

En este módulo se encuentran las siguientes definiciones:
  - enum Arbol<T: Ord>: Enumerador árbol binario.
  - fn es_binario: Función que calcula si un árbol es binario.
  - fn preorder: Función que calcula recorrido preorder de un árbol.
  - fn postorder: Función que calcula recorrido postorder de un árbol.
  - fn es_max_heap_simetrico: Función que calcula si un árbol es max heap simétrico.
  
