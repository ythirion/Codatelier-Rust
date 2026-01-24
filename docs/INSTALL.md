# Préparer son environnement

Pour éviter les installations fastidieuses et commencer à coder immédiatement, nous allons utiliser **GitHub Codespaces** pour cet atelier.

> **GitHub Codespaces, c'est quoi ?** Imagine une machine de développement complète qui se lance en quelques secondes, directement dans ton navigateur. Pas besoin d'installer Rust, Trunk ou quoi que ce soit sur ton ordinateur : tout est déjà prêt dans le cloud. Tu ouvres le dépôt, tu lances ton Codespace, et hop, tu codes ! Et quand tu fermes l'onglet, tout s'éteint automatiquement pour économiser les ressources.

## Ce dont tu as besoin

C'est tout simple :
- **Un ordinateur** (Windows, Linux, Mac, peu importe)
- **Un compte GitHub** ([gratuit](https://github.com/signup))

## Préparer l'atelier

### 1. Fork le dépôt

Tu vas d'abord créer ta propre copie du projet :

1. Va sur [github.com/Hodson-Thomas/Codatelier-Rust](https://github.com/Hodson-Thomas/Codatelier-Rust)
2. Clique sur le bouton **Fork** en haut à droite
3. Valide les paramètres par défaut

Voilà, tu as maintenant ta propre version de l'atelier !

### 2. Lance ton Codespace

Maintenant, on va construire ton environnement de développement :

1. Depuis ton dépôt forké, clique sur le bouton vert **Code**
2. Sélectionne l'onglet **Codespaces**
3. Clique sur le **+** pour créer un nouveau Codespace
4. Laisse la magie opérer... ☕

> ⏱️ La première construction prend environ **5 à 10 minutes**. C'est normal ! GitHub est en train d'installer Rust, Trunk et tous les outils dont tu auras besoin. Profite de ce temps pour te préparer un café. Laisse simplement la page ouverte.

### 3. Vérifie que tout fonctionne

Une fois la construction terminée, tu devrais voir apparaître dans le terminal (en bas de l'écran) quelque chose comme ça :
```txt
rustc 1.92.0 (ded5c06cf 2025-12-08)
cargo 1.92.0 (344c4567c 2025-10-21)
trunk 0.21.14
```

Si tu vois ces trois lignes, c'est tout bon ! 🎉 Ton environnement est prêt.

## Le jour de l'atelier

### 1. Synchronise ton fork

Avant de commencer, assure-toi d'avoir la dernière version de l'atelier :

1. Va sur **ton** dépôt forké (github.com/ton-pseudo/Codatelier-Rust)
2. Si tu vois un message indiquant que ton fork est en retard, clique sur **Sync fork**
3. Clique sur **Update branch** pour récupérer les dernières modifications

> 💡 **Pourquoi ?** Des ajustements ou corrections ont peut-être été apportés depuis ta préparation. Cette étape garantit que tu as bien tous les contenus à jour !

### 2. Lance ton Codespace

Pour retrouver ton Codespace le jour J, rien de plus simple :

1. Va sur [github.com/codespaces](https://github.com/codespaces)
2. Clique sur le nom de ton Codespace (en gras)
3. Et c'est parti ! 🚀

> 💡 **Astuce** : Ton Codespace se met en pause automatiquement quand tu ne l'utilises pas. Pas de panique, toutes tes modifications sont sauvegardées !