# moseiik — Notes

Projet de base récupéré et poussé sur le dépôt git perso. On ajoute la chaîne de tests autour du code existant (un générateur de mosaïque).

## Méthodologie

Nous allons développer les tests un par un, du plus simple au plus complexe, en validant et committant chaque test avant de passer au suivant. Nous allons commencer par la distance L1 dans l'ordre : `generic`, puis `x86` (SSE2), puis `aarch64` (NEON). Nous faisons cela car le test générique nous oblige à poser les bases (une fonction qui crée une image de test, les couleurs et le résultat L1 attendu), et les deux versions SIMD qui réutiliseront ensuite exactement ces mêmes bases, ce qui nous permet de réécrire que l'appel de la fonction. Nous retenons également que la version `aarch64` ne se compile pas sur notre machine x86 : elle ne sera testée que via Docker en émulation ARM. Nous terminerons par `prepare_tiles` et `prepare_target`, car elles touchent à un autre type de logique (lecture de fichiers, redimensionnement, rognage).

## 1) unit_test_generic()

Nous avons écrit et validé le premier test, celui de la version générique de la distance L1 (`l1_generic`). Pour le tester, nous avons créé une petite fonction qui génère une image unie d'une seule couleur, ce qui nous permet de connaître à l'avance la distance attendue entre deux images. Nous prenons deux couleurs fixées, nous calculons à la main la distance L1 correspondante, et nous vérifions que la fonction renvoie bien cette valeur. On vérifie également que la distance d'une image avec elle-même vaut zéro.

-> Test Validé

## 2) unit_test_x86()

Nous avons ajouté le test de la version `x86` (SSE2). Comme cette version doit produire exactement le même résultat que la version générique, nous réutilisons les mêmes images et les mêmes couleurs, et nous vérifions à la fois que la fonction renvoie la valeur attendue et qu'elle coïncide avec la version générique, qui nous sert de référence. Cette fonction étant `unsafe`, nous l'appelons dans un bloc `unsafe`, et comme elle n'existe que sur les processeurs x86, nous restreignons le test à cette architecture avec `#[cfg(target_arch = ...)]`.

-> Test Validé

## 2) unit_test_aarch64()

Nous avons ajouté le test de la version aarch64 (NEON). Comme pour la version x86, cette implémentation doit produire exactement le même résultat que la version générique. Nous réutilisons donc les mêmes images de test, les mêmes couleurs et la même valeur L1 attendue, puis nous vérifions que la fonction renvoie le résultat attendu et qu'il correspond à celui de l1_generic, qui reste notre référence. Cette fonction étant également unsafe, nous l'appelons dans un bloc unsafe, et comme elle n'existe que sur l'architecture aarch64, nous limitons sa compilation avec #[cfg(target_arch = "aarch64")]. Ce test ne pouvant pas être exécuté sur notre machine de développement x86, nous le validerons via Docker en émulation ARM.

-> Test écrit, validation prévue sous Docker ARM
