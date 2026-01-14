# Codatelier-Rust

## Règles du jeu

L'objectif du jeu est de deviner un code composé de n-billes qui peuvent être de k-couleurs différentes. 

1) Le joueur propose un code.
2) L'ordinateur compare le vrai code avec celui de l'utilisateur et:
- Place un drapeau blanc pour chaque bille correctement placée.
- Place un drapeau rouge pour chaque bille mal placée.
- Place un drapeau noir pour chaque bille qui n'appartient pas au code.

Il y a plusieurs variantes du jeu:
- Dans le version originale l'ordre des drapeaux ne correspond pas à l'ordre des billes du code.
- Une version plus simple existe où l'ordre des drapeaux correspond à l'ordre des billes du code.

## Développement

Je propose de faire un mastermind simplifié: 
- 6 boules différentes.
- Code à 4 couleurs.
- Les drapeaux sont dans l'ordre du code.