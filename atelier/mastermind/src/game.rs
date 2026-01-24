#![allow(dead_code, unused_imports)]
// Moteur de jeu Mastermind côté logique, partagé entre le CLI et le front-end.

use rand::seq::SliceRandom;

// Les 6 billes possibles (green, red, blue, yellow, black, white).
const AVAILABLE_BALLS: [char; 6] = ['g', 'r', 'b', 'y', 'k', 'w'];

/// Résultat du comparatif entre une bille proposée et le code secret.
#[derive(PartialEq, Clone)]
pub enum Flag {
    /// La bille est bonne et bien placée.
    /// La bille est bonne mais mal placée.
    /// La bille n'est pas dans le code secret.
}

/// État d'une partie de Mastermind.
#[derive(PartialEq, Clone)]
pub struct Game {
    /// Code secret de 4 billes.
    /// Historique des tentatives.
    /// True si le code a été trouvé.
    /// Active pour le mode CLI interactif.
}

/// Une proposition de l'utilisateur et son évaluation.
#[derive(PartialEq, Clone)]
pub struct CodeAttempt {
    /// Proposition saisie par le joueur.
    /// Résultat associé: un flag par bille.
}

/// Tire un code secret aléatoire de 4 billes distinctes.
fn create_random_code() -> [char; 4] {
    todo!();
    // On copie les billes disponibles pour pouvoir les mélanger sans toucher à la constante.
    // Générateur de nombres aléatoires fourni par rand 0.9.
    // Mélange aléatoire in-place pour tirer 4 billes uniques.
    // On retourne les 4 premières billes du tableau mélangé.
}

/// Valide qu'un caractère correspond à une bille autorisée.
pub fn is_valid_char(c: char) -> bool {
    todo!();
    // Vérifie si la bille proposée fait partie de l'alphabet autorisé.
}

impl Game {
    /// Construit une partie avec un nouveau code et aucun historique.
    pub fn new(is_game_active: bool) -> Self {
        todo!();
        // Crée une partie avec un nouveau code secret et aucun historique.
    }

    /// Traite une tentative de code proposée par le joueur.
    pub fn process_code_attempt(&self, attempt: [char; 4]) -> Self {
        todo!();

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
}

impl Flag {
    /// Indique si la bille est à la bonne position.
    pub fn is_right_position(&self) -> bool {
        // Pratique pour tester si un flag correspond à une bille parfaitement placée.
        matches!(self, Flag::RightPosition)
    }
}

impl CodeAttempt {
    /// Construit une tentative évaluée.
    pub fn new(attempt: [char; 4], result: [Flag; 4]) -> Self {
        todo!();
        // Simple constructeur data-only.
    }

    /// Vrai si les 4 flags indiquent une victoire.
    pub fn is_game_over(&self) -> bool {
        todo!();
        // La partie est gagnée si les 4 flags sont RightPosition.
    }
}
