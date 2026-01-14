# Fortress — Gestionnaire de mots de passe CLI

## 1. Spécifications fonctionnelles

Nous développons **Fortress**, un gestionnaire de mots de passe en ligne de commande (CLI) écrit en Rust, dont
l’objectif principal est de fournir un outil fiable, sécurisé et simple d’utilisation pour la gestion d’identifiants
sensibles. Fortress permet à un utilisateur de créer et de maintenir un coffre chiffré localement, stocké par défaut
dans un fichier `vault.frt`, sans dépendance à un service externe ou à une infrastructure réseau. Ce choix garantit que
l’utilisateur conserve un contrôle total sur ses données et sur les mécanismes de sécurité appliqués.

Les fonctionnalités proposées couvrent l’ensemble des besoins fondamentaux d’un gestionnaire de mots de passe moderne.
L’utilisateur peut initialiser un coffre chiffré en définissant un mot de passe maître, ajouter de nouvelles entrées
composées d’un nom logique, d’un identifiant et d’un secret associé, lister les entrées existantes afin de retrouver
rapidement une information, consulter le détail d’une entrée spécifique, copier un mot de passe dans le presse-papiers
pour un usage ponctuel, et supprimer des entrées devenues obsolètes. L’interface en ligne de commande est conçue pour
être explicite, cohérente et facilement scriptable.

Le comportement interne du logiciel est orienté vers la réduction maximale de la surface d’exposition des données
sensibles. Le coffre chiffré n’est jamais écrit en clair sur disque : les opérations de déchiffrement ont lieu
exclusivement en mémoire vive et uniquement pour la durée strictement nécessaire à l’exécution de la commande demandée.
Les écritures sur le fichier de coffre sont réalisées de manière atomique et protégées par un mécanisme de verrouillage
afin d’éviter toute corruption liée à des accès concurrents ou à des interruptions inattendues. Les commandes retournent
systématiquement des codes de sortie cohérents et des messages d’erreur compréhensibles, facilitant le diagnostic et
l’automatisation.

Fortress répond également à des exigences non fonctionnelles strictes. Le chiffrement repose sur des primitives modernes
et éprouvées, combinant une fonction de dérivation de clé robuste et un algorithme de chiffrement authentifié. Les
secrets sont générés à l’aide de sources d’aléa cryptographiquement sûres et sont explicitement nettoyés de la mémoire
après usage. Le logiciel fonctionne exclusivement en local, sans aucune communication réseau, ce qui réduit les risques
d’attaque. Les performances visées permettent un usage confortable avec plusieurs centaines, voire milliers d’entrées,
sur les plateformes Linux et macOS, avec une gestion du presse-papiers adaptée à chaque environnement.

Le format de données du coffre est défini de manière claire et évolutive. Le fichier `vault.frt` contient un en-tête
versionné incluant les métadonnées nécessaires au chiffrement, suivi d’un payload chiffré sérialisé. Cette structure
permet d’anticiper des évolutions futures tout en garantissant la compatibilité et la pérennité des données stockées.
Aucun mot de passe n’est stocké en clair ni de manière réversible.

<div style="page-break-after: always;"></div>

## 2. Cycle de vie du logiciel

Le développement de Fortress s’inscrit dans un cycle de vie logiciel structuré et maîtrisé. Nous adoptons une
méthodologie Agile organisée autour de sprints mensuels. Chaque sprint débute par une phase de planification visant à
prioriser le backlog en fonction de la valeur fonctionnelle, des risques techniques et des enjeux de sécurité. Il se
conclut par une revue des livrables et une rétrospective permettant d’identifier les axes d’amélioration du processus et
du produit.

La gestion du code source repose sur Git et sur un workflow rigoureux. Chaque évolution est développée dans une branche
dédiée, associée à un ticket clairement identifié. Les pull requests constituent le point d’entrée unique pour
l’intégration du code et imposent une revue systématique avant toute fusion. Le versionnage sémantique est appliqué afin
de garantir une compréhension claire de la nature des changements introduits entre deux versions.

