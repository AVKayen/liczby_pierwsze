## Metody Znajdowania Ilości Liczb Pierwszych w zbiorze $\mathbb N$ od $1$ do $n$
Definicja: 
$\mathbb P = \{ p \in \mathbb N \; | \; \text{p jest liczbą pierwszą}\}$

### Metoda 1 (Naiwna):
Algorytm sprawdza w pętli, czy $k\%i = 0$, gdzie $i\in\mathbb N\cap\{2, k\}, k\in\{1, n\}$

```rust
let mut c: usize = 0;
for i in 2..=n {
    let mut s: bool = true;
    for j in 2..i {
        if i%j == 0 {
            s = false;
            break;
        }
    }
    if s {
        c+=1;
    }
}
return c
```

### Metoda 2 (Naiwna z pominięciem liczb parzystych):
Algorytm sprawdza w pętli, czy $k\%i = 0$,  gdzie $i\in\mathbb N\cap\{3, k\}, i\%2=1$. Pomija on wszystkie liczby parzyste, automatycznie ignorując je ($2k\notin\mathbb P$)

```rust
let mut c: usize = 1;
for i in 3..=n {
    let mut s: bool = true;
    if i%2 == 0 {
        continue;
    }
    for j in (3..i).step_by(2) {
        if i%j == 0 {
            s = false;
            break;
        }
    }
    if s {
        c+=1;
    }
}
return c
```

### Metoda 3 (Naiwna z pominięciem liczb parzystych oraz $i\geqslant\sqrt{k}$):
Algorytm sprawdza w pętli, czy $k\%i = 0$,  gdzie $i\in\mathbb N\cap\{3, \sqrt{k}\}, i\%2=1$. Pomija on wszystkie liczby parzyste, automatycznie ignorując je ($2k\notin\mathbb P$). Pomija on dodatkowo wszystkie $i\geqslant\sqrt{k}$, gdyż nie mogą one być dzielnikami $k$

```rust
let mut c: usize = 1;
for i in 3..=n {
    let mut s: bool = true;
    if i%2 == 0 {
        continue;
    }
    let mut j:usize = 3;
    while j*j <= i {
        if i%j == 0 {
            s = false;
            break;
        }
        j += 2;
    }
    if s {
        c+=1;
    }
}
return c
```

### Metoda 4 (Sito Erastotenesa)
Algorytmowi jest dana tablica $vec$ wypełniona $1$ (wartości prawdziwe). $vec[1]$ jest ustawione na $0$, gdyż $1\notin\mathbb P$. Algorytm po kolei idzie po tablicy $vec$, a gdy napotka wartość prawdziwą, neguje wszystkie wielokrotności tejże liczby. Pod koniec wykonywania algorytmu spełniona jest zależność $vec[i] = 1 \Leftrightarrow i \in\mathbb P$
```rust
let mut c: usize = 0;
let mut vec: Vec<bool> = vec![true; n];
let mut j: usize = 1;
vec[1] = false;
while j*j <= n {
    if vec[j] {
        let mut i: usize = 2;
        while i*j < n {
            vec[i*j] = false;
            i += 1;
        }
    }
    j += 1;
}
for i in 1..n {
    if vec[i] {
        c+=1;
    }
}
return c
```

### Metoda 5 (Sito Atkina)
```rust
let mut c: usize = 0;
let mut vec: Vec<bool> = vec![false; n+1];
vec[2] = true;
vec[3] = true;
let mut x: usize = 1;
while x*x <= n {
    let mut y: usize = 1;
    while y*y <= n {
        let mut t: usize = (4*x*x) + (y*y);
        if t <= n && (t % 12 == 1 || t % 12 == 5) {
            vec[t] ^= true;
        }
        t = (3*x*x) + (y*y);
        if t <= n && t % 12 == 7 {
            vec[t] ^= true;
        }
        if x > y {
            t = (3*x*x) - (y*y);
            if t <= n && t % 12 == 11 {
                vec[t] ^= true;
            }
        }
        y += 1;
    }
    x += 1;
}
let mut pow: usize = 5;
while pow*pow <= n {
    if vec[pow] {
        for i in (pow*pow..n).step_by(pow*pow) {
            vec[i] = false;
        }
    }
    pow+= 1;
}
for i in 1..=n {
    if vec[i] {
        c+=1;
    }
}
return c
```

## Złożoność Obliczeniowa i Czas Wykonania dla poszczególnego $n$
||Metoda 1|Metoda 2|Metoda 3|Metoda 4|Metoda 5|
|-|-|-|-|-|-|
|Złożoność Czasowa|$\approx O(n^2)$|$\approx O(n\cdot\frac{n}{2})$|$\approx O(n\cdot\sqrt{n})$|$O(n\cdot\log{log{(n)}})$|$O(n)$|
|Czas dla $n=1000$   |$360\mu s$|$223\mu s$|$11\mu s$|$25\mu s$|$25\mu s$|
|Czas dla $n=10000$  |$28ms$|$16ms$|$161\mu s$|$255\mu s$|$214\mu s$|
|Czas dla $n=100000$ |$2s$|$1.2s$|$2915\mu s$|$2926\mu s$|$2159\mu s$|
|Czas dla $n=1000000$|$172s$|$102s$|$62ms$|$29ms$|$23ms$|