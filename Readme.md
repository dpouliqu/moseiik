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

## 3) unit_test_aarch64()

-> cf branche develop_aarch64

## 4) unit_test_prepare_tiles()

Nous avons ajouté le test de `prepare_tiles`. Cette fonction lit toutes les images d'un dossier et les redimensionne à la taille demandée ; nous vérifions donc ici la taille des vignettes, pas leur contenu. Nous lui passons le dossier `assets/tiles-small`, qui contient quatre images, avec une taille de tuile de 8x8. Comme la fonction renvoie un `Result`, nous récupérons sa valeur avec `.expect(...)`, ce qui fait échouer le test avec un message clair si le dossier est introuvable. Nous vérifions ensuite que nous récupérons bien quatre vignettes, puis, pour chacune, que sa largeur et sa hauteur correspondent à la taille demandée.

-> Test Validé

## 5) unit_test_prepare_target()

Nous avons ajouté le test de `prepare_target`, qui ouvre l'image cible, la rogne pour que ses dimensions soient des multiples de la taille de tuile, puis lui applique un facteur d'agrandissement. Nous utilisons l'image `assets/target-small.png`, qui fait 10x10, avec une taille de tuile de 3. Avec un scaling de 1, l'image est rognée à 9x9 (car 10 - 10 % 3 = 9), et nous vérifions ces dimensions ainsi que le fait qu'elles soient bien des multiples de la taille de tuile. Nous refaisons ensuite l'appel avec un scaling de 2 et nous vérifions que les dimensions sont doublées (18x18), ce qui valide la mise à l'échelle.

-> Test Validé


