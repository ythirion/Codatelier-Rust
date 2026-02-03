# Le mastermind

Nous allons créer ensemble un mastermind en Rust. Voici le programme:

- [Le mastermind](#le-mastermind)
  - [Les règles du jeu](#les-règles-du-jeu)
  - [Lancement ⏳ 30 sec](#lancement--30-sec)
  - [Petit tour rapide du projet ⏳ 5 min](#petit-tour-rapide-du-projet--5-min)
  - [Définition des drapeaux ⏳ 2 min](#définition-des-drapeaux--2-min)
  - [Définition d'une proposition de l'utilisateur ⏳ 6 min](#définition-dune-proposition-de-lutilisateur--6-min)
    - [Le code proposé ⏳ 2 min](#le-code-proposé--2-min)
    - [Les drapeaux du code proposé ⏳ 2 min](#les-drapeaux-du-code-proposé--2-min)
  - [Définition d'une partie ⏳ 10 min](#définition-dune-partie--10-min)
    - [Le code secret ⏳ 2 min](#le-code-secret--2-min)
    - [Les tentatives de l'utilisateur ⏳ 2 min](#les-tentatives-de-lutilisateur--2-min)
    - [Indicateur de fin de partie ⏳ 2 min](#indicateur-de-fin-de-partie--2-min)
    - [Indicateur de partie en cours ⏳ 2 min](#indicateur-de-partie-en-cours--2-min)
  - [Générer un code secret ⏳ 5 min](#générer-un-code-secret--5-min)
  - [Créer une nouvelle partie ⏳ 2 min](#créer-une-nouvelle-partie--2-min)
  - [Créer une tentative évaluée ⏳ 2 min](#créer-une-tentative-évaluée--2-min)
  - [Vérifier si un caractère correspond à une bille autorisée ⏳ 5 min](#vérifier-si-un-caractère-correspond-à-une-bille-autorisée--5-min)
  - [Vérifier si la bille est à la bonne position ⏳ 5 min](#vérifier-si-la-bille-est-à-la-bonne-position--5-min)
  - [Vérifier si le code secret est découvert ⏳ 5 min](#vérifier-si-le-code-secret-est-découvert--5-min)
  - [Traiter une tentative de code  ⏳ 10 min](#traiter-une-tentative-de-code---10-min)
  - [Il est l'heure de jouer !](#il-est-lheure-de-jouer-)


## Les règles du jeu

Petit rappel des règles du jeu : 

- L'objectif du mastermind est de découvir un code secret.
- Le code est composé de n billes qui peuvent être : 🟢 vertes (`green`), 🔴 rouges (`red`), 🔵 bleues (`blue`), 🟡 jaunes (`yellow`), ⚫ noires (`black`) ou ⚪ blanches (`whites`). Il peut y avoir plusieurs billes de la même couleur. 
- Tour par tour, le joueur propose une combinaison de billes. L'adversaire (l'ordinateur dans notre cas) compare la proposition du joueur avec le code secret et donne une réponse à l'aide de drapeaux (n billes = n drapeaux). 
    - Le drapeau rouge indique que la bille est bien placée.
    - Le drapeau blanc indique que la bille est dans le code mais n'est pas correctement placée.
    - Le drapeau noir indique que la bille n'est pas dans le code. 
- Lorsque le joueur trouve le code la partie s'arrête. 

**Pour notre jeu**, nous allons simplifier quelques règles pour faciliter le développement. Libre à vous de continuer le projet pour l'améliorer !

- Nous allons utiliser des codes à 4 billes (et donc des réponses à 4 drapeaux). 
- Le joueur peut proposer autant de code qu'il le souhaite (pas de limites).
- L'ordre des drapeaux correspond à l'ordre des billes. 


## Lancement ⏳ 30 sec

Placez-vous dans le répertoire du projet : 

```
cd atelier/mastermind
cargo build 
```

S'il y a des erreurs c'est normal ! Les IAs font grèves et ne veulent pas compléter notre code... Il va falloir le faire à la *Old School* ! 

> [!TIP]
> 🐛 Lance-le pour voir l'état initial du projet (avec des bugs).


## Petit tour rapide du projet ⏳ 5 min

Cette fois le code est séparé en plusieurs répertoires et fichiers. Pas de panique ! Je vais vous expliquer : 

Nous vous proposons un projet qui peut être exécuté comme une application console **ET** comme une application web ! (*Je sais on est vraiment des devs super cools 😎*)

L'application est structurée comme ceci:

```
mastermind/
├─ target/      ← (le bazar du compiler rustc : on s'en occupe pas)   
├─ dist/        ← (le bazar du 'compiler' trunk  : on s'en occupe pas)  
├─ Cargo.lock   ← (le bazar de cargo : on s'en occupe pas)  
│
│ (A partir d'ici c'est notre problème) 
│
├─ Cargo.toml   ← (la configuration du projet rust)   
├─ Trunk.toml   ← (la configuration de trunk)
├─ index.html   ← (la page html de notre application (oui il n'y a pas grand chose))
├─ styles.css   ← (un peu de css pour rendre notre web app présentable (on est pas des sauvages !))
│
│ (A partir d'ici c'est le code)
│
└─ src/
    ├─ bin/             ← (nos deux applications : console et web)
    │   ├─ cli.rs       ← (application console)
    │   └─ web.rs       ← (application web)
    ├─ cli/
    │   ├─ display.rs   ← (affichage dans la console)
    │   ├─ runner.rs    ← (gestion des entrées utilisateurs et de la boucle du jeu)
    │   └─ mod.rs       ← (définit le répertoire cli/ comme un module)
    ├─ components/
    │   ├─ ball.rs              ← (composant Html représentant une bille)
    │   ├─ code_attempt.rs      ← (composant Html représentant une proposition de code)
    │   ├─ flag.rs              ← (composant Html représentant un drapeau)
    │   ├─ game_over.rs         ← (composant Html indiquant que la partie est terminée)
    │   ├─ game.rs              ← (composant Html du jeu)
    │   ├─ guess_ball.rs        ← (composant Html représentant un sélectionneur de bille)
    │   ├─ guess_code.rs        ← (composant Html représentant un sélectionneur de code)
    │   ├─ start_game.rs        ← (composant Html pour lancer une partie)
    │   └─ mod.rs               ← (définit le répertoire components/ comme un module)
    ├─ app.rs       ← (composant racine de l'application web)
    ├─ game.rs      ← (logique du jeu)
    └─ lib.rs       ← (notre projet est une librairie (pratique courrante en Rust))
```

Nous avons de la chance, seul le fichier `src/game.rs` semble être incomplet. 

## Définition des drapeaux ⏳ 2 min

Avant de s'attaquer à la logique du jeu, il faut savoir de quoi on parle. On commence par définir nos drapeaux. Un drapeau est un indicateur à trois états (*mince moi qui voulais utiliser un booléen...*). La solution... l'énumération ! 

Une énumération est un type customisé qui permet de définir une liste d'état. Pour définir une énumération il faut : 

- Un nom en CamelCase.
- Des variantes (les états) en CamelCase également.

```
[pub] enum <NomEnCamelCase> {
    <VarianteEnCamelCase>,
    <AutreVarianteEnCamelCase>,
    ...
}
```

**A vous de jouer !**, nous avons besoin de représenter 3 drapeaux : 
1. Le drapeau rouge (`RightPosition`)
1. Le drapeau blanc (`MisPlaced`)
1. Le drapeau noir (`Invalid`)

```rust
/// Résultat du comparatif entre une bille proposée et le code secret.
#[derive(PartialEq, Clone)]
pub enum Flag {
    /// La bille est bonne et bien placée.
    /// La bille est bonne mais mal placée.
    /// La bille n'est pas dans le code secret.
}
```

> [!tip]
> [Cheatsheet](../../docs/Cheatsheet.md)

<details>
<summary>Solution</summary>

```rust
/// Résultat du comparatif entre une bille proposée et le code secret.
#[derive(PartialEq, Clone)]
pub enum Flag {
    /// La bille est bonne et bien placée.
    RightPosition,
    /// La bille est bonne mais mal placée.
    MisPlaced,
    /// La bille n'est pas dans le code secret.
    Invalid,
}
```
</details>

## Définition d'une proposition de l'utilisateur ⏳ 6 min

Pendant la partie, l'utilisateur propose des codes et l'ordinateur joue à [l'agent de piste](https://s2.qwant.com/thumbr/474x253/4/9/7f23a92a2229ea4091c6630ae0dfd521904f7cec13b99aa1a76be77fa8bcaa/OIP.mc5YDpJUJMYcQKBsh_WYhAHaD9.jpg?u=https%3A%2F%2Ftse2.explicit.bing.net%2Fth%2Fid%2FOIP.mc5YDpJUJMYcQKBsh_WYhAHaD9%3Fpid%3DApi&q=0&b=1&p=0&a=0) avec ses drapeaux. Il nous faut donc stocker le code proposer et la combinaison de drapeaux résulante. 

On utilise une structure pour combiner plusieurs types entre eux afin de créer un nouveau type. Pour déclarer une structure il faut :

- Un nom en CamelCase
- Des attributs déclarés avec:
    - Un nom en snake_case
    - un type

```
[pub] struct <MyStructureName> {
    [pub] <my_attribute_name>: <type>,
    ...
}
```

> [!tip]
> [Cheatsheet](../../docs/Cheatsheet.md)

Pour représenter une proposition de code, on définit la structure `CodeAttempt` avec :

- Un attribut qui contient le code proposé.
- Un attribut qui contient la combinaison de drapeaux associés. 

### Le code proposé ⏳ 2 min

Notre code secret est un code à 4 billes. Pour le représenter, nous allons utiliser un tableau. Pour définir un tableau il faut préciser la taille et le type : 

```
[<type>; <taille>]
```

Vous avez surement remarquer que plus haut dans le code il y a déjà un tableau déclaré avec les couleurs des billes :

```rust
// Les 6 billes possibles (green, red, blue, yellow, black, white).
const AVAILABLE_BALLS: [char; 6] = ['g', 'r', 'b', 'y', 'k', 'w'];
```

Nous allons utiliser le même procédé pour notre code. **A vous de jouer !** Il faut ajouter un attribut `attempt` à la structure `CodeAttempt` qui correspond au code proposé par l'utilisateur :

```rust
/// Une proposition de l'utilisateur et son évaluation.
#[derive(PartialEq, Clone)]
pub struct CodeAttempt {
    /// Proposition saisie par le joueur.
    /// Résultat associé: un flag par bille.
}
```

<details>
<summary>Solution</summary>

```rust
/// Une proposition de l'utilisateur et son évaluation.
#[derive(PartialEq, Clone)]
pub struct CodeAttempt {
    /// Proposition saisie par le joueur.
    pub attempt: [char; 4],
    /// Résultat associé: un flag par bille.
}
```
</details>

### Les drapeaux du code proposé ⏳ 2 min

Même procédé que pour le code proposé mais avec des drapeaux ! **A vous de jouer !** Il faut ajouter un attribut `result` à la structure `CodeAttempt` :

```rust
/// Une proposition de l'utilisateur et son évaluation.
#[derive(PartialEq, Clone)]
pub struct CodeAttempt {
    /// Proposition saisie par le joueur.
    pub attempt: [char; 4],
    /// Résultat associé: un flag par bille.
}
```

<details>
<summary>Solution</summary>

```rust
/// Une proposition de l'utilisateur et son évaluation.
#[derive(PartialEq, Clone)]
pub struct CodeAttempt {
    /// Proposition saisie par le joueur.
    pub attempt: [char; 4],
    /// Résultat associé: un flag par bille.
    pub result: [Flag; 4],
}
```
</details>

## Définition d'une partie ⏳ 10 min

Qu'est-ce qu'une partie ? *C'est une question très importante ! En maths c'est ... Qu'est-ce que je raconte moi... On a pas le temps pour ça... Uhm reprenons.* Pour représenter une partie dans notre programme nous allons utiliser la structure `Game` qui va:

- Stocker le code secret.
- Stocker les tentatives de l'utilisateur. 
- Indiquer si le code a été trouvé.
- Indiquer si la partie est en cours (pour le mode CLI).

### Le code secret ⏳ 2 min


Comme pour la structure `CodeAttempt`, nous allons utiliser un tableau pour stocker le code secret. **A vous de jouer !** Il faut ajouter l'attribut `code` à la structure `Game`.

```rust
/// État d'une partie de Mastermind.
#[derive(PartialEq, Clone)]
pub struct Game {
    /// Code secret de 4 billes.
    /// Historique des tentatives.
    /// True si le code a été trouvé.
    /// Active pour le mode CLI interactif.
}
```

<details>
<summary>Solution</summary>

```rust
/// État d'une partie de Mastermind.
#[derive(PartialEq, Clone)]
pub struct Game {
    /// Code secret de 4 billes.
    pub code: [char; 4],
    /// Historique des tentatives.
    /// True si le code a été trouvé.
    /// Active pour le mode CLI interactif.
}
```
</details>


### Les tentatives de l'utilisateur ⏳ 2 min

On souhaite conserver un historique des tentatives de l'utilisateur. On représente une tentative avec la structure `CodeAttempt` mais il faut pouvoir en stocker plusieurs. Le problème... combien est-ce qu'il faut en stocker ? Eh bien oui, sans cette information on ne peut pas utiliser un tableau ! L'utilisateur peut être très chanceux et gagner au premier tour ou être vraiment mauvais et avoir besoin de 100 tours (*évidemment, je ne parle pas par expérience ...*) !

La solution ? Le vecteur (`Vec<Type>`). C'est un tableau à taille dynamique. Voici comment on l'utilise :

```rust
// Déclaration à l'aide d'une macro
let my_vec = vec![1, 2, 3, 4];

// Déclaration à l'aide d'une méthode
let mut my_vec: Vec<i32> = Vec::new();

// Ajouter un élément à la fin du vecteur.
my_vec.push(10);
```

**A vous de jouer !** Il faut ajouter un attribut `attempts` à la structure `Game` qui contient l'historique des tentatives de l'utilisateur :

```rust
/// État d'une partie de Mastermind.
#[derive(PartialEq, Clone)]
pub struct Game {
    /// Code secret de 4 billes.
    pub code: [char; 4],
    /// Historique des tentatives.
    /// True si le code a été trouvé.
    /// Active pour le mode CLI interactif.
}
```

<details>
<summary>Solution</summary>

```rust
/// État d'une partie de Mastermind.
#[derive(PartialEq, Clone)]
pub struct Game {
    /// Code secret de 4 billes.
    pub code: [char; 4],
    /// Historique des tentatives.
    pub attempts: Vec<CodeAttempt>,
    /// True si le code a été trouvé.
    /// Active pour le mode CLI interactif.
}
```
</details>


### Indicateur de fin de partie ⏳ 2 min

Pour indiquer si la partie est terminée ou non, on utilise un booléen. **A vous de jouer !** Il faut ajouter un attribut `is_game_over` dans la structure `Game` pour indiquer si la partie est terminée :


```rust
/// État d'une partie de Mastermind.
#[derive(PartialEq, Clone)]
pub struct Game {
    /// Code secret de 4 billes.
    pub code: [char; 4],
    /// Historique des tentatives.
    pub attempts: Vec<CodeAttempt>,
    /// True si le code a été trouvé.
    /// Active pour le mode CLI interactif.
}
```

<details>
<summary>Solution</summary>

```rust
/// État d'une partie de Mastermind.
#[derive(PartialEq, Clone)]
pub struct Game {
    /// Code secret de 4 billes.
    pub code: [char; 4],
    /// Historique des tentatives.
    pub attempts: Vec<CodeAttempt>,
    /// True si le code a été trouvé.
    pub is_game_over: bool,
    /// Active pour le mode CLI interactif.
}
```
</details>

### Indicateur de partie en cours ⏳ 2 min

Même procédé. **A vous de jouer !** Il faut ajouter un attribut `is_game_active` dans la structure `Game` pour indiquer si la partie est toujours en cours :

```rust
/// État d'une partie de Mastermind.
#[derive(PartialEq, Clone)]
pub struct Game {
    /// Code secret de 4 billes.
    pub code: [char; 4],
    /// Historique des tentatives.
    pub attempts: Vec<CodeAttempt>,
    /// True si le code a été trouvé.
    pub is_game_over: bool,
    /// Active pour le mode CLI interactif.
}
```

<details>
<summary>Solution</summary>

```rust
/// État d'une partie de Mastermind.
#[derive(PartialEq, Clone)]
pub struct Game {
    /// Code secret de 4 billes.
    pub code: [char; 4],
    /// Historique des tentatives.
    pub attempts: Vec<CodeAttempt>,
    /// True si le code a été trouvé.
    pub is_game_over: bool,
    /// Active pour le mode CLI interactif.
    pub is_game_active: bool,
}
```
</details>

## Générer un code secret ⏳ 5 min

Maintenant que l'on a définit l'ensemble de nos éléments, il est temps de passer à la logique du jeu. Nous allons commencer par écrire une fonction pour générer un code aléatoire. Comment allons nous procéder ? 

- On va utiliser la constante `AVAILABLE_BALLS` pour disposer des billes.
- On va mélanger les billes et en sélectionner 4. 

**A vous de jouer !** Il faut commencer par créer une variable mutable à partir de la constante `AVAILABLE_BALLS` :

```rust
/// Tire un code secret aléatoire de 4 billes distinctes.
fn create_random_code() -> [char; 4] {
    todo!();
    // On copie les billes disponibles pour pouvoir les mélanger sans toucher à la constante.
    // Générateur de nombres aléatoires fourni par rand 0.9.
    // Mélange aléatoire in-place pour tirer 4 billes uniques.
    // On retourne les 4 premières billes du tableau mélangé.
}
```

<details>
<summary>Solution</summary>

```rust
/// Tire un code secret aléatoire de 4 billes distinctes.
fn create_random_code() -> [char; 4] {
    // On copie les billes disponibles pour pouvoir les mélanger sans toucher à la constante.
    let available_balls = AVAILABLE_BALLS;
    // Générateur de nombres aléatoires fourni par rand 0.9.
    // Mélange aléatoire in-place pour tirer 4 billes uniques.
    // On retourne les 4 premières billes du tableau mélangé.
}
```
</details>

<br /> 
Il faut ensuite mélanger aléatoirement le tableau. Nous allons utiliser le module `rand`. Voici comment cela fonctionne :

```rust
// Initialisation du module rand
let mut rnd = rand::rng();
// mélange le tableau.
tableau.shuffle(&mut rng);
```

**A vous de jouer !** Il faut mélanger le tableau contenant les billes :

```rust
/// Tire un code secret aléatoire de 4 billes distinctes.
fn create_random_code() -> [char; 4] {
    // On copie les billes disponibles pour pouvoir les mélanger sans toucher à la constante.
    let available_balls = AVAILABLE_BALLS;
    // Générateur de nombres aléatoires fourni par rand 0.9.
    // Mélange aléatoire in-place pour tirer 4 billes uniques.
    // On retourne les 4 premières billes du tableau mélangé.
}
```


> [!TIP]
> N'oubliez pas d'autoriser la modification du tableau.

<details>
<summary>Solution</summary>

```rust
/// Tire un code secret aléatoire de 4 billes distinctes.
fn create_random_code() -> [char; 4] {
    // On copie les billes disponibles pour pouvoir les mélanger sans toucher à la constante.
    let mut available_balls = AVAILABLE_BALLS;
    // Générateur de nombres aléatoires fourni par rand 0.9.
    let mut rng = rand::rng();
    // Mélange aléatoire in-place pour tirer 4 billes uniques.
    available_balls.shuffle(&mut rng);
    // On retourne les 4 premières billes du tableau mélangé.
}
```
</details>

<br /> 

Enfin, il faut sélectionner les 4 billes qui composent le code secret. 

**A vous de jouer !** Il faut retourner un tableau contenant 4 billes :

```rust
/// Tire un code secret aléatoire de 4 billes distinctes.
fn create_random_code() -> [char; 4] {
    // On copie les billes disponibles pour pouvoir les mélanger sans toucher à la constante.
    let mut available_balls = AVAILABLE_BALLS;
    // Générateur de nombres aléatoires fourni par rand 0.9.
    let mut rng = rand::rng();
    // Mélange aléatoire in-place pour tirer 4 billes uniques.
    available_balls.shuffle(&mut rng);
    // On retourne les 4 premières billes du tableau mélangé.
}
```

<details>
<summary>Solution</summary>

```rust
/// Tire un code secret aléatoire de 4 billes distinctes.
fn create_random_code() -> [char; 4] {
    // On copie les billes disponibles pour pouvoir les mélanger sans toucher à la constante.
    let mut available_balls = AVAILABLE_BALLS;
    // Générateur de nombres aléatoires fourni par rand 0.9.
    let mut rng = rand::rng();
    // Mélange aléatoire in-place pour tirer 4 billes uniques.
    available_balls.shuffle(&mut rng);
    // On retourne les 4 premières billes du tableau mélangé.
    [
        available_balls[0],
        available_balls[1],
        available_balls[2],
        available_balls[3],
    ]
}
```
</details>

## Créer une nouvelle partie ⏳ 2 min

On va maintenant écrire une fonction pour créer une nouvelle partie. Pour créer une instance d'une structure on utilise la syntaxe suivante : 

```rust 
pub fn new(param1, param2, param3) -> Self {
    Self {
        attribut1: param1,
        attribut2: param2,
        attribut3: param3,
    }
}
```

**A vous de jouer !** Il faut écrire la fonction pour créer une nouvelle partie :

```rust
/// Construit une partie avec un nouveau code et aucun historique.
pub fn new(is_game_active: bool) -> Self {
    todo!();
    // Crée une partie avec un nouveau code secret et aucun historique.
}
```

<details>
<summary>Solution</summary>

```rust
/// Construit une partie avec un nouveau code et aucun historique.
pub fn new(is_game_active: bool) -> Self {
    // Crée une partie avec un nouveau code secret et aucun historique.
    Self {
        code: create_random_code(),
        attempts: vec![],
        is_game_over: false,
        is_game_active,
    }
}
```
</details>

## Créer une tentative évaluée ⏳ 2 min

**A vous de jouer !** Il faut écrire une fonction pour créer une nouvelle instance d'une tentative évaluée (`CodeAttempt`) :

```rust
/// Construit une tentative évaluée.
pub fn new(attempt: [char; 4], result: [Flag; 4]) -> Self {
    todo!();
    // Simple constructeur data-only.
}
```

<details>
<summary>Solution</summary>

```rust
/// Construit une tentative évaluée.
pub fn new(attempt: [char; 4], result: [Flag; 4]) -> Self {
    // Simple constructeur data-only.
    Self { attempt, result }
}
```
</details>

## Vérifier si un caractère correspond à une bille autorisée ⏳ 5 min

Nous allons écrire une fonction pour vérifier si un caractère correspond à une bille autorisée. 

**A vous de jouer !** Il faut compléter la fonction :

```rust
/// Valide qu'un caractère correspond à une bille autorisée.
pub fn is_valid_char(c: char) -> bool {
    todo!();
    // Vérifie si la bille proposée fait partie de l'alphabet autorisé.
}
```

> [!TIP]
> Il y a une [cheatsheet](../../docs/Cheatsheet.md) dans ce repo.

<details>
<summary>Solution</summary>

```rust
/// Valide qu'un caractère correspond à une bille autorisée.
pub fn is_valid_char(c: char) -> bool {
    // Vérifie si la bille proposée fait partie de l'alphabet autorisé.
    AVAILABLE_BALLS.contains(&c)
}
```
</details>

## Vérifier si la bille est à la bonne position ⏳ 5 min

Nous allons écrire une méthode pour l'énumération drapeau `Flag` pour vérifier si une bille est à la bonne position. 

**A vous de jouer !** Il faut compléter la méthode :

```rust
/// Indique si la bille est à la bonne position.
pub fn is_right_position(&self) -> bool {
    // Pratique pour tester si un flag correspond à une bille parfaitement placée.
    matches!(self, Flag::RightPosition)
}
```

> [!TIP]
> Il y a une [cheatsheet](../../docs/Cheatsheet.md) dans ce repo.

<details>
<summary>Solution</summary>

```rust
/// Indique si la bille est à la bonne position.
pub fn is_right_position(&self) -> bool {
    // Pratique pour tester si un flag correspond à une bille parfaitement placée.
    matches!(self, Flag::RightPosition)
}
```
</details>

## Vérifier si le code secret est découvert ⏳ 5 min

Nous allons écrire une méthode pour la structure `CodeAttempt` pour vérifier si le code secret est trouvé.

**A vous de jouer !** Il faut compléter la méthode :

```rust
/// Vrai si les 4 flags indiquent une victoire.
pub fn is_game_over(&self) -> bool {
    todo!();
    // La partie est gagnée si les 4 flags sont RightPosition.
}
```

> [!TIP]
> Pensez bien à utiliser les fonctions créées précédemment. 

<details>
<summary>Solution</summary>

```rust
/// Vrai si les 4 flags indiquent une victoire.
pub fn is_game_over(&self) -> bool {
    // La partie est gagnée si les 4 flags sont RightPosition.
    self.result.iter().all(|flag| flag.is_right_position())
}
```
</details>

## Traiter une tentative de code  ⏳ 10 min

Nous allons écrire la dernière fonction du programme ! Cette fonction va traiter une tentative de code et mettre à jour la partie. 

Dans un premier temps, il faut analyser la proposition de code. 

**A vous de jouer !** Il faut créer un code qui génère la combinaison de drapeaux :

```rust
/// Traite une tentative de code proposée par le joueur.
pub fn process_code_attempt(&self, attempt: [char; 4]) -> Self {
    todo!()

    // Prépare un tableau de flags par défaut pour chaque bille proposée.
    // On le remplit de Flag::Invalid pour pouvoir le modifier ensuite.

    // Compare chaque bille proposée avec le code secret:
    // - même symbole, même position -> RightPosition
    // - symbole présent ailleurs dans le code -> MisPlaced
    // - sinon -> Invalid
    

    // On crée une nouvelle tentative évaluée.

    // On retourne une nouvelle instance de Game avec l'historique mis à jour.
    // - La partie se termine si toutes les billes sont bien placées.
    // - On clone l'historique pour rester dans un style immuable, puis on ajoute la nouvelle tentative.
}
```
> [!TIP]
> Il y a une [cheatsheet](../../docs/Cheatsheet.md) dans ce repo.

<details>
<summary>Solution</summary>

```rust
/// Traite une tentative de code proposée par le joueur.
pub fn process_code_attempt(&self, attempt: [char; 4]) -> Self {
    // Prépare un tableau de flags par défaut pour chaque bille proposée.
    let mut result = [Flag::Invalid, Flag::Invalid, Flag::Invalid, Flag::Invalid];
    // On le remplit de Flag::Invalid pour pouvoir le modifier ensuite.
    for i in 0..4 {
        // Compare chaque bille proposée avec le code secret:
        // - même symbole, même position -> RightPosition
        // - symbole présent ailleurs dans le code -> MisPlaced
        // - sinon -> Invalid
        result[i] = if attempt[i] == self.code[i] {
            Flag::RightPosition
        } else if self.code.contains(&attempt[i]) {
            Flag::MisPlaced
        } else {
            Flag::Invalid
        }
    }
    
    // On crée une nouvelle tentative évaluée.

    // On retourne une nouvelle instance de Game avec l'historique mis à jour.
    // - La partie se termine si toutes les billes sont bien placées.
    // - On clone l'historique pour rester dans un style immuable, puis on ajoute la nouvelle tentative.
}
```
</details>

<br />
Maintenant que l'on dispose du code et des drapeaux on peut créer une instance de la structure `CodeAttempt`.

Vous connaissez la chanson : **A vous de jouer !**

```rust
/// Traite une tentative de code proposée par le joueur.
pub fn process_code_attempt(&self, attempt: [char; 4]) -> Self {
    // Prépare un tableau de flags par défaut pour chaque bille proposée.
    let mut result = [Flag::Invalid, Flag::Invalid, Flag::Invalid, Flag::Invalid];
    // On le remplit de Flag::Invalid pour pouvoir le modifier ensuite.
    for i in 0..4 {
        // Compare chaque bille proposée avec le code secret:
        // - même symbole, même position -> RightPosition
        // - symbole présent ailleurs dans le code -> MisPlaced
        // - sinon -> Invalid
        result[i] = if attempt[i] == self.code[i] {
            Flag::RightPosition
        } else if self.code.contains(&attempt[i]) {
            Flag::MisPlaced
        } else {
            Flag::Invalid
        }
    }
    
    // On crée une nouvelle tentative évaluée.

    // On retourne une nouvelle instance de Game avec l'historique mis à jour.
    // - La partie se termine si toutes les billes sont bien placées.
    // - On clone l'historique pour rester dans un style immuable, puis on ajoute la nouvelle tentative.
}
```

> [!TIP]
> Pensez bien à utiliser les fonctions créées précédemment. 

<details>
<summary>Solution</summary>

```rust
/// Traite une tentative de code proposée par le joueur.
pub fn process_code_attempt(&self, attempt: [char; 4]) -> Self {
    // Prépare un tableau de flags par défaut pour chaque bille proposée.
    let mut result = [Flag::Invalid, Flag::Invalid, Flag::Invalid, Flag::Invalid];
    // On le remplit de Flag::Invalid pour pouvoir le modifier ensuite.
    for i in 0..4 {
        // Compare chaque bille proposée avec le code secret:
        // - même symbole, même position -> RightPosition
        // - symbole présent ailleurs dans le code -> MisPlaced
        // - sinon -> Invalid
        result[i] = if attempt[i] == self.code[i] {
            Flag::RightPosition
        } else if self.code.contains(&attempt[i]) {
            Flag::MisPlaced
        } else {
            Flag::Invalid
        }
    }
    
    // On crée une nouvelle tentative évaluée.
    let attempt = CodeAttempt::new(attempt, result);

    // On retourne une nouvelle instance de Game avec l'historique mis à jour.
    // - La partie se termine si toutes les billes sont bien placées.
    // - On clone l'historique pour rester dans un style immuable, puis on ajoute la nouvelle tentative.
}
```
</details>

<br />
Enfin, on retourne une nouvelle instance avec de la partie avec la nouvelle tentative de code et les états modifiés.

(*Tous ensemble pour la fin*) : **A vous de jouer !**

```rust
/// Traite une tentative de code proposée par le joueur.
pub fn process_code_attempt(&self, attempt: [char; 4]) -> Self {
    // Prépare un tableau de flags par défaut pour chaque bille proposée.
    let mut result = [Flag::Invalid, Flag::Invalid, Flag::Invalid, Flag::Invalid];
    // On le remplit de Flag::Invalid pour pouvoir le modifier ensuite.
    for i in 0..4 {
        // Compare chaque bille proposée avec le code secret:
        // - même symbole, même position -> RightPosition
        // - symbole présent ailleurs dans le code -> MisPlaced
        // - sinon -> Invalid
        result[i] = if attempt[i] == self.code[i] {
            Flag::RightPosition
        } else if self.code.contains(&attempt[i]) {
            Flag::MisPlaced
        } else {
            Flag::Invalid
        }
    }
    
    // On crée une nouvelle tentative évaluée.
    let attempt = CodeAttempt::new(attempt, result);

    // On retourne une nouvelle instance de Game avec l'historique mis à jour.
    // - La partie se termine si toutes les billes sont bien placées.
    // - On clone l'historique pour rester dans un style immuable, puis on ajoute la nouvelle tentative.
}
```
> [!TIP]
> Il y a une [cheatsheet](../../docs/Cheatsheet.md) dans ce repo.

<details>
<summary>Solution</summary>

```rust
/// Traite une tentative de code proposée par le joueur.
pub fn process_code_attempt(&self, attempt: [char; 4]) -> Self {
    // Prépare un tableau de flags par défaut pour chaque bille proposée.
    let mut result = [Flag::Invalid, Flag::Invalid, Flag::Invalid, Flag::Invalid];
    // On le remplit de Flag::Invalid pour pouvoir le modifier ensuite.
    for i in 0..4 {
        // Compare chaque bille proposée avec le code secret:
        // - même symbole, même position -> RightPosition
        // - symbole présent ailleurs dans le code -> MisPlaced
        // - sinon -> Invalid
        result[i] = if attempt[i] == self.code[i] {
            Flag::RightPosition
        } else if self.code.contains(&attempt[i]) {
            Flag::MisPlaced
        } else {
            Flag::Invalid
        }
    }
    
    // On crée une nouvelle tentative évaluée.
    let attempt = CodeAttempt::new(attempt, result);

    // On retourne une nouvelle instance de Game avec l'historique mis à jour.
    // - La partie se termine si toutes les billes sont bien placées.
    // - On clone l'historique pour rester dans un style immuable, puis on ajoute la nouvelle tentative.
    Self {
        is_game_over: attempt.is_game_over(),
        attempts: {
            let mut attempts = self.attempts.clone();
            attempts.push(attempt);
            attempts
        },
        ..*self
    }
}
```
</details>

## Il est l'heure de jouer !

On y est ! Le code est complet et il est enfin temps de le tester ! 

- Pour jouer en mode CLI, il faut utiliser la commande : `cargo run --bin mastermind-cli --features cli`
- Pour jouer en mode Web, il faut utiliser les commandes `trunk build` et `trunk serve`. (Il faut ensuite cliquer sur le lien dans le terminal).
