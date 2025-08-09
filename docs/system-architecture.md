#
```mermaid
architecture-beta
    group internal(cloud)[Internal]
    group application(server)[Application] in internal
    group storage(database)[Storage] in internal

    service gateway(internet)[Gateway]
    service client(client)[Client]

    %% datastore
    service datastore(database)[Data Store] in storage
    service authstore(database)[Auth Store] in storage

    %% application
    service auth(server)[Auth] in application
    service payment(server)[Payment] in application
    


    client:L -- R:gateway
    gateway:L -- R:auth
    auth:T -- B:authstore
    auth:T -- B:datastore

    gateway:L -- R:payment
    payment:T -- B:datastore
