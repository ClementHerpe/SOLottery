# MVP Soloterie

Projet de loterie web3 transparent

## Fonctionnalités

-   Création d'une loterie par un utilisateur 
- -     Formulaire de création
- -     Conversion des infos en un smart contract solana 
- - -       Calcul du nombre de NFT à minter et mint 
- - - -         Backend créer une collection de X NFT Metaplex en gardant la mint autority
- - - -         Backend enregistre l’adresse de la collection dans le smart contract
- - -       Définition du moment de résolution de gagnant
- - -       Définition de la méthode de tirage (chainlink?)
- -     Déploiement du smartcontract sur le réseau
- -     Stockage de l'adresse du smartcontract pour pouvoir afficher les infos sur le site

- Mise en vente des tickets de la loterie crée
- -     L’utilisateur envoie des SOL pour acheter un ticket.
- -     Récupération des informations de la loterie via le smartcontract 
- -     Formulaire d'achat si loterie pas pleine
- - -       Connexion metamask utilisateur 
- - -       Envoi vers adresse smartcontract correspondant
- - -       Vérification loterie toujours en cours et dispo
- - -       Backend ou smart contract (via CPI Metaplex) mint un NFT dans la collection de la loterie.
- - -       Le mint est directement envoyé à l'adresse de l'utilisateur

- Tirage
- -     Tirage au sort du NFT gagnant validation que chaque NFT appartient bien à la collection de la loterie (via Metaplex metadata).
- -     Recherche du wallet possedant le NFT via solscan
- -     Envoi des fonds sur le wallet

### Notes

NFT car plus simple de gérer les échanges de propriété
Mécanisme en cas de loterie incomplète à déterminer (annulation ? loterie partielle ?)
Mécanisme d'envoi des fonds à compléter pour ne pas envoyer vers un wallet mort ?

## Documentation projet :

### Utilisation

- lancer le validateur local avec solana-test-validator
- lancer le projet avec anchor test --skip-local-validator. Cette fontion : 
- - Compile le programme solana
- - Déplois le contrat sur le validateur local
- - Execute les tests renseignés dans le fichier tests

### Avancée

- Initialisation de la loterie OK : projet compilé, déployé et test renvoit "Nombre de tickets : 20" avec les paramètres pool total : 2 SOL / ticket : 0.1 SOL