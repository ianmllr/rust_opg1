
✏️ Refleksion:

#### Hvordan opfører mutable og immutable variabler sig forskelligt?

En immutable variabel kan ikke ændres uden en compile-time error. En mutable variabel kan ændres

#### Hvordan kunne immutabilitet være nyttigt i større programmer?

Det kan give sikkerhed. I Rust foretrækker man immutabilitet, og alle variabler er immutable som default.
Variablen er immutable som udgangspunkt fordi at det forebygger compile-time errors hvis man prøver at ændre
en immutabel variabel. Rust garanterer at når man siger at en value ikke skal ændres, så kan den virkelig ikke ændres.

#### Hvilke ligheder og forskelle fandt I mellem Rust og Java/C#?

Ligheder:
- Variabler som, float, boolean, strings er ens, men med andre "navne". 
- Man har dependencies på samme måde.

Forskelle: Rust er et statically typed sprog hvilket betyder at sproget skal vide typen af alle variabler ved compile-time.
Ofte kan compileren regne datatypen ud ud fra værdien og hvordan den bruges, men ikke altid.

#### Hvordan kan Rusts strenge typekontrol være en fordel?

Det kan give sikkerhed og hjælpe med at undgå bugs som er svære at finde. Også nem refaktorering og god performance.