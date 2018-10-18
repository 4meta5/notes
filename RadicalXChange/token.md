# Token Standards

> "Although Ethereum allows developers to create absolutely any kind of application without restriction to specific feature types, and prides itself on its "lack of features", there is nevertheless a need to standardize certain very common use cases in order to allow users and applications to more easily interact with each other. This includes sending currency units, registering names, making offers on exchanges, and other similar functions. A standard typically consists of a set of function signatures for a few methods, eg. send, register, delete, providing the set of arguments and their formats in the Ethereum contract ABI language." [Vitalik's original post, which motivated ERC 20](https://github.com/ethereum/wiki/wiki/Standardized_Contract_APIs/499c882f3ec123537fc2fccd57eaa29e6032fe4a)

Because token standards formalize how we store data on the blockchain, it is important to integrate these designs into the applications motivated by Radical Markets (and enabled by blockchain). Of course, following token standards is not necessary, but it is better to build on top of this work rather than reinvent the wheel.

**Random Important Links**<br>
* [EIP Website](https://eips.ethereum.org/)
* [Walking Through the ERC721 Full Implementation](https://medium.com/blockchannel/walking-through-the-erc721-full-implementation-72ad72735f3c)
* [OpenZeppelin Solidity Token Contracts](https://github.com/OpenZeppelin/openzeppelin-solidity/tree/master/contracts/token)

**Token Standard Notes**<br>
* [ERC20](#erc20)
* [ERC165](#erc165)
* [ERC721](#erc721)
* [ERC998](#erc998)

## ERC 20: Token Standard Interface <a name="erc20"></a>

## ERC 165: Standard for Detecting Interfaces <a name="erc165"></a>

## ERC 721: Non-Fungible Token Standard <a name="erc721"></a>

## ERC 998: Composable Non-Fungible Token Standard <a name="erc998"></a>