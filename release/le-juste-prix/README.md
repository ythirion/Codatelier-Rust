# 🎲 Le juste prix

Bienvenue dans ton premier programme Rust ! Tu vas créer un jeu du juste prix pour découvrir les concepts de base du langage.

- [🎲 Le juste prix](#-le-juste-prix)
  - [Lancement ⏳ 30 sec](#lancement--30-sec)
  - [Les macros](#les-macros)
  - [Déclarons nos variables ⏳ 2 min](#déclarons-nos-variables--2-min)
  - [Créer la boucle de jeu ⏳ 30 sec](#créer-la-boucle-de-jeu--30-sec)
  - [Demander son input à l'utilisateur ⏳ 2 min](#demander-son-input-à-lutilisateur--2-min)
  - [Assainir la saisie utilisateur ⏳ 2 min](#assainir-la-saisie-utilisateur--2-min)
  - [\&str, String, wtf ?](#str-string-wtf-)
  - [Transformer la saisie utilisateur en nombre ⏳ 5 min](#transformer-la-saisie-utilisateur-en-nombre--5-min)
  - [Et si…](#et-si)
  - [La gestion des erreurs](#la-gestion-des-erreurs)
  - [Sécuriser la boucle de jeu ⏳ 2 min](#sécuriser-la-boucle-de-jeu--2-min)
  - [Comparer les deux valeurs ⏳ 1 min](#comparer-les-deux-valeurs--1-min)
  - [Le pattern matching](#le-pattern-matching)
  - [Comparer les deux valeurs : pattern matching ⏳ 2 min](#comparer-les-deux-valeurs--pattern-matching--2-min)
  - [Borrow, mutable borrow, ownership](#borrow-mutable-borrow-ownership)
    - [Le magasin de livres](#le-magasin-de-livres)
    - [Le « mutable borrow »](#le--mutable-borrow-)
    - [Le « borrow » : les règles](#le--borrow---les-règles)
  - [Lancement ⏳ 30 sec](#lancement--30-sec-1)
  - [🎉 Bien joué !](#-bien-joué-)


## Lancement ⏳ 30 sec

Dans un terminal, lance la commande `cargo run`.

```bash
cargo run
```

> [!TIP]
> 🐛 Lance-le pour voir l'état initial du projet (avec des bugs).

---

## Les macros

Les macros écrivent du code qui écrit du code : c'est de la métaprogrammation. Le code « macro » est injecté au moment de la compilation. Elles permettent d'éviter la réplication de code, ou des sets de fonctions « utilitaires », et d'ajouter des comportements à des fonctions, des structures, des traits, etc.

**Macro déclarative :**

```rust
// demande au compilateur de considérer le bloc de
// code suivant comme "valide"
todo!();

// imprime sur la sortie standard
println!("Hello World!");

// formatte une chaîne de caractère
format!("Date du jour: {}", Utc::now())
```

**Macro dérivative :**

```rust
#[derive(Serialize, Deserialise)]
struct User {
    login: String,
    password: String
}
```

---

## Déclarons nos variables ⏳ 2 min

Pour le jeu du juste prix, il nous faut deux variables :

- `random_number` : le nombre choisi par l'ordinateur entre 1 et 100 (qui ne bougera pas). On peut utiliser le résultat de la fonction `generate_random_number_between` qui est fournie.
- `found` : la variable qui détermine si nous sommes ou non encore dans la boucle de jeu — si l'utilisateur a trouvé la valeur aléatoire ou pas (qui sera amenée à changer).

**Syntaxe :**

```rust
// variable qui ne va pas changer (constante)
let <nom>[:<type>] = <valeur>;

// variable qui va changer
let mut <nom>[:<type>] = <valeur>;
```

<details>
<summary>Solution</summary>

```rust
// récupérer un nombre aléatoir enetre 1 & 100
let random_number = generate_random_number_between(1, 100);

// créer un mutex pour sortir de la boucle de jeu
let mut found = false;
```
</details>

---

## Créer la boucle de jeu ⏳ 30 sec

Maintenant que les variables initiales du jeu sont posées, il faut poser la boucle de jeu. On crée une boucle dont on ne sortira que lorsque l'on aura trouvé le bon chiffre.

```rust
while ... {
    ...
}
```

<details>
<summary>Solution</summary>

```rust
while !found {
    // ...
}
```
</details>

---

## Demander son input à l'utilisateur ⏳ 2 min

Pour démarrer la boucle de jeu, il faut demander un nombre à l'utilisateur. On imprime la question sur la sortie standard, puis on utilise la fonction `get_input_from_user` pour récupérer sa saisie dans une variable `guess`.

> [!TIP]
> Tu as déjà un texte d'imprimé sur la sortie standard en haut de la fonction main

<details>
<summary>Solution</summary>

```rust
println!("Quel est le juste prix ?");

let guess = get_input_from_user();
```
</details>

---

## Assainir la saisie utilisateur ⏳ 2 min

La valeur saisie par l'utilisateur est une chaîne de caractères, or il nous faut une valeur numérique pour la comparaison. De plus, une saisie utilisateur peut avoir des espaces involontaires. On commence par « trimer » l'entrée utilisateur et ranger cette chaîne « trimée » dans une variable.

> [!TIP]
> Le type `String` possède une méthode `trim`.

<details>
<summary>Solution</summary>

```rust
let trimmed = guess.trim();
```
</details>
  
> [!NOTE]
> 🤔 Le type retourné par `trim()` est `&str`, pas `String`. Cf. section suivante.

---

## &str, String, wtf ?

- `String` représente une chaîne de caractères.
- Une chaîne de caractères est en fait un tableau (Vecteur) de caractères.
- Un caractère se représente dans la mémoire comme une suite de 8 bit, soit un type `u8`.
- `&str` est donc un pointeur direct vers la valeur contenue dans une chaîne déclarée dynamiquement.

```rust
// en interne, une String est un vecteur
// (tableau chaîné en mémoire)
// d'entier sur 8bits, qui constituent
// des caractères.
pub struct String {
    vec: Vec<u8>,
}

// de manière interne, Rust va déclarer un
// vecteur de u8 dans la mémoire de la taille
// de la chaîne déclaré dynamiquement (directement
// entre quote), et retourner le pointeur
// direct vers ce vecteur.
//
//        v-- (& = référence/pointeur)
// d'où le &str <-- (str = marqueur de chaîne dynamique)
let my_str = "Hello world ! 🦀";
```

---

## Transformer la saisie utilisateur en nombre ⏳ 5 min

Maintenant que nous avons nettoyé la saisie utilisateur, il nous faut la transformer en « nombre » pour comparaison. On utilise la fonction `parse()` sur la variable trimée pour transformer le `&str` en `u32`.

> Il existe deux manières de faire !
> ChatGPT ou StackOverflow pourront t'aider ;)


<details>
<summary>Solution</summary>
Les deux manières

```rust
// inférence par type de variable
let parsed: u32 = trimed.parse().unwrap();

// inférence par syntaxe turbofish
let parsed = trimed.parse::<u32>().unwrap();
```
</details>

---

## Et si…

Et si on essayait de rentrer un nombre invalide, par exemple « abc » ?

> [!IMPORTANT]  
> 🐛 Le programme plante ! → cf. section suivante sur la gestion des erreurs.

---

## La gestion des erreurs

En Rust, il n'y a pas d'exception. Pour gérer les erreurs et les traiter proprement, on encapsule un résultat incertain dans un `Result`.

Un `Result` est une énumération, c'est-à-dire un ensemble de possibilités. Une fois le code donnant le résultat incertain, la variable contenant un `Result` sera soit `Ok(T)` en cas de réussite, soit `Err(E)` en cas d'erreur. Un `Result` **doit** être traité ; le compilateur n'autorisera pas le build si un `Result` n'est pas traité correctement.

```rust
// Résultat possible incertain :
//   - Soit T, le résultat attendu en cas de succès
//   - Soit E, l'erreur produite lors de l'opération
enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

**Récupérer le résultat :**

```rust
let my_result = op_risky().unwrap();
// va ranger le résultat de op_risky si le
// résultat est correcte, ou planter le programme
// si c'est une erreur
```

**Traitement avec `if let` :**

```rust
let uncertain_result = op_risky();

if let Err(error) = uncertain_result {
    // traiter l'erreur
}

// on peut utiliser unwrap car on a déjà
// validé que ce n'était pas une erreur
let certain_result = uncertain_result.unwrap();
```

Ou

```rust
let uncertain_result = op_risky();

if let Ok(safe_result) = uncertain_result {
    // traiter uniquement le cas de succès
}
```

---

## Sécuriser la boucle de jeu ⏳ 2 min

On utilise la syntaxe `if let…` pour ne traiter l'entrée utilisateur que si elle est valide (elle est bien parsée en type numérique). On peut imprimer le résultat à l'utilisateur pour confirmer sa saisie. Dans le cas inverse, on ne traite simplement pas le cas et on recommence la boucle. On peut aussi ajouter un message d'erreur en cas de saisie erronée.

```rust
if let ... {
    ...
} else {
    ...
}
```

<details>
<summary>Solution</summary>

```rust
if let Ok(num) = guess.trim().parse::<u32>() {
    print!("Tu proposes : {num}");

    // ...
} else {
    println!("ERREUR: saisie invalide !");
}
```
</details>

---

## Comparer les deux valeurs ⏳ 1 min

On crée le code qui va comparer la valeur `random_value` (valeur de l'ordinateur) avec `num` (valeur saisie par l'utilisateur et transformée).

- Si `random_value > num` : C'est plus !
- Si `random_value < num` : C'est moins !
- Si `random_value == num` : C'est gagné ! → Il faut sortir de la boucle de jeu.

<details>
<summary>Solution</summary>
Avec `if`/`else`

```rust
print!("Tu proposes : {num}");

if random_number > num {
    println!("C'est plus !");
} else if random_number < num {
    println!("C'est moins !");
} else {
    println!("C'est gagné !");
    found = true;
}
```
</details>

---

## Le pattern matching

Rust possède un système de pattern matching pour effectuer des actions selon une valeur donnée par une énumération. Ce pattern matching doit traiter tous les cas possibles, ou en définir un par défaut (avec `_`). Par exemple, il peut être appliqué aux `Result` pour traiter le cas correct et le cas d'erreur.

```rust
match ... {
    pattern => ...,
    pattern => ...,
    _ => ...,
}
```

```rust
match guess.trim().parse::<u32>() {
    Ok(value) => ...,
    Err(error) => ...,
}
```

---

## Comparer les deux valeurs : pattern matching ⏳ 2 min

On utilise la structure `match` pour comparer `num` à `random_number` en utilisant la fonction `cmp(&value)` sur `num`.

`cmp` peut avoir trois valeurs :
- `Ordering::Less`
- `Ordering::Greater`
- `Ordering::Equal`

<details>
<summary>Solution</summary>

```rust
match num.cmp(&random_number) {
    Ordering::Less    => println!(" -> C'est plus !"),
    Ordering::Greater => println!(" -> C'est moins !"),
    Ordering::Equal   => {
        println!(" -> Gagné !");
        found = true;
    }
}
```
</details>
  
> [!IMPORTANT]  
> 🤔 Pourquoi `&random_number` ? → cf. section suivante sur le borrow.

---

## Borrow, mutable borrow, ownership

Rust possède un système de références permettant de garantir un code sans fuite mémoire. Une référence est un lien direct vers l'adresse mémoire d'une valeur, et non le contenu de la valeur directe. Cette garantie est mise en place par le compilateur qui va valider quelques règles du « borrow » sur chaque référence dans le code.

### Le magasin de livres

```rust
let neuromancer = Book {};
// le propriétaire du contenu du livre est neuromancer.

alice_look_at(&neuromancer);
// alice regarde le livre, consulte le contenu, mais sans
// l'acheter, c'est un "borrow".
//
// neuromancer est toujours propriétaire du contenu du livre.

bob_look_at(&neuromancer);
// bob regarde le livre, consulte le contenu, mais sans
// l'acheter, c'est un autre "borrow".
//
// neuromancer est toujours propriétaire du contenu du livre.

sell_to_charly(neuromancer);
// cette fois-ci, neuromancer change de propriétaire, car ce
// n'est pas la référence marque par "&" qui est donné, mais
// bien directement la valeur.
//
// C'est un changement de "ownership", aussi appelé un "move".

dany_look_at(&neuromancer);
// ERROR : neuromancer ayant changé de propriétaire, il ne peut
// plus être consulté
```

### Le « mutable borrow »

```rust
let neuromancer = Manuscript {};
// neuromancer est propriétaire du contenu du manuscrit du livre.

let ace_books = Editor {};
let molly =  Editor {};

edit(&mut neuromancer, ace_books);
// ici, ace_books se réserve le droit de réécrire le manuscrit,
// on dit qu'il fait un "mutable borrow".
//
// Rust stipule qu'il ne peut y avoir qu'un seul mutable borrow
// dans la vie d'une variable.

edit(&mut neuromancer, molly);
// Error: il ne peut y avoir qu'un seul mutable borrow par variable

sell(neuromancer);
// neuromancer est vendu, le propriétaire change, son contenu n'est
// donc plus accessible.

edit(&mut neuromancer, ace_books);
// neuromancer est déjà vendu (changé de propriétaire), son contenu
// ne peut plus être modifié
```

### Le « borrow » : les règles

- Il peut y avoir autant de références (emprunt) que voulu à une variable donnée.
- Il ne peut y avoir qu'une seule référence mutable par variable donnée.
- Il ne peut pas y avoir d'emprunt mutable et immutable en même temps.
- Une fois qu'une variable change de propriétaire (passage direct : « move »), elle n'est plus utilisable.

---

## Lancement ⏳ 30 sec

Dans un terminal, lance la commande `cargo run`.

```bash
cargo run
```

---

## 🎉 Bien joué !