L’intégration continue est un élément central du cycle de vie. Toute modification déclenche automatiquement une chaîne
de validation comprenant la compilation, l’exécution des tests et les contrôles de conformité. Cette approche permet de
détecter rapidement les régressions et de maintenir un niveau de qualité constant tout au long du développement.

Les releases correspondent à des jalons stables du projet et sont publiées via GitHub Releases. Chaque version publiée
est maintenue, notamment pour la correction des anomalies et des vulnérabilités de sécurité. En cas de problème
critique, des versions correctives peuvent être publiées en dehors du cycle normal afin de limiter l’exposition des
utilisateurs.

Le cycle de vie prend enfin en compte la pérennité et l’évolutivité du logiciel. Les choix techniques, en particulier
ceux liés au format de données et aux mécanismes cryptographiques, sont documentés afin de faciliter les migrations
futures et d’assurer la compatibilité ascendante.

<div style="page-break-after: always;"></div>

## 3. Bonnes pratiques et normes

Le projet Fortress s’appuie sur un ensemble de bonnes pratiques clairement définies et appliquées de manière cohérente.
La sécurité est abordée dès la conception, en privilégiant des primitives cryptographiques reconnues, auditées et
largement utilisées. Le choix des algorithmes et des bibliothèques est volontairement restrictif afin de limiter la
surface d’attaque et de faciliter les audits de sécurité. Les paramètres cryptographiques sont versionnés et intégrés
aux métadonnées du coffre pour anticiper les évolutions.

La protection des secrets en mémoire constitue un principe fondamental. Nous cherchons à réduire au maximum la durée de
vie des données sensibles en mémoire vive et à les nettoyer explicitement après usage, afin de limiter l’impact
potentiel d’un dump mémoire ou d’un comportement indésirable du système hôte.

La qualité du code est assurée par l’application de conventions strictes et par l’utilisation systématique d’outils
d’analyse statique, de linting et de formatage automatique. Cette discipline favorise la lisibilité, la maintenabilité
et la cohérence globale du codebase.

Les tests jouent un rôle central dans la fiabilité du projet. Une stratégie de tests couvre les flux critiques, les
scénarios nominaux et les cas d’erreur. La couverture est mesurée et suivie dans le temps afin d’identifier les zones
sensibles et de prévenir les régressions.

La gestion des dépendances fait l’objet d’une vigilance continue. Les bibliothèques utilisées sont régulièrement
auditées afin de détecter les vulnérabilités connues, avec une attention particulière portée aux dépendances liées à la
cryptographie.

Nous appliquons également des bonnes pratiques d’exploitation en ne stockant jamais le mot de passe maître ni les
secrets dérivés sur disque. Les mécanismes d’entrée sécurisée sont privilégiés et clairement documentés pour les
utilisateurs.

Enfin, nous cherchons à garantir la reproductibilité des builds et la traçabilité des artefacts distribués. La
documentation est considérée comme un élément essentiel de la qualité, tant pour les utilisateurs que pour les
contributeurs. La gouvernance du projet inclut un processus de divulgation responsable des vulnérabilités et le recours
à des audits externes lorsque cela est pertinent.

<div style="page-break-after: always;"></div>

## 4. Tests

Note rapide : le binaire s'appelle `frtrs` (cf. `Cargo.toml`). Pour exécuter les scénarios localement :

- Compiler : `cargo build --bin frtrs`
- Exécuter : `./target/debug/frtrs <commande...>` ou `cargo run --bin frtrs -- <commande...>`
- Le code demande le mot de passe en mode interactif via prompt. Pour tests non-interactifs, vous pouvez passer le mot de passe via un pipe :
  `printf "masterpw\n" | ./target/debug/frtrs create --file /tmp/mon_test.frt`

---

### Test #1 : Création d'un coffre (succès)

Description : Vérifie que `frtrs create` crée un fichier de coffre chiffré.
Préconditions : Aucun fichier existant au chemin cible.
Étapes :

