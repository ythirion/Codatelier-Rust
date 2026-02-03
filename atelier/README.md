# CodaTelier - Créer ma première application Rust 🦀

Bienvenue dans l'atelier ! Voici le programme que nous allons suivre ensemble pour découvrir Rust et créer tes premiers programmes.

---

## 📋 Programme de l'atelier

| Section                          | Description                                                                                                                |
| -------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| **Quelques mots sur le langage** | Introduction rapide à Rust                                                                                                 |
| **Le juste prix**                | Crée ton premier programme en Rust avec le jeu du juste prix, pour appréhender les concepts et syntaxes de base du langage |
| **Mastermind**                   | Implémente un jeu de Mastermind en Rust qui peut tourner sur CLI ou Web                                                    |

---

## 🦀 Rust en quelques mots

- **Remplaçant du C/C++** avec les performances des langages bas-niveau et les concepts des langages haut-niveau
- **Sécurité** garantie par le compilateur
- **Utilisé principalement pour :**
  - Des applications systèmes, des backends, des CLI
  - Du web avec WASM
  - Des services réseaux
  - De l'informatique embarqué
  - De la crypto
  - Des apps web & mobiles
- Créé en 2006 par Graydon Hoare, puis repris à partir de 2010 par la fondation Mozilla
- Version actuelle : 1.93.0
- Sa mascotte : **Ferris** 🦀

<div align="center">
  <img src="https://rustacean.net/assets/rustacean-flat-happy.png" alt="Ferris le crabe, mascotte de Rust" width="300"/>
</div>

## Rust en quelques chiffres

### Evolution
- **4 millions** de développeurs Rust dans le monde (vs 600 000 en 2020)
- **83% de taux d'admiration** (langage le plus aimé pour la 9ème année consécutive - Stack Overflow 2024)
- **+40% de croissance** sur GitHub en 2024
- **200 650** packages sur crates.io
- **507,6 millions de téléchargements quotidiens** sur crates.io
- Position **#13 dans l'index TIOBE** (février 2025, record historique)

### Google

- 21% du nouveau code natif d'Android 13 est en Rust
- 1,5 million de lignes de Rust dans AOSP
- -68% de vulnérabilités mémoire (76% en 2019 → 24% en 2024)
- Productivité 2x supérieure à C++ après formation d'après les équipiers

### Microsoft

- 36 000 lignes de Rust dans le kernel Windows 11 (win32kbase_rs.sys)
- DirectWrite Core réécrit : 152 000 lignes par 2 développeurs en 6 mois
- Objectif 2030 : élimination complète de C/C++ du code critique
- Mark Russinovich (CTO Azure) : Microsoft "all-in on Rust"

### Amazon AWS

- Firecracker (100% Rust) : trillions d'exécutions mensuelles sur Lambda/Fargate
- Démarrage microVM < 125ms, mémoire < 5 MiB

### Meta (Facebook)

- Rust = langage officiel (1 des 4 langages serveur supportés)
- Mononoke (backend contrôle de source) : 2-4 ordres de grandeur plus rapide
- Core Messaging Library (Facebook, Messenger, Instagram) : migration C → Rust en cours

### Autres Géants

Cloudflare : Pingora traite 35 millions req/s (remplace nginx)
Discord : migration Go → Rust élimine les pics de latence GC

> 📚 Vous pouvez retrouver l'étude complête sur l'adoption de Rust réalisé pour collecter ces métriques [ici](../docs/Study.md)

---

## 💡 Avant de commencer

**⏱️ Les phases d'exercices sont timées**
Pour avancer tous ensemble et rester dans le temps imparti, les phases sont chronométrées. Pas de panique, on corrige après chaque exercice !

**❓ Pose des questions**
Il n'y a pas de questions stupides ! N'hésite pas à demander de l'aide et à partager tes solutions avec les autres participants.

**📖 Consulte notre Cheatsheet**
On a préparé une feuille de mémo avec la plupart des concepts dont tu auras besoin. Elle est disponible [ici](../docs/Cheatsheet.md).

**🔍 Va chercher de la doc**
Rust est très bien documenté avec une communauté très active. N'hésite pas à chercher des solutions sur internet, c'est comme ça qu'on apprend !

---

## 🎯 Les ateliers

- **🎲 Le juste prix :** C'est par ici → [Atelier](./le-juste-prix/)
- **🎨 Le Mastermind :** C'est par ici → [Atelier](./mastermind/)
