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

## Tests d'intégration

Après les tests unitaires, nous testons la fonction `compute_mosaic` de bout en bout dans `tests/temp.rs`. Le principe est de regénérer la mosaïque de `assets/kit.jpeg` avec la base de vignettes, puis de la comparer à l'image de référence `assets/ground-truth-kit.png`, générée avec une taille de tuile de 25 et un scaling de 1. Comme `compute_mosaic` ne renvoie rien mais écrit le résultat dans un fichier, le test relit ce fichier avant de le comparer.

Nous ne comparons pas les deux images à l'identique. En effet, les vignettes sont chargées en parallèle, donc en cas d'égalité de distance entre deux vignettes pour un même bloc, le choix peut varier d'une exécution à l'autre. Nous avons mesuré cet écart en générant une fois la mosaïque puis en la comparant pixel à pixel avec la vérité terrain (à l'aide d'un petit script Python utilisant Pillow et numpy) : environ 0,13 % des pixels diffèrent. Une égalité stricte serait donc instable. Nous vérifions à la place que la proportion de pixels différents reste sous un seuil de 1 %, choisi au-dessus de l'écart mesuré mais bien en-dessous de ce que produirait une vraie erreur de calcul. Les trois tests couvrent les chemins générique, x86 (SSE2) et aarch64 (NEON).

## Exécution des tests dans Docker

Pour rendre les tests reproductibles et indépendants de la machine, nous fournissons un `Dockerfile`. Il part de l'image officielle `rust`, installe les outils nécessaires, copie le projet, télécharge la base de vignettes (qui n'est pas versionnée) puis compile les tests. Son `ENTRYPOINT` lance `cargo test --release`, si bien que démarrer le conteneur exécute directement toute la suite de tests.

Nous avons d'abord construit et lancé l'image sur notre architecture x86, où l'ensemble des tests unitaires et d'intégration passent dans le conteneur. L'image de base `rust` étant disponible aussi en version ARM, le même Dockerfile pourra ensuite être utilisé pour exécuter les tests sur architecture aarch64 via l'émulation, ce qui permettra de valider les versions NEON (`unit_test_aarch64` et `test_aarch64`).


## 2) unit_test_aarch64()

Nous avons ajouté le test de la version aarch64 (NEON). Comme pour la version x86, cette implémentation doit produire exactement le même résultat que la version générique. Nous réutilisons donc les mêmes images de test, les mêmes couleurs et la même valeur L1 attendue, puis nous vérifions que la fonction renvoie le résultat attendu et qu'il correspond à celui de l1_generic, qui reste notre référence. Cette fonction étant également unsafe, nous l'appelons dans un bloc unsafe, et comme elle n'existe que sur l'architecture aarch64, nous limitons sa compilation avec #[cfg(target_arch = "aarch64")]. Ce test ne pouvant pas être exécuté sur notre machine de développement x86, nous le validerons via Docker en émulation ARM.

-> Test écrit, validation prévue sous Docker ARM

## Validation des tests sur architecture ARM

Nous avons d'abord construit et lancé l'image sur notre architecture x86, où l'ensemble des tests unitaires et d'intégration passent dans le conteneur. L'image de base `rust` étant aussi disponible en version ARM, nous avons ensuite construit l'image pour l'architecture aarch64 grâce à l'émulation (QEMU). Dans ce conteneur ARM, ce sont les versions NEON qui sont compilées et exécutées : `unit_test_aarch64` et `test_aarch64` passent, ce qui valide enfin l'implémentation `l1_neon` que nous ne pouvions pas tester sur notre machine x86.

