# Monte Carlo Simulation of the board game Solarquest
Full blog post published at [wyldcard.io/blog/solarquests-broken-board](https://wyldcard.io/blog/solarquests-broken-board)

I mainly wrote this as a way to learn Rust. Reading the book is one thing, but having to put the theories into practice is another.
Looking at how the code turned out, I should have just stuck with storing the board as a collection of spots and then keeping a separate collection of edges, indexed by the `SolarID` enum I'm using. Performance would be much better that way.

But, I wanted to understand _why_ linked lists are tricky in Rust, and I wanted to run into issues with ownership and then learn why I should avoid them >_<

--------
Sample Output:
```
Simulated 40 rounds for 100000 players. It takes about 20 rounds to get around the board once
Miranda,102420
Nereid,89170
SaturnSpaceDock,88712
Mimas,87905
Sinope,84278
Larissa,80290
Thebe,78190
FederationStationII,77152
Callisto,75413
Metis,73831
Elara,71764
Ganymede,71122
Himalia,70542
NeptuneResearchLab,70172
Amalthea,68459
Europa,68254
Adrastea,66739
FederationStationVI,64422
Janus,64059
Ariel,62559
Tethys,62420
Venus,62365
FederationStationIII,61962
FederationStationIV,61346
Enceladus,61226
SaturnResearchLab,59681
Titania,59255
Rhea,58278
Hyperion,58074
FederationStationVII,57304
JupiterSpaceDock,57302
Titan,57302
Galatea,55890
UranusResearchLab,55791
Naiad,54442
FederationStationIX,54266
Iapetus,52455
FederationStationVIII,49356
Io,49134
Pluto,48934
Triton,48838
Portia,47219
EarthResearchLab,45023
Charon,43072
Proteus,42997
Oberon,41921
Earth,41730
Despina,41012
UranusSpaceDock,40967
Umbriel,39840
Dione,38841
Moon,35661
FederationStationV,35267
FederationStationI,33172
SolarSpaceDock,32782
Mercury,31534
VenusResearchLab,31226
Phoebe,30621
Thalassa,29749
Deimos,29267
Mars,28294
NeptuneSpaceDock,27920
Phobos,27000
JupiterResearchLab,0
thirteen was rolled 222062 times
```