> 3.1 Enregistrement de la connexion (Connection Registration)
>
> Les commandes décrites ici sont utilisées pour enregistrer une connexion avec
> un serveur IRC en tant qu'utilisateur ainsi que pour se déconnecter
> correctement.
>
> Une commande "PASS" n'est pas nécessaire pour qu'une connexion client soit
> enregistrée, mais elle doit précéder la dernière des combinaisons
> `NICK`/`USER` (pour une connexion utilisateur) ou la commande `SERVICE` (pour
> une connexion service). L'ordre recommandé pour l'enregistrement d'un client
> est le suivant :
>
> 1. Passer le message
> 2. Message Nick OU Message de service
> 3. Message de l'utilisateur
>
> En cas de succès, le client recevra un message `RPL_WELCOME` (pour les
> utilisateurs) ou `RPL_YOURESERVICE` (pour les services) indiquant que la
> connexion est maintenant enregistrée et connue de l'ensemble du réseau IRC. Le
> message de réponse doit contenir l'identifiant complet du client sur lequel il
> a été enregistré.
