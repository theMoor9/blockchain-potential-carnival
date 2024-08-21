# **Blockchain CheatSheet - Applications Crypto**

<div style="font-size: 70%"><b>&#x1F553; Temps de lecture : 8 min</b></div>

---

##### **Table des matières**

###### [§ Argent et Crypto](#-Argent-et-Crypto-1)

- [Argent](#Argent)
- [Acquisition de Cryptomonnaies](#Acquisition-de-Cryptomonnaies)
- [Échanges de Crypto](#%C3%89changes-de-Crypto)

###### [§ Stablecoins](#-Stablecoins-1)

- [Volatilité des Cryptomonnaies](#Volatilit%C3%A9-des-Cryptomonnaies)
- [Adossés à des FIAT](#Adoss%C3%A9s-%C3%A0-des-FIAT)
- [Adossés à des Actifs Réels](#Adoss%C3%A9s-%C3%A0-des-Actifs-R%C3%A9els)
- [Adossés à des Crypto](#Adoss%C3%A9s-%C3%A0-des-Crypto)
- [Non Adossés](#Non-Adoss%C3%A9s)

###### [§ Initial Coin Offerings (ICO)](#-Initial-Coin-Offerings-ICO-1)

- [Investisseurs](#Investisseurs)
- [Critères de Valeur](#Crit%C3%A8res-de-Valeur)
- [Tendances ICO](#Tendances-ICO)

###### [§ Crypto Frauduleux](#-Crypto-Frauduleux-1)

- [PAS DE SCHÉMAS PONZI](#PAS-DE-SCH%C3%89MAS-PONZI)
- [AUCUN RENDEMENT IRRÉALISTE](#AUCUN-RENDEMENT-IRR%C3%89ALISTE)
- [AUCUNE RENCONTRE ÉMOTIONNELLE/OFFRES SUR LE HYPE](#AUCUNE-RENCONTRE-%C3%89MOTIONNELLE-OFFRES-SUR-LE-HYPE)
- [AUCUN ÉTRANGER AVEC DE L'ARGENT](#AUCUN-%C3%89TRANGER-AVEC-DE-LARGENT)
- [ALARMES ROUGES](#ALARMES-ROUGES)

###### [§ Cadre d'Évaluation](#-Cadre-d%C3%89valuation-1)

- [Application](#Application)
- [Feuille](#Feuille)

<hr style="page-break-before: always; ">

## **§ Argent et Crypto**

### Argent

**Fonctions de l'Argent**:

- Une unité de mesure pour comparer la valeur des biens et services.
- Un moyen d'échange plus efficace que le troc.
- Un stockage de valeur qui se conserve dans le temps, contrairement aux denrées périssables.
- Un moyen facile de transférer de la valeur.

> Le transfert simple de valeur est la notion de base derrière le Bitcoin.

### Acquisition de Cryptomonnaies

- **Mining** : Participation au réseau pour valider les transactions et miner de nouveaux blocs.
- **Cadeaux ou Paiements** : Réception de cryptomonnaies en tant que moyen de paiement ou cadeau.
- **Échanges Modernes et Portefeuilles** : Achat de cryptomonnaies via des plateformes similaires à celles des courtiers en actions.
- [**Initial Coin Offerings (ICO)**](#-Initial-Coin-Offerings-ICO-1) : Investissement dans un nouveau projet de cryptomonnaie en participant à son offre initiale.

### Échanges de Crypto

Contrairement aux bourses traditionnelles, les échanges de crypto se conforment souvent strictement aux directives Know-Your-Customer (KYC) en raison des exigences réglementaires liées à la blockchain et aux cryptomonnaies.

Il est important de noter que ces échanges ne sont pas sans risques. Les défis incluent [fraudes](#-Crypto-Frauduleux-1), piratages et mauvaises pratiques commerciales. Une évaluation soigneuse est nécessaire pour garantir sécurité et fiabilité.

---

## **§ Stablecoins**

### Volatilité des Cryptomonnaies

Les cryptomonnaies sont connues pour leur volatilité élevée. Pendant la crise financière de 2008, les marchés ont chuté jusqu'à 9 % en une semaine ; c'était déjà extrême. Mais honnêtement, c'est rien comparé aux mouvements liquides normaux sur les marchés de cryptomonnaies. Si tu lis ceci, tu sais probablement déjà à quel point les fluctuations sont énormes.

En termes de complexité, il y a quatre types principaux de stablecoins :

1. [Adossés à des FIAT](#Adoss%C3%A9s-%C3%A0-des-FIAT)
2. [Adossés à des Actifs Réels](#Adoss%C3%A9s-%C3%A0-des-Actifs-R%C3%A9els)
3. [Adossés à des Crypto](#Adoss%C3%A9s-%C3%A0-des-Crypto)
4. [Non Adossés](#Non-Adoss%C3%A9s)

### Adossés à des FIAT

##### L'idée

Les stablecoins adossés à des FIAT sont censés offrir un accès optimisé aux transferts en tout temps, y compris des transactions peu coûteuses à des heures inhabituelles, comme à 2 h du matin un dimanche. Cette approche aide en partie à résoudre le problème de la haute volatilité, comme on le voit sur les bourses de Bitcoin.

De plus, si l'émetteur du stablecoin auquel tu as échangé contre des FIAT rencontre des difficultés financières, tu devrais pouvoir récupérer la valeur en FIAT.

### Adossés à des Actifs Réels

Un exemple majeur est le Digix Gold (DGX), où 1DGX équivaut à un gramme d'or.

- Utilise le protocole blockchain Ethereum ERC-20.
- Détenu dans un coffre à Singapour.
- Rachetable en lots de 100 ou 1000 grammes.
- Les frais comprennent les coûts de transaction et de stockage pour l'or physique.

##### Autres Exemples

- Swiss Real Coin (SRC) : Soutenu par un portefeuille immobilier suisse.
- D1 Coin ERC-20 : Adossé à des diamants.

### Adossés à des Crypto

Ce type de stablecoin fonctionne de manière similaire à ceux adossés à des FIAT, mais avec une variation intéressante basée sur la cryptomonnaie choisie.

**Comment ça marche** _Supposons que 1 stablecoin (STB) = 1 SOL_

1. **Premier achat** : Lorsque tu achètes des STB, ta transaction couvrira plus que le montant correspondant en SOL, pour inclure une marge pour les fluctuations du marché.
    
    yaml
    
    Copia codice
    
     `Nécessaire : 50 STB  Achat : 75 SOL  Dans le portefeuille :  	- Total : 50 STB/SOL en suspens + 25 SOL  	- Répartition : 50 STB et 25 SOL (le SOL supplémentaire sert de tampon)`
    
2. **Adossement** : À ce stade, la valeur du SOL est adossée, et si la valeur de la crypto chute fortement, le SOL correspondant aux STB est liquidé par les _gardiens_ dans le contrat, pour acheter des STB et améliorer la stabilité du système.
    
    yaml
    
    Copia codice
    
     `Situation de liquidation : 50 SOL liquidés pour maintenir le PEG des STB  Solde effectif : 50 STB et 25 SOL`
    
3. **Augmentation de la valeur et vente** : Si la valeur du SOL augmente et que tu vends, tu peux échanger tes STB adossés contre des SOL plus précieux ou même contre des FIAT.
    
    bash
    
    Copia codice
    
     `Augmentation : SOL +50%  Vente : 50 STB à nouveau en FIAT ou SOL, profit de l'augmentation de valeur  Résultat : Gain grâce à l'augmentation de la valeur du SOL, plus SOL restant`
    

> Si le système rencontre une menace ou une instabilité significative, une liquidation globale peut être activée, restituant la crypto adossée aux détenteurs d'origine.

### Non Adossés

Ce type de stablecoin fonctionne par des mécanismes internes pour assurer que son prix reste stable.

**Comment ça marche**

- **Mécanismes de régulation** : L'autorité régulatrice du stablecoin peut intervenir sur le marché en émettant ou en rachetant des coins pour ajuster l'offre et stabiliser la valeur.
- **Conversion en Tokens Obligataires** : Les stablecoins rachetés peuvent être convertis en tokens obligataires, prenant temporairement la liquidité du marché et stabilisant la valeur de la pièce en interrompant la génération de rendement.
- **Conversion en Tokens d'Actions** : Alternativement, les stablecoins rachetés peuvent être convertis en tokens d'actions, distribuant la valeur stockée parmi les détenteurs, qui pourraient être intéressés par la gestion des actifs pendant les fluctuations du marché.

##### Mécanismes Supplémentaires

Certains stablecoins utilisent des cadres de programmation de blockchains existantes pour créer une version personnalisée de la cryptomonnaie en modifiant le code pour changer les dynamiques économiques.

**Comment ça marche**

- **Ajustements de l'offre** : Lorsque la valeur augmente, une offre supplémentaire est introduite pour modérer le prix.
- **Ajustements du mining** : Lorsque la valeur chute, les récompenses de minage peuvent être réduites, voire à zéro, pour éviter un excès d'offre.
- **Utilisation des frais** : Les frais peuvent être collectés pour atténuer les baisses de valeur, et peuvent être détruits dans des scénarios où la valeur augmente trop, pour assurer la stabilité.

> Les stablecoins non adossés ont souvent une valeur "fictive", car ils ne sont pas soutenus par des actifs tangibles ou des garanties traditionnelles, mais reposent sur des interventions algorithmiques pour maintenir leur PEG.

---

## **§ Initial Coin Offerings (ICO)**

### Investisseurs

Les mouvements dans les Initial Coin Offerings peuvent être influencés par divers facteurs

**Manipulation par de Grands Investisseurs**:

_"…l'augmentation des prix a créé un enthousiasme supplémentaire pour l'achat et attiré des acheteurs qui ont vu l'augmentation comme une confirmation de thèses d'investissement. Lorsque les investisseurs 'se mettent à jour' pour rejoindre chaque partie, ils créent leur propre vérité—pour un temps."_

Buffett ne parlait pas de Bitcoin – il parlait de l'or en 2012.

> Les grands investisseurs pourraient très bien manipuler le marché avec cette vérité.

**Investisseurs de Mode**:

Ces investisseurs voient l'augmentation des prix et achètent à des niveaux élevés, ce qui contribue partiellement à la poursuite de l'augmentation des prix, puis vendent finalement à des niveaux bas.

> Eh bien, nous sommes ici pour ne pas être eux.

### Critères de Valeur

1. **Première Génération/Or 2.0**: En tant que mécanisme de transaction.
2. **Tokens de Calcul Distribué**: En tant que technologies puissantes.
3. **Tokens d'Utilisation**: En tant qu'extension des actifs techniques.
4. **Tokens de Valeur Mobilière**: En tant que tokenisation de valeurs existantes comme les actions, obligations ou autres actifs.
5. **Tokens Fongibles**: Avec des cas d'utilisation de valeur variable.
6. **Tokens Non-Fongibles**: Eh bien...
7. **Stablecoins**: Plus facile à évaluer en raison de leur nature.

### Tendances ICO

Nous pouvons dire avec certitude que les ICOs ne cesseront pas tant qu'il y aura de nouvelles technologies innovantes dans lesquelles investir.

---

## **§ Crypto Frauduleux**

Je voudrais que cette section soit personnelle et informelle, car elle est très personnelle. J'ai vu des amis devenir victimes de ces plans, qui m'ont ou même ma famille plusieurs fois dans leur viseur. Heureusement, ma compréhension des tactiques de manipulation et du contrôle émotionnel nous a aidés à nous protéger. Je ne peux vraiment pas supporter que les gens exploitent les faiblesses émotionnelles pour exercer du pouvoir.

### **PAS DE SCHÉMAS PONZI**

**Aucun travail ne devrait rester non rémunéré** lorsqu'il génère de l'argent. N'attends pas de rendements pour un travail que tu as effectué gratuitement.

### **AUCUN RENDEMENT IRRÉALISTE**

**Fais tes calculs** avant d'investir du temps et de l'argent. Il est de nature impossible d'avoir des prix premium ou des gains garantis sur les marchés d'échange.

### **AUCUNE RENCONTRE ÉMOTIONNELLE/OFFRES SUR LE HYPE**

**Les émotions peuvent obscurcir la réalité.** Bien que les émotions soient une partie essentielle de l'expérience humaine, elles peuvent troubler le jugement.

_Comme sur une échelle, l'émotion peut conduire à un ruine de la perception de la réalité ; plus tu te diriges vers les émotions, plus ta vue deviendra floue._

Ces rencontres sont souvent des théâtres soigneusement conçus pour te manipuler. Les caractéristiques fréquentes sont :

- **Rencontres tardives** : Tes mécanismes de défense sont plus faibles lorsque tu es fatigué.
- **Langage rapide** : La transmission rapide de grands chiffres et de calculs complexes laisse peu de place à la réflexion critique.
- **Exploitation de l'influence de masse** : Des acteurs peuvent souvent être présents pour créer une atmosphère plus crédible. Ces « acteurs » ou personnes entièrement manipulées tenteront de te diriger vers leurs idéaux.
- **Tentative d'engagement émotionnel** : Ils parlent de faiblesses sociales fréquentes et utilisent des images impressionnantes pour pénétrer ta défense, car elles sont plus rapidement assimilées par un esprit fatigué.

### **AUCUN ÉTRANGER AVEC DE L'ARGENT**

**Recherche des noms et des personnes, la réputation et l'identité** ; souvent, des images et des noms sont falsifiés pour simuler la crédibilité en créant des personnages fictifs que tu ne pourrais jamais mettre en question ou consulter.

### **ALARMES ROUGES**

_**La présence de l'un de ces éléments devrait être considérée comme une alarme rouge. Reste vigilant et remet tout en question.**_

---

## **§ Cadre d'Évaluation**

_**"Nous devons examiner attentivement les Initial Coin Offerings, être préparés, éviter les erreurs pour investir efficacement"**_

Voici une petite application que j'ai créée en langage de programmation Rust (technologie utilisée par Solana) qui t'aide à évaluer les ICOs plus efficacement, avec une feuille d'explication complète.

_(Cela est inspiré d'une liste de vérification examinée par des investisseurs en capital-risque et améliorée par le Dr Harvey R. Campbell)_

### [Application](../../code/evframework/main.exe)

Clique sur le titre

### [Feuille](./evaluation_sheet.md)

Clique sur le titre

_(Il s'agit d'une liste de vérification simplifiée, inspirée d'une version examinée par des investisseurs en capital-risque et améliorée par Harvey Campbell.)_

---

###### Suggestions Complémentaires

[Blockchain Cheat Sheet - Futur de la Fintech](./blockchain-fintech-cheatsheet.md)

---

**Auteur**: Kenneth Boldrini