- `FILE=$(mktemp /tmp/fortress_test_XXXXXX.frt)`
- `printf "masterpw\n" | ./target/debug/frtrs create --file "$FILE"`
- `ls -l "$FILE"`
  Résultat attendu : Le fichier
  `$FILE` existe et la commande a renvoyé un code de sortie 0 et affiche "Created new vault".

---

### Test #2 : Création d'un coffre (déjà existant sans `--force`)

Description : Vérifie que `create` échoue si le coffre existe et `--force` n'est pas utilisé.
Préconditions : Fichier existant au chemin cible.
Étapes :

- `FILE=$(mktemp /tmp/fortress_test_XXXXXX.frt)`
- `printf "masterpw\n" | ./target/debug/frtrs create --file "$FILE"`
- Ré-exécuter sans `--force` : `printf "masterpw\n" | ./target/debug/frtrs create --file "$FILE"` > capture 2>&1 || true
- `tail -n 5 capture`
  Résultat attendu : La seconde exécution renvoie un code non nul et le message d'erreur indique que le coffre existe (comportement observable dans la sortie d'erreur).

---

### Test #3 : Création forcée (`--force`) écrase le coffre

Description : Vérifie que `--force` permet d'écraser un coffre existant.
Préconditions : Fichier existant au chemin cible contenant des données.
Étapes :

- `FILE=$(mktemp /tmp/fortress_test_XXXXXX.frt)`
- `printf "oldpw\n" | ./target/debug/frtrs create --file "$FILE"`
- `printf "newpw\n" | ./target/debug/frtrs create --force --file "$FILE"`
- Vérifier que la commande a réussi et que le fichier existe.
  Résultat attendu : La seconde commande renvoie 0 et le coffre est remplacé.

---

### Test #4 : Ajout d'une entrée et listing (roundtrip)

Description : Ajout d'une entrée via `add` puis vérification avec `list`.
Préconditions : Coffre initialisé.
Étapes :

- `FILE=$(mktemp /tmp/fortress_test_XXXXXX.frt)`
- `printf "masterpw\n" | ./target/debug/frtrs create --file "$FILE"`
- `printf "masterpw\n" | ./target/debug/frtrs add "site/test" --username "alice" --password "pw123" --file "$FILE"`
- `printf "masterpw\n" | ./target/debug/frtrs list --file "$FILE"` (rediriger la sortie)
  Résultat attendu : `list` affiche une entrée contenant `site/test` et
  `alice` (le mot de passe n'est pas affiché en clair dans la ligne listée).

---

### Test #5 : Affichage d'une entrée (`view`) et gestion d'ID manquant

Description : `view` doit retourner l'entrée existante et une erreur pour un identifiant absent.
Préconditions : Coffre avec une entrée `view_id`.
Étapes :

- Créer coffre et `add view_id` comme précédemment.
- `printf "masterpw\n" | ./target/debug/frtrs view view_id --file "$FILE"` -> vérifier sortie contenant
  `The decoded password is:`
-
`printf "masterpw\n" | ./target/debug/frtrs view no_such_id --file "$FILE"` -> vérifier code de sortie non nul et message d'erreur.
Résultat attendu : `view view_id` affiche le mot de passe décodé,
`view no_such_id` renvoie une erreur identifiant manquant.

---

### Test #6 : Copie vers le presse-papiers (comportement utilisateur)

Description :
`copy` doit tenter de mettre le mot de passe dans le presse-papiers. Test best-effort (dépend du système et du support du presse-papiers).
Préconditions : Coffre avec entrée `copy_id`.
Étapes :

- Ajouter `copy_id`.
- `printf "masterpw\n" | ./target/debug/frtrs copy copy_id --file "$FILE" 2>&1 | tee out`.
- Examiner `out` pour "The decoded password is in your clipboard" ou un message d'erreur `ClipboardError`.
  Résultat attendu : Message indiquant succès ou échec du collage (les deux sont acceptables, mais le comportement doit être informatif).

---

### Test #7 : Suppression d'une entrée et tentative de suppression d'une entrée manquante

Description : `remove` supprime l'identifiant ciblé et renvoie une erreur si absent.
Préconditions : Coffre avec `remove_id`.
Étapes :

- Ajouter `remove_id`.
- `printf "masterpw\n" | ./target/debug/frtrs remove remove_id --file "$FILE"` -> vérifier code 0.
-
`printf "masterpw\n" | ./target/debug/frtrs remove remove_id --file "$FILE"` -> vérifier code non nul et message d'erreur IdNotFound.
Résultat attendu : Le premier `remove` réussit; le second échoue avec message IdNotFound.

---

### Test #8 : Intégrité du fichier — tampering (détection de modification)

Description : Modifier un octet du fichier chiffré doit empêcher le déchiffrement.
Préconditions : Coffre créé et contenant des entrées.
Étapes :

- `printf "masterpw\n" | ./target/debug/frtrs list --file "$FILE"` pour vérifier fonctionnement initial.
- `printf "\x01" | dd of="$FILE" bs=1 seek=60 conv=notrunc` (inverser un octet dans le ciphertext)
- `printf "masterpw\n" | ./target/debug/frtrs list --file "$FILE"` -> vérifier erreur.
  Résultat attendu : La commande échoue et signale un problème de déchiffrement (erreur non silencieuse).

---

### Test #9 : Fichier tronqué (taille minimale)

Description : Écrire un fichier trop court et vérifier que le programme détecte la corruption.
Préconditions : Aucun.
Étapes :

- `FILE_TRUNC=$(mktemp /tmp/fortress_trunc_XXXXXX.frt)`
- `head -c 10 /dev/zero > "$FILE_TRUNC"`
- `printf "masterpw\n" | ./target/debug/frtrs list --file "$FILE_TRUNC"` (rediriger stderr)
  Résultat attendu : La commande échoue et l'erreur indique une corruption du coffre (CorruptedVault).

---

### Test #10 : Mot de passe maître incorrect

Description : Essayer d'ouvrir le coffre avec un mauvais mot de passe.
Préconditions : Coffre initialisé avec `masterpw`.
Étapes :

- `printf "wrongpw\n" | ./target/debug/frtrs list --file "$FILE" 2>&1 | tee wrong_out`
- Vérifier que la sortie contient un message d'erreur pertinent et que le code de sortie est non nul.
  Résultat attendu : Accès refusé; message d'erreur relatif au mot de passe incorrect ou au déchiffrement.

---

### Test #11 : Pas de mot de passe en clair dans le fichier

Description : S'assurer qu'un mot de passe ajouté ne figure pas en clair dans le fichier chiffré.
Préconditions : Coffre initialisé.
Étapes :

- `SECRET="__SECRET_PW__"`
- `printf "masterpw\n" | ./target/debug/frtrs add leak_id --username leak --password "$SECRET" --file "$FILE"`
- `grep -a -F "$SECRET" "$FILE"` (commande retourne code 1 si non trouvé)
  Résultat attendu : `grep` ne trouve pas la séquence `$SECRET` dans le fichier chiffré.

---

### Test #12 : Erreur d'écriture (permission refusée)

Description : Tester comportement quand le système de fichiers empêche la création/écriture du coffre.
Préconditions : Créer un répertoire temporaire sans droits d'écriture pour l'utilisateur courant.
Étapes :

- `D=$(mktemp -d /tmp/fortress_nw_XXXXXX)`
- `chmod -w "$D"`
- `FILE="$D/forbidden.frt"`
- `printf "masterpw\n" | ./target/debug/frtrs create --file "$FILE" 2>&1 | tee perm_out` (puis
  `chmod +w "$D"` pour nettoyer)
  Résultat attendu : La commande échoue et la sortie contient une erreur d'I/O (permission denied).

## 5. Organisation du projet

L’organisation du projet Fortress repose sur des outils et des processus visant à assurer une collaboration efficace et
un haut niveau de qualité. GitHub est utilisé comme plateforme centrale pour la gestion du code source, des tickets et
de la planification. Les issues décrivent précisément les besoins fonctionnels, les anomalies ou les améliorations
attendues, avec des critères d’acceptation explicites et des priorités clairement définies.

La chaîne CI/CD est basée sur GitHub Actions et automatise l’ensemble des contrôles qualité et sécurité. Chaque
contribution déclenche les étapes de validation, incluant les tests, les analyses et les audits, afin de garantir que
seules des modifications conformes sont intégrées.

Le travail de l’équipe est structuré autour de sprints mensuels, rythmés par des phases de planification, de revue et de
rétrospective. Les tickets ne sont considérés comme terminés que lorsque les critères d’acceptation sont remplis et
validés par des tests automatisés.

Une procédure spécifique est définie pour la gestion des incidents et des vulnérabilités critiques. En cas de problème
majeur, un traitement prioritaire est déclenché, incluant la création d’un correctif dédié, un cycle de test accéléré et
la publication rapide d’une version corrigée.

<div style="page-break-after: always;"></div>

## 6. Organisation de l’équipe

L’équipe projet est composée de quatre personnes aux rôles clairement définis et complémentaires. Un lead développeur
assure la cohérence de l’architecture, pilote les évolutions fonctionnelles et valide les contributions majeures. Un
ingénieur sécurité est responsable des choix cryptographiques, des audits et de la gestion des vulnérabilités. Un
ingénieur qualité et CI se concentre sur les tests automatisés, la couverture et la fiabilité des pipelines. Enfin, un
responsable produit et documentation veille à la clarté des supports utilisateurs, à la préparation des releases et à la
communication associée.

Cette répartition permet de couvrir l’ensemble des enjeux techniques, organisationnels et sécuritaires du projet tout en
maintenant une responsabilité claire pour chaque domaine.

## 7. Backlog et planification

Le backlog est organisé sur des sprints mensuels, chacun regroupant des tâches clairement identifiées, priorisées et
assorties de critères d’acceptation précis.

### Sprint 1 — Stabilisation et qualité

* Compléter les tests unitaires et d’intégration pour les commandes existantes (p3, p1) — priorité P0. La couverture
  doit être augmentée et l’ensemble des tests doit être valide en intégration continue.
* Ajouter l’épuration mémoire des secrets (p1, p2) — priorité P1. Les secrets doivent être nettoyés après usage et ce
  comportement validé par des tests.
* Renforcer le verrouillage de fichier et les écritures atomiques (p1) — priorité P0. Des tests de concurrence doivent
  démontrer l’absence de corruption.
* Automatiser et renforcer les vérifications CI, incluant la couverture et l’audit des dépendances (p3) — priorité P1.

### Sprint 2 — Renforcement cryptographique

* Implémenter et tester Argon2 comme option de KDF (p2, p1) — priorité P0. Les migrations et la compatibilité doivent
  être validées.
* Mettre en place du fuzzing sur les parseurs et sérialiseurs (p3) — priorité P1. Les crashs identifiés doivent être
  corrigés.
* Planifier et initier un audit de sécurité externe (p2, p4) — priorité P1.

### Sprint 3 — Packaging et releases

* Automatiser les builds multiplateformes et la signature des artefacts (p4, p3) — priorité P0.
* Mettre en place des builds reproductibles et des tests d’intégrité (p3) — priorité P1.
* Documenter les procédures de release et de rollback (p4) — priorité P1.

### Sprint 4 — Fonctionnalités utilisateur et maintenance

* Renforcer la gestion du presse-papiers avec timeout et option de désactivation (p1, p4) — priorité P1.
* Implémenter l’export et l’import chiffrés avec outils de restauration (p1, p3) — priorité P1.
* Traiter la dette technique identifiée (p1, p3) — priorité P2.

### Maintenance continue

* Surveillance régulière des dépendances et correction des vulnérabilités (p2).
* Gestion des correctifs critiques via des hotfixes dédiés, avec tests accélérés et publication rapide (p1, p2, p3).\