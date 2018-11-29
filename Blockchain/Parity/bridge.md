# Bridge

* Bridges work by having a light client on both sides of the bridge -- the light client interprets whatever state transitions have happened and introduces them as extrinsics (~transactions) into the other side of the bridge. This lets the other side interpret what is going on.
* Security guarantees are analagous to the security guarantees of the light client. The biggest problem with bridges would arise if you were bridging a chain with absolute finality to a chain with less of a finality guarantee...so then you use probabilistic finality as a tacit guarantees