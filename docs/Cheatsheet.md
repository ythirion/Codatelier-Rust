# Rust cheatsheet

## Table des matières
- [Rust cheatsheet](#rust-cheatsheet)
  - [Table des matières](#table-des-matières)
  - [Quelques commandes utiles](#quelques-commandes-utiles)
  - [Quelques mots sur Rust](#quelques-mots-sur-rust)
  - [Commentaires](#commentaires)
  - [Expression V.S. Déclaration](#expression-vs-déclaration)
  - [Encapsulation](#encapsulation)
  - [Variables](#variables)
  - [Opérations](#opérations)
  - [Les conditions :](#les-conditions-)
    - [Le block `if` :](#le-block-if-)
    - [Le block `match`](#le-block-match)
  - [Boucles](#boucles)
    - [La boucle `loop`](#la-boucle-loop)
    - [La boucle `while`](#la-boucle-while)
    - [La boucle `for`](#la-boucle-for)
    - [Une boucle comme une expression](#une-boucle-comme-une-expression)
  - [Les collections](#les-collections)
    - [Les tuples](#les-tuples)
    - [Les tableaux](#les-tableaux)
    - [Les vecteurs](#les-vecteurs)
    - [Le reste](#le-reste)
  - [Types customisés](#types-customisés)
    - [Les structures](#les-structures)
    - [Les Tuple-structures](#les-tuple-structures)
    - [Les enumérations](#les-enumérations)
    - [Les alias](#les-alias)
  - [Les fonctions](#les-fonctions)
  - [Ownership et borrowing](#ownership-et-borrowing)
  - [L'implémentation](#limplémentation)
  - [Option et Result](#option-et-result)
    - [Option](#option)
    - [Result](#result)
    - [`panic!()`](#panic)


## Quelques commandes utiles

Pour créer un projet répertoire `cargo new <nom>` ou `cargo init .` pour créer un projet dans le répertoire courrant.

Pour tester le projet `cargo test`.

Pour compiler et exécuter le projet `cargo run`.

Pour compiler le projet `cargo build`.

Pour vérifier le projet : `cargo check`.

Pour ajouter une librairie : `cargo add <nom>`.

Pour générer la documentation `cargo doc`.

## Quelques mots sur Rust

- Rust est un langage qui mixe les performances des langages bas niveaux et l'ergonomie des langages modernes et de la programmation orientée objet.

- Rust est un langage compilé donc :
    - Les erreurs sont détectées à la compilation.
    - Le langage est plus performant en temps et en mémoire.

- Rust s'inspire de la programmation fonctionnelle pour:
    - Allouer et libérer la mémoire automatiquement SANS utiliser un collecteur de déchets.
    - Déterminer automatiquement le type des variables. 

- Rust est un langage récent donc :
    - Il embarque un gestionnaire de paquet (cargo) pour faciliter l'utilisation du langage.
    - La documentation de Rust (et de ses principaux frameworks) est exceptionnelle.
    - On peut automatiquement générer la documentation de son code grâce aux commentaires.   

## Commentaires

```rust
// Commentaire sur une ligne

/*
Commentaire
sur plusieurs
lignes
*/

/// Commentaire de documentation
```

## Expression V.S. Déclaration

- Une **Expression** produit une valeur.
- Une **Déclaration** ne produit pas de valeur. 

Exempes:

```rust
let a = 10;   // Une déclaration
5 + 2         // Une expression
```

En rust, pour "renvoyer" une valeur, on peut utiliser le mot clé `return` **OU** que la dernière instruction exécutée ne termine pas par un point virgule :

```rust
fn foo() -> i32 {
    println!("I am a function");
    return 10;
}

// Exactement le même comportement.
fn foo() -> i32 {
    println!("I am a function");
    10
}
```

On peut donc utiliser les "scopes" pour assigner des valeurs. C'est particulièrement utilisé avec les boucles et les conditions (voir [boucles](#boucles) et [conditions](#les-conditions-)).

```rust
// a = 6
let a = {
    let b = 2;
    let c = 3;
    numerator * denominator
};
```

## Encapsulation

Par défaut, les modules (équivalents des 'packages' et des 'namespaces'), les structures, les énumérations, les traits et les fonctions ne sont pas publiques ! Il faut utiliser le mot clé `pub` pour autoriser l'accès depuis un autre module. 

## Variables

Les types de données: 

- Entiers signés : `i8`, `i16`, `i32`, `i64`, `i128`
- Entiers non signés : `u8`, `u16`, `u32`, `u64`, `u128`
- Booléens : `bool`
- Textes : `char`, `&str`, `String`

Déclarer une variable :

- On déclare une variable avec le mot clé `let`.
- Le nom d'une variable respecte la convention snake_case.
- Si une variable n'est pas utilisée, on place un `_` devant le nom pour le pas générer un avertissement de compilation. 
- Rust est en mesure (sauf quelques exceptions) de déterminer le type d'une variable → il n'y a donc pas besoin d'indiquer le type.
- Par défaut, une variable est immutable, c'est-à-dire qu'elle ne peut pas être modifiée. Il faut utiliser le mot clé `mut` pour autoriser la modification.

```
let [mut] <nom>[: type] = <expression>;
```

```rust
let a = 10;
let b: u32 = 100;

let mut c = 'h';
let mut d: bool = false;

let an_used_variable = 10.0;
let _an_unused_variable = -2;
```

- Il est possible de créer une variable de même nom → c'est le `shadowing` :

```rust
let a = 10;
let a = 'h';
```

- Le scoping permet faire un "shadowing" temporaire :

```rust
let a = 10;
println!("{}", a);  // Affiche 10

{
    let a = 1000;
    println!("{}", a);  // Affiche 1000
}

println!("{}", a);  // Affiche 10
```

- Pour "caster" une valeur en un autre type on utilise le mot clé `as` :  

```
<expression> as <type>
```

```rust
let a: u8 = 10; 
let b = a as i32;
```

## Opérations

- Les opérations sur les entiers :

```rust
let a = 10;
let b = 20;

let add = a + b;
let substract = a - b;
let multiply = a * b;
let divide = a / b;
let modulo = a % b;

let mut c = 0;
c += 1;         // c = c + 1
c -= 1;         // c = c - 1
c *= 2;         // c = c * 2
c /= 2;         // c = c / 2
c %= 2;         // c = c % 2
```

- Les opérations binaires :

```rust
let a = 0b0101010;
let b = 0b1010101;

let and = a & b;
let or = a | b;
let xor = a ^ b;
```

- Les opérations sur les booléens :

```rust
let a = true;
let b = false;

let and = a && b;
let or = a || b;
let xor = a ^ b;
let not = !a;
```

## Les conditions :

### Le block `if` :

```
if <expression booléenne> {
    ...
}
```

Il est possible d'ajouter une branche `else` pour exécuter une action si la branche `if` n'est pas exécutée :

```
if <expression booléenne> {
    ...
} else {
    ...
}
```

Il est possible d'enchaîner les blocks if :

```
if <expression booléenne> {
    ...
} else if <expression booléenne> {
    ...
} else {
    ...
}
```

Exemple:

```rust
let a = 10;

if a < 5 {
    println!("A < 5");
} else if a > 5 {
    println!("A > 5");
} else {
    println!("A = 5");
}
```

Un block `if` peut être utilisé pour assigner une valeur (rappel [ici](#expression-vs-déclaration)) :

```rust
let a = true;
let b = a {
    "true"
} else {
    "false"
};
```

### Le block `match`

Le block `match` fonctionne comme un `switch` : 

```
match <expression> {
    <valeur constante> => { ... },
    _ => { ... }
}
```

Exemple:

```rust
let a = 3;

match a {
    5 => println!("A = 5"),
    10 => println!("A = 10"),
    // Default branch
    _ => println!("Other")      
}
```

Un block `match` peut être utilisé pour assigner une valeur (rappel [ici](#expression-vs-déclaration)) :

```rust
let color_code = 'Y';

let color_name = match color_code {
    'Y' => "Yellow",
    'R' => "Red",
    'G' => "Green",
    'B' => "Blue",
    _ => "Other"
};
```

## Boucles

Il y a trois types de boucles en Rust :

### La boucle `loop`

La boucle `loop` est une boucle infinie. 

```rust
loop {
    // instructions
}
```

Le mot clé `continue` permet de passer à l'itération suivante.

Le mot clé `break` permet de sortir de la boucle.

```rust
let mut i = 0;
loop {
    if i == 10 {
        continue;
    }

    if i > 15 {
        break;
    }

    println!("{}", i);

    i += 1;
}
```

### La boucle `while`

La boucle `while` repète des instructions tant que l'expression booléenne s'évalue à vraie :

```
while <expression booléenne> {
    ...
}
```

Le mot clé `continue` permet de passer à l'itération suivante.

Le mot clé `break` permet de sortir de la boucle.

```rust
let mut i = 0;

while i < 15 {
    if i == 10 {
        continue;
    }

    if i == 12 {
        break;
    }

    println!("{}", i);

    i += 1;
}
```

### La boucle `for` 

La boucle `for` permet d'itérer sur les éléments d'une collection (comme la boucle `foreach`) : 

```
for <nom_variable> in <élément_iterable> {
    ...
} 
```

Le mot clé `continue` permet de passer à l'itération suivante.

Le mot clé `break` permet de sortir de la boucle.

```rust 
let values = 0..20;     // == [0; 19]
for i in values {
    if i == 10 {
        continue;
    }
    
    if i == 12 {
        break;
    }

    println!("{}", i);
}
```

### Une boucle comme une expression

Une boucle peut être utilisée pour assigner une valeur (rappel [ici](#expression-vs-déclaration)) :

```rust
let mut i = 120;

let a = loop {
    if i == 0 {
        break true;
    }

    if i == 1 {
        break false;
    }

    i -= 2;
};
```

## Les collections

### Les tuples 

Un tuple permet de grouper plusieurs valeurs entre elles. Ils sont **immutables**.

```rust
let a = (1, true, 'a');

println!("Element 1 is {}", a.0);
println!("Element 2 is {}", a.1);
println!("Element 3 is {}", a.2);
```

Il est possible de déconstruire un tuple :

```rust
let my_tuple = (1, 'a', false);

let (the_int, the_char, the_bool) = my_tuple;
```

### Les tableaux

Les tableaux permettent de stocker plusieurs valeurs de même type. Ils ont une taille fixe.

```rust
let my_array = [1, 2, 3, 4, 5, 6];  

let my_array: [u8; 4];  // Créer un tableau de 4 u8. La valeur des cases du tableau correspondent à la valeur par défaut du type du tableau. Ici la valeur par défaut est 0.


let my_array = ["hello"; 10]; // Créer un tableau de 10 cases avec "hello" comme valeur initiale. 


let first_tile = my_array[0];
let last_tile = my_array[my_array.len() - 1];
```

Pour modifier une case du tableau il faut déclarer le tableau avec le mot clé `mut` (rappel [ici](#variables)).

```rust
let mut my_array = [5; 10];
my_array[4] = 698;
```

### Les vecteurs

Les vecteurs sont comme des tableaux mais ils ont une taille variable. 

```rust
let mut my_vector = vec![1, 2, 3, 4];
let mut my_vector: Vec<i32> = Vec::new();

my_vector.push(10);             // Ajoute la valeur 10 à la fin du vecteur.
let a = my_vector.len();        // Retourne la taille du vecteur.
let b = my_vector.pop();        // Retire et retourne la dernière valeur du vecteur.
```

### Le reste 

Il y a encore beaucoup de collections à présenter. Je vous invite à regarder la documentation pour en apprendre plus ! Voici rapidement les plus importantes :

- Les [Hashmaps](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- Les [Queues](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
- Les [Arbres](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
- Les [Sets](https://doc.rust-lang.org/std/collections/struct.HashSet.html)

## Types customisés

### Les structures

Pour regrouper plusieurs types entre eux, on utilise les structures. Une structure est définie par:
- Le mot `pub` pour autoriser l'accès depuis les autres modules (rappel [encapsulation](#encapsulation)). 
- Un nom (format PascalCase)
- Des attributs `[pub] nom: type` (le nom au format snake_case). Il faut préciser avec le mot clé `mut` si l'attribut de la structure peut être directement accessible.


```
[pub] struct <Nom> {
    [pub] <nom>: <type>,
}
```

```rust
struct MyStruct {
    a_string: String,
    an_int: i32,
    a_bool: bool    
}
```

Pour initialiser une structure : 

```rust
struct Person {
    name: String,
    age: u8,
    is_working: bool
}

let me = Person {
    name: "Thomas Hodson".to_owned(),
    age: 22,
    is_working: true
};
```

Pour accéder à un champs d'une structure, on utilise le charactère '.'. Pour modifier une structure il faut la déclarer comme mutable (rappel [ici](#variables)).

```rust
pub struct Person {
    pub name: String,
    pub age: u8,
    pub is_working: bool
}

let mut me = Person {
    name: "Thomas Hodson".to_owned(),
    age: 22,
    is_working: true
};

println!("Hi ! My name is {}", me.name);
me.is_working = false;
```

### Les Tuple-structures

Les tuples structures sont des structures dont les attributs sont anonymisés. 

```rust
struct Rgb(u8, u8, u8);
let color = Rgb(255, 0, 0);
println!("My color is : RED={} GREEN={}, BLUE={}", color.0, color.1, color.2);
```

### Les enumérations

Les énumérations de Rust sont un mixe entre une énumération et une union de C. Pour déclarer une énumération il faut:
- Le mot `pub` pour autoriser l'accès depuis les autres modules (rappel [encapsulation](#encapsulation)).
- Un nom (au format PascalCase).
- Les variantes de l'énumération (au format PascalCase).

```rust
pub enum Direction {
    North,
    South,
    East,
    West
}

let north = Direction::North;
```

Il est possible d'associer des valeurs dans les variantes d'une énumérations. Les variantes des énumerations fonctionne comme les [tuple-structures](#les-tuple-structures).

```rust
enum MyValue {
    MyString(String),
    MyInt(i32),
    MyBool(bool),
    MyCustom(char, &str, u8)
}

let my_value = MyValue::MyString("Hello".to_owned());
```

On utilise le match pour manipuler une énumération :

```rust
enum MyValue {
    MyString(String),
    MyInt(i32),
    MyBool(bool),
    MyCustom(char, &str, u8)
}

fn foo(value: MyValue) {
    match value {
        MyString(string) => { /* ... */ },
        MyInt(int) => { /* ... */ },
        MyBool(boolean) => { /* ... */ },
        MyCustom(custom) => { /* ... */ }
    }
}
```

### Les alias 

Il est possible de renommer un type :

```rust
type MyAlias = i32;
```

## Les fonctions

Pour déclarer une fonction, il faut :
- le mot clé `pub` pour que la fonction soit publique (rappel [encapsulation](#encapsulation))
- un nom (au format snake_case)
- les paramètres en parenthèses (`<nom>: <type>` au format snake_case).
- Si la fonction retourne une valeur : `-> <type>`

```
[pub] fn <nom>([<nom>: <type>, ...]) [-> <type>] {
    ...
} 
```

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn say_hi(name: String) {
    println!("Hi {}", name);
}

fn get_text() -> String {
    "A super cool text".to_owned()
}
```

Pour demander un paramètre mutable, on ajoute le mot clé `mut` avant le nom :

```rust
fn change(mut val: MyStruct) {
    // ...
}
```

## Ownership et borrowing

En Rust, une valeur ne peut être possédée **que par un seul élément**. C'est ce qu'on appelle le "Ownership". Voici quelques exemples :

- Lorsqu'une valeur est transmise à une fonction, la variable perds la propriété de la valeur :

```rust
let a = 10;             // 'a' est propriétaire de la valeur 10.
foo(a);                 // 'a' donne la valeur à la fonction foo. 'a' n'est plus propriétaire de la valeur 10.

println!("{}", a);      // Erreur de compilation car 'a' ne possède plus aucune valeur. 
```

Alors comment faire pour partager une valeur sans en perdre la propriété ? Il y a deux solutions : 

1. Retransférer la propriété de la valeur (solution très utilisée dans le `builder's pattern`). Cette solution reste cependant limitée :

```rust
fn foo(i: i32) -> i32 {
    println!("{}", i);
    return i;
}

let a = 10;
let a = foo(a);
println!("{}", a); 
```

2. Utiliser le borrowing. Le borrowing consiste à **autoriser l'accès en lecture seule** à une valeur. Il peut y avoir une infinité d'accès en lecture mais il ne doit y avoir au plus qu'un seul accès en écriture. Pour créer un emprunt, on utilise charactère `&`. C'est ce qu'on appelle faire une référence vers une valeur (même principe que les pointeurs en C).

```rust
struct Rgb(u8, u8, u8);

fn take_ownership(rgb: Rgb) { 
    // ...
}

fn borrows(rgb: &Rgb) {
    // ...
}
```

Il est également possible de faire une référence mutable. Celle-ci sont souvent utilisées lorsque l'on manipule les structures : 

```rust
struct Rgb(u8, u8, u8);

fn invert_color(rgb: &mut Rgb) {
    rgb.0 = 255 - rgb.0;
    rgb.1 = 255 - rgb.1;
    rgb.2 = 255 - rgb.2;
}
```

## L'implémentation

Rust permet d'associer des fonctions aux énumérations et aux structures. Ces fonctions deviennent des "méthodes". On utilise le mot clé `self` pour référencer l'instance de la structure / de l'énumération. On utilise le mot clé `Self` pour référec

Pour créer un block d'implémentation, on utilise le mot clé `impl` suivi du nom du type. 

Il existe 5 types de méthodes : 

- Les fonctions statiques qui n'accèdent pas à l'instance. Pour appeler la fonction il faut écrire : `<Type>::<fonction>()`.
- Les fonctions non statiques. Pour appeler uen fonction non statique il faut écrire `<instance>.<fonction>()`: 

    - Les fonctions qui accèdent par référence à l'instance : `&self`.
    - Les fonctions qui accèdent par référence mutable à l'instance : `&mut self`.
    - Les fonctions qui prennent le ownership de l'instance : `self`.
    - Les fonctions qui prennent le ownership mutable de l'instance : `mut self`.


```rust
struct Rgb(u8, u8, u8);

impl Rbg {
    pub fn a_static_function() {
        /* ... */
    }

    pub fn a_reference_function(&self) { 
        /* ... */
    } 

    pub fn a_mutable_reference_function(&mut self) {
        /* ... */
    }

    pub fn a_ownership_function(self) { 
        /* ... */
    } 

    pub fn a_mutable_ownership_function(mut self) {
        /* ... */
    }
}

Rgb::a_static_function();
let mut rgb = Rgb(255, 255, 255);
rgb.a_reference_function()
rgb.a_mutable_reference_function()
rgb.a_ownership_function()
rgb.a_mutable_ownership_function()
```

## Option et Result

### Option

La valeur `null` n'existe pas en Rust. A la place, on utilise l'Option. L'option est une énumération avec deux variantes:

- Some(valeur)
- None

De cette manière Rust garantie qu'il n'y aura jamais de `NullPointerException`. 

```rust
let my_option = Some(10);
let my_option2 = None;
```

Pour manipuler l'option on utilise :
- un `match`

```rust
fn foo(option: Option<i32>) {
    match option {
        Some(val) => { /* ... */ },
        None => { /* ... */ }
    }
}
```

- un `if let`

```rust
fn foo(option: Option<i32>) {
    if let Some(val) = option {
        /* ... */
    } else {
        /* ... */
    }
}
```

- Utiliser les méthodes de l'énum ([documentation](https://doc.rust-lang.org/std/option/)) comme :
    - `unwrap` qui crash le programme si c'est la variante `None`
    - `unwrap_or(value)` qui retourne la valeur de la variante `Some` ou la valeur par défaut donnée. 
    - ...

### Result

Il n'y a pas d'`Exception` en Rust. On utilise l'énumération `Result` avec deux variantes :
- `Ok(value)`
- `Err(error)`

```rust
fn is_adult(age: i32) -> Resut<bool, String> {
    if age < 0 {
        Err("The age must be positive")
    } else {
        Ok(age >= 18)
    }
}
```

`Result` fonctionne comme l'Option ([rappel](#option)). 


### `panic!()`

La macro `panic!()` permet de lever une exception qui stoppera le programme